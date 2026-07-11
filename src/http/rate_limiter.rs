use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Limitador de taxa baseado no algoritmo de janela deslizante (*sliding window*).
///
/// Mantém um `VecDeque` de timestamps das requisições realizadas dentro
/// da janela de tempo configurada. Quando o número máximo de requisições
/// é atingido, a thread bloqueia (`sleep`) até que haja espaço na janela.
///
/// O algoritmo garante que não mais que `max_requests` requisições sejam
/// feitas em qualquer intervalo de duração `window`.
#[derive(Debug)]
pub(crate) struct SlidingWindow {
    max_requests: u32,
    window: Duration,
    timestamps: VecDeque<Instant>,
}

impl SlidingWindow {
    /// Cria uma nova instância de `SlidingWindow`.
    ///
    /// ## Params
    /// - `max_requests`: Número máximo de requisições permitidas na janela.
    /// - `window`: Duração da janela de tempo (ex.: 1 segundo).
    ///
    /// ## Returns
    /// `SlidingWindow` — nova instância com o `VecDeque` pré-alocado.
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            max_requests,
            window,
            timestamps: VecDeque::with_capacity(max_requests as usize),
        }
    }

    /// Adquire uma permissão para realizar uma requisição.
    ///
    /// Remove timestamps expirados (fora da janela) e, se o limite foi
    /// atingido, bloqueia a thread até que um slot seja liberado.
    /// Ao final, registra o timestamp atual.
    pub fn acquire(&mut self) {
        let now = Instant::now();
        let cutoff = now - self.window;

        while let Some(&ts) = self.timestamps.front() {
            if ts < cutoff {
                self.timestamps.pop_front();
            } else {
                break;
            }
        }

        if self.timestamps.len() >= self.max_requests as usize {
            if let Some(&oldest) = self.timestamps.front() {
                let wait = self.window.saturating_sub(now - oldest);
                if !wait.is_zero() {
                    log::debug!(target: "gitlab_wrapper::rate_limiter", "Rate limit reached, waiting {wait:?}");
                    std::thread::sleep(wait);
                    let now = Instant::now();
                    let cutoff = now - self.window;
                    while let Some(&ts) = self.timestamps.front() {
                        if ts < cutoff {
                            self.timestamps.pop_front();
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        self.timestamps.push_back(Instant::now());
    }
}
