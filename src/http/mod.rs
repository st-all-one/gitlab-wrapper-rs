//! Módulo HTTP — cliente HTTP, paginação e limitador de taxa.
//!
//! Este módulo contém os componentes de transporte HTTP utilizados para
//! comunicar com a API do GitLab. Inclui o cliente HTTP propriamente dito
//! (`client::HttpClient`), estruturas e funções de paginação
//! (`pagination::PaginationInfo`, `paginate_all`, `keyset_paginate_all`),
//! e um limitador de taxa baseado em janela deslizante
//! (`rate_limiter::SlidingWindow`).

pub mod client;
pub mod pagination;
pub mod rate_limiter;
