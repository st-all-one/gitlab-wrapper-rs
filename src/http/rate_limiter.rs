//! Limitador de taxa assíncrono baseado em `tokio::sync::Semaphore`.
//!
//! Cada permissão adquirida é mantida por 1 segundo antes de ser devolvida
//! ao semáforo, garantindo que no máximo `max_rps` requisições sejam
//! disparadas por segundo.

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;

/// Limitador de taxa baseado em semáforo com liberação atrasada.
///
/// Quando `acquire()` é chamado, uma permissão é removida do semáforo
/// e automaticamente devolvida após 1 segundo. Isso cria um sliding window
/// natural: o número de requisições em qualquer janela de 1 segundo nunca
/// excede `max_permits`.
#[derive(Debug, Clone)]
pub(crate) struct RateLimiter {
    semaphore: Arc<Semaphore>,
}

impl RateLimiter {
    /// Cria um novo limitador de taxa.
    ///
    /// ## Params
    /// - `max_rps`: Máximo de requisições por segundo.
    pub fn new(max_rps: u32) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_rps as usize)),
        }
    }

    /// Adquire uma permissão para realizar uma requisição.
    ///
    /// Bloqueia assincronamente até que uma permissão esteja disponível.
    /// A permissão é automaticamente devolvida após 1 segundo via task
    /// background.
    ///
    /// Se o semáforo estiver fechado, registra um aviso e retorna sem aplicar
    /// throttling: não existe caminho de pânico em biblioteca. O fechamento só
    /// ocorreria se um detentor do `Arc` chamasse `Semaphore::close`, o que este
    /// módulo nunca faz; o retorno antecipado é apenas defensivo.
    #[tracing::instrument(
        target = "gitlab_wrapper::http",
        skip(self),
        fields(operation = "http.rate_limiter.acquire")
    )]
    pub async fn acquire(&self) {
        let Ok(permit) = Arc::clone(&self.semaphore).acquire_owned().await else {
            tracing::warn!(
                target: "gitlab_wrapper::http",
                "rate limiter semaphore closed; proceeding without throttling"
            );
            return;
        };
        // Devolve a permissão após 1 segundo, liberando espaço na janela
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            drop(permit);
        });
    }
}
