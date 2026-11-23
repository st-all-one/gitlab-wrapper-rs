//! Instrumentação e geração de IDs de correlação para operações do cliente.
//!
//! Este módulo fornece um gerador de `instance` (UUID v7 em produção, sequência
//! determinística em testes), helpers de span com campos estáveis e helpers para
//! eventos estruturados de resultado. Nenhum desses helpers registra tokens,
//! corpos de requisição ou resposta.

use std::sync::atomic::{AtomicU64, Ordering};

use tracing::{Span, event, info_span};
use uuid::Uuid;

/// Alvo de tracing usado por todos os spans e eventos deste módulo.
pub const TELEMETRY_TARGET: &str = "gitlab_wrapper::telemetry";

// IDs determinísticos: versão 7 no nibble de versão, variante RFC 4122 e contador nos bits baixos.
const DETERMINISTIC_ID_START: u128 = 0x0000_0000_0000_7000_8000_0000_0000_0000;
const DETERMINISTIC_ID_MASK: u128 = 0x0000_ffff_ffff_ffff_3fff_ffff_ffff_ffff;

/// Contador usado quando o gerador determinístico é ativado.
static DETERMINISTIC_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Gerador de IDs de correlação (`instance`) para spans e erros.
///
/// Por padrão gera UUID v7 com timestamp atual. Com [`ErrorGenerator::deterministic`],
/// gera `Uuid::from_u128` a partir de uma sequência estável e reproduzível, sem
/// variáveis de ambiente globais. O contador é atômico e não mantém guardas
/// através de `await`.
#[derive(Debug, Clone, Copy, Default)]
pub struct ErrorGenerator {
    deterministic: bool,
}

impl ErrorGenerator {
    /// Cria um gerador padrão, que produz UUID v7 baseados em timestamp.
    pub(crate) const fn new() -> Self {
        Self {
            deterministic: false,
        }
    }

    /// Os IDs retornados são `Uuid::from_u128` de uma sequência iniciada em
    /// `0x00000000_00007000_80000000_00000000` (versão 7, variante RFC 4122),
    /// com os 62 bits baixos usados como contador incrementado atomicamente a
    /// cada chamada. A sequência é global ao processo, portanto testes paralelos
    /// não devem assumir valores absolutos — apenas unicidade, versão e formato.
    #[cfg(test)]
    pub(crate) const fn deterministic() -> Self {
        Self {
            deterministic: true,
        }
    }

    /// Gera o próximo ID de correlação em formato de string.
    pub(crate) fn next_instance(self) -> String {
        self.next_uuid().to_string()
    }

    /// Gera o próximo ID de correlação como [`Uuid`].
    pub(crate) fn next_uuid(self) -> Uuid {
        if self.deterministic {
            let offset = DETERMINISTIC_COUNTER.fetch_add(1, Ordering::Relaxed);
            let bits = DETERMINISTIC_ID_START | (u128::from(offset) & DETERMINISTIC_ID_MASK);
            Uuid::from_u128(bits)
        } else {
            Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext))
        }
    }
}

/// Gera um ID de correlação UUID v7 para uso em instâncias de erro.
pub fn error_instance() -> String {
    ErrorGenerator::new().next_instance()
}

/// Cria um span de nível `INFO` para uma operação, com `operation` e `instance`.
///
/// O `instance` é gerado no momento da criação do span, permitindo correlacionar
/// o span com o erro correspondente. Nenhum dado sensível é incluído.
pub fn new_operation_span(operation: &str) -> Span {
    new_operation_span_with(operation, ErrorGenerator::new())
}

/// Cria um span de nível `INFO` usando um gerador de IDs explícito (útil em testes).
pub fn new_operation_span_with(operation: &str, generator: ErrorGenerator) -> Span {
    info_span!(
        target: TELEMETRY_TARGET,
        "gitlab_wrapper.operation",
        operation = operation,
        instance = generator.next_instance().as_str(),
    )
}

/// Emite um evento estruturado de sucesso associado ao span informado.
///
/// `ok` indica se a operação concluiu com sucesso, `category` é a categoria
/// semântica (por exemplo `error.description()` ou `"success"`), e `count`
/// é um contador opcional (número de itens processados, páginas etc.).
pub fn record_outcome(span: &Span, ok: bool, category: &str, count: Option<u64>) {
    event!(
        target: TELEMETRY_TARGET,
        parent: span,
        tracing::Level::INFO,
        ok = ok,
        category = category,
        count = count,
        "operation.outcome"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_generator_produces_distinct_valid_uuids() {
        let generator = ErrorGenerator::deterministic();
        let first = generator.next_uuid();
        let second = generator.next_uuid();
        assert_ne!(first, second);
        assert_eq!(first.get_version_num(), 7);
        assert_eq!(first.get_variant(), uuid::Variant::RFC4122);
    }

    #[test]
    fn production_generator_produces_v7() {
        let instance = error_instance();
        let parsed = Uuid::parse_str(&instance);
        assert!(parsed.is_ok_and(|uuid| uuid.get_version_num() == 7));
    }

    #[test]
    fn span_and_outcome_are_emitted() {
        let span = new_operation_span_with("test.operation", ErrorGenerator::deterministic());
        record_outcome(&span, true, "success", Some(1));
    }
}
