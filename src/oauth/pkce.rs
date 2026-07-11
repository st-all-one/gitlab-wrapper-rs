//! Utilitários PKCE (Proof Key for Code Exchange) para o fluxo de código de autorização OAuth.
//!
//! Gera um verificador aleatório e seu respectivo desafio (SHA-256 codificado em base64url)
//! de acordo com a especificação RFC 7636.

use sha2::{Digest, Sha256};

const VERIFIER_LENGTH: usize = 32;

/// Gera um verificador PKCE aleatório.
///
/// O verificador é uma sequência de 32 bytes aleatórios, codificada em base64url
/// sem padding, conforme a RFC 7636.
///
/// ## Returns
/// `String` — o verificador PKCE codificado em base64url.
///
/// ## Panics
/// Pode entrar em pânico se a fonte de aleatoriedade do sistema falhar (CSPRNG).
pub fn generate_code_verifier() -> String {
    let mut bytes = vec![0u8; VERIFIER_LENGTH];
    getrandom::getrandom(&mut bytes).expect("CSPRNG failure");
    base64url_encode(&bytes)
}

/// Gera o desafio PKCE (`code_challenge`) a partir de um verificador.
///
/// Aplica SHA-256 ao verificador fornecido e codifica o hash em base64url
/// sem padding, conforme a RFC 7636.
///
/// ## Params
/// - `verifier`: O verificador PKCE gerado por [`generate_code_verifier`].
///
/// ## Returns
/// `String` — o desafio PKCE codificado em base64url.
pub fn generate_code_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let hash = hasher.finalize();
    base64url_encode(&hash)
}

/// Codifica um slice de bytes em base64url sem padding.
///
/// Converte os bytes para base64 usando a implementação interna [`base64_simple`]
/// e substitui os caracteres `+` e `/` por `-` e `_` respectivamente, removendo
/// o padding `=` no final, conforme o formato base64url (RFC 4648 §5).
///
/// ## Params
/// - `input`: Slice de bytes a ser codificado.
///
/// ## Returns
/// `String` — representação base64url sem padding.
fn base64url_encode(input: &[u8]) -> String {
    let b64 = base64_simple(input);
    b64.replace('+', "-")
        .replace('/', "_")
        .trim_end_matches('=')
        .to_string()
}

/// Implementação minimalista de codificação base64.
///
/// Processa a entrada em blocos de 3 bytes, produzindo 4 caracteres do alfabeto
/// base64 padrão (`A–Z`, `a–z`, `0–9`, `+`, `/`). Blocos parciais recebem
/// padding com `=`.
///
/// ## Params
/// - `input`: Slice de bytes a ser codificado.
///
/// ## Returns
/// `String` — representação base64 com padding.
fn base64_simple(input: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();

    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let triple = (b0 << 16) | (b1 << 8) | b2;

        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);

        if chunk.len() >= 2 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }

        if chunk.len() >= 3 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }

    result
}
