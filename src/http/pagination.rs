use crate::core::errors::GitLabError;
use crate::core::constants::DEFAULT_PER_PAGE;

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct PaginationInfo {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub total: Option<u64>,
    pub total_pages: Option<u32>,
    pub next_page: Option<u32>,
    pub prev_page: Option<u32>,
}

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
    }
}

pub(crate) fn extract_error_messages(body: &str) -> Option<Vec<String>> {
    let mut messages = Vec::new();
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(msg) = val.get("message") {
            match msg {
                serde_json::Value::String(s) => messages.push(s.clone()),
                serde_json::Value::Object(map) => {
                    for (_key, val) in map {
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
        if let Some(error) = val.get("error") {
            if let Some(s) = error.as_str() {
                messages.push(s.to_string());
            }
        }
    }
    if messages.is_empty() { None } else { Some(messages) }
}

pub(crate) fn extract_retry_after(body: &str) -> Option<u64> {
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(retry_after) = val.get("retry_after").and_then(|v| v.as_u64()) {
            return Some(retry_after);
        }
    }
    None
}

/// Auto-paginate through all pages of a collection endpoint.
pub(crate) fn paginate_all<T: serde::de::DeserializeOwned, F>(
    fetch_page: F,
    _operation: &str,
) -> Result<Vec<T>, GitLabError>
where
    F: Fn(u32) -> Result<(Vec<T>, PaginationInfo), GitLabError>,
{
    let mut all_items = Vec::new();
    let mut page: u32 = 1;

    loop {
        let (items, pagination) = fetch_page(page)?;
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

    Ok(all_items)
}
