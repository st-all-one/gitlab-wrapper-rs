use crate::core::constants::DEFAULT_PER_PAGE;
use crate::core::errors::GitLabError;
use crate::core::telemetry::{new_operation_span, record_outcome};
use std::future::Future;
use std::pin::Pin;

/// Alvo de tracing usado pelos spans e eventos de paginação.
const HTTP_TARGET: &str = "gitlab_wrapper::http";

/// Informações de paginação extraídas dos cabeçalhos HTTP da resposta.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct PaginationInfo {
    /// Número da página atual.
    pub page: Option<u32>,
    /// Número de itens por página.
    pub per_page: Option<u32>,
    /// Total de itens disponíveis.
    pub total: Option<u64>,
    /// Total de páginas disponíveis.
    pub total_pages: Option<u32>,
    /// Número da próxima página (se houver).
    pub next_page: Option<u32>,
    /// Número da página anterior (se houver).
    pub prev_page: Option<u32>,
    /// Cursor para a próxima página em keyset pagination.
    pub next_cursor: Option<String>,
}

/// Parseia os cabeçalhos de paginação retornados pela API do GitLab.
#[tracing::instrument(target = HTTP_TARGET, skip(headers), fields(operation = "http.parse_pagination_headers"))]
pub(crate) fn parse_pagination_headers(headers: &reqwest::header::HeaderMap) -> PaginationInfo {
    PaginationInfo {
        page: headers
            .get("x-page")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok()),
        per_page: headers
            .get("x-per-page")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok()),
        total: headers
            .get("x-total")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok()),
        total_pages: headers
            .get("x-total-pages")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok()),
        next_page: headers
            .get("x-next-page")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok()),
        prev_page: headers
            .get("x-prev-page")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok()),
        next_cursor: headers
            .get("x-next-cursor")
            .and_then(|v| v.to_str().ok())
            .map(ToString::to_string),
    }
}

/// Extrai mensagens de erro do corpo de uma resposta JSON da API do GitLab.
#[tracing::instrument(target = HTTP_TARGET, skip(body), fields(operation = "http.extract_error_messages"))]
pub(crate) fn extract_error_messages(body: &str) -> Option<Vec<String>> {
    let mut messages = Vec::new();
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(msg) = val.get("message") {
            match msg {
                serde_json::Value::String(s) => messages.push(s.clone()),
                serde_json::Value::Object(map) => {
                    // Ordena as chaves para manter a ordem das mensagens determinística.
                    let mut entries: Vec<_> = map.iter().collect();
                    entries.sort_by_key(|(a, _)| *a);
                    for (_key, val) in entries {
                        match val {
                            serde_json::Value::String(s) => messages.push(s.clone()),
                            serde_json::Value::Array(arr) => {
                                for v in arr {
                                    if let Some(s) = v.as_str() {
                                        messages.push(s.to_string());
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                serde_json::Value::Array(arr) => {
                    for v in arr {
                        if let Some(s) = v.as_str() {
                            messages.push(s.to_string());
                        }
                    }
                }
                _ => {}
            }
        }
        if let Some(error) = val.get("error")
            && let Some(s) = error.as_str()
        {
            messages.push(s.to_string());
        }
    }
    if messages.is_empty() {
        None
    } else {
        Some(messages)
    }
}

/// Extrai o valor de `retry_after` do corpo de uma resposta de rate limit (HTTP 429).
#[tracing::instrument(target = HTTP_TARGET, skip(body), fields(operation = "http.extract_retry_after"))]
pub(crate) fn extract_retry_after(body: &str) -> Option<u64> {
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(body)
        && let Some(retry_after) = val.get("retry_after").and_then(serde_json::Value::as_u64)
    {
        return Some(retry_after);
    }
    None
}

/// Auto-pagina todas as páginas de um endpoint com paginação baseada em número de página.
///
/// Itera sobre as páginas chamando `fetch_page` repetidamente até que uma página
/// retorne menos itens que `DEFAULT_PER_PAGE` ou não haja mais próxima página.
#[tracing::instrument(target = HTTP_TARGET, skip(fetch_page), fields(operation = %operation))]
pub(crate) async fn paginate_all<T, F>(
    fetch_page: F,
    operation: &str,
) -> Result<Vec<T>, GitLabError>
where
    T: serde::de::DeserializeOwned + 'static,
    F: Fn(
        u32,
    )
        -> Pin<Box<dyn Future<Output = Result<(Vec<T>, PaginationInfo), GitLabError>> + Send>>,
{
    let span = new_operation_span(operation);
    let mut all_items = Vec::new();
    let mut page: u32 = 1;

    loop {
        let (items, pagination) = fetch_page(page).await?;
        let count = items.len();
        all_items.extend(items);

        if count < DEFAULT_PER_PAGE as usize {
            break;
        }

        match pagination.next_page {
            Some(np) => page = np,
            None => break,
        }
    }

    record_outcome(&span, true, "success", Some(all_items.len() as u64));
    Ok(all_items)
}

/// Auto-pagina todas as páginas de um endpoint com paginação por cursor (keyset).
#[expect(dead_code, reason = "reserved for future keyset pagination")]
#[tracing::instrument(target = HTTP_TARGET, skip(fetch_page), fields(operation = %operation))]
pub(crate) async fn keyset_paginate_all<T, F>(
    fetch_page: F,
    operation: &str,
) -> Result<Vec<T>, GitLabError>
where
    T: serde::de::DeserializeOwned + 'static,
    F: Fn(
        Option<String>,
    )
        -> Pin<Box<dyn Future<Output = Result<(Vec<T>, PaginationInfo), GitLabError>> + Send>>,
{
    let span = new_operation_span(operation);
    let mut all_items = Vec::new();
    let mut cursor: Option<String> = None;

    loop {
        let (items, pagination) = fetch_page(cursor.clone()).await?;
        let count = items.len();
        all_items.extend(items);

        if count == 0 {
            break;
        }

        match pagination.next_cursor {
            Some(next) => {
                if Some(&next) == cursor.as_ref() {
                    break;
                }
                cursor = Some(next);
            }
            None => break,
        }
    }

    record_outcome(&span, true, "success", Some(all_items.len() as u64));
    Ok(all_items)
}
