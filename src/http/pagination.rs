use crate::core::errors::GitLabError;
use crate::core::constants::DEFAULT_PER_PAGE;

/// Informações de paginação extraídas dos cabeçalhos HTTP da resposta.
///
/// Contém campos opcionais para página atual, total de itens, total de páginas,
/// próxima página, página anterior e cursor da próxima página (para keyset pagination).
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
///
/// Extrai os valores dos cabeçalhos `x-page`, `x-per-page`, `x-total`,
/// `x-total-pages`, `x-next-page`, `x-prev-page` e `x-next-cursor`.
///
/// ## Params
/// - `headers`: Cabeçalhos HTTP da resposta.
///
/// ## Returns
/// `PaginationInfo` — estrutura com os campos preenchidos quando os respectivos
/// cabeçalhos estão presentes.
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
            .map(|s| s.to_string()),
    }
}

/// Extrai mensagens de erro do corpo de uma resposta JSON da API do GitLab.
///
/// Procura pelos campos `message` e `error` no JSON e coleta todas as strings
/// encontradas em uma lista.
///
/// ## Params
/// - `body`: Corpo da resposta como string.
///
/// ## Returns
/// `Option<Vec<String>>` — `Some` com a lista de mensagens se houver,
/// `None` se nenhuma mensagem for encontrada.
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

/// Extrai o valor de `retry_after` do corpo de uma resposta de rate limit (HTTP 429).
///
/// ## Params
/// - `body`: Corpo da resposta como string.
///
/// ## Returns
/// `Option<u64>` — número de segundos a aguardar antes de tentar novamente,
/// ou `None` se o campo não estiver presente.
pub(crate) fn extract_retry_after(body: &str) -> Option<u64> {
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(retry_after) = val.get("retry_after").and_then(|v| v.as_u64()) {
            return Some(retry_after);
        }
    }
    None
}

/// Auto-pagina todas as páginas de um endpoint com paginação baseada em número de página.
///
/// Itera sobre as páginas chamando `fetch_page` repetidamente, incrementando o número
/// da página até que uma página retorne menos itens que `DEFAULT_PER_PAGE` ou
/// não haja mais próxima página (`next_page`).
///
/// ## Params
/// - `fetch_page`: Função que recebe o número da página e retorna os itens e `PaginationInfo`.
/// - `_operation`: Identificador textual da operação (para logging).
///
/// ## Returns
/// `Result<Vec<T>, GitLabError>` — lista consolidada de todos os itens de todas as páginas.
///
/// ## Errors
/// Retorna `GitLabError` se alguma chamada a `fetch_page` falhar.
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

#[allow(dead_code)]
/// Auto-pagina todas as páginas de um endpoint com paginação por cursor (keyset).
///
/// Utiliza o cabeçalho `X-NEXT-CURSOR` para navegar entre as páginas.
/// A iteração termina quando uma página retorna zero itens ou o cursor
/// não se altera entre chamadas consecutivas.
///
/// ## Params
/// - `fetch_page`: Função que recebe um cursor opcional e retorna os itens e `PaginationInfo`.
/// - `_operation`: Identificador textual da operação (para logging).
///
/// ## Returns
/// `Result<Vec<T>, GitLabError>` — lista consolidada de todos os itens de todas as páginas.
///
/// ## Errors
/// Retorna `GitLabError` se alguma chamada a `fetch_page` falhar.
pub(crate) fn keyset_paginate_all<T: serde::de::DeserializeOwned, F>(
    fetch_page: F,
    _operation: &str,
) -> Result<Vec<T>, GitLabError>
where
    F: Fn(Option<&str>) -> Result<(Vec<T>, PaginationInfo), GitLabError>,
{
    let mut all_items = Vec::new();
    let mut cursor: Option<String> = None;

    loop {
        let (items, pagination) = fetch_page(cursor.as_deref())?;
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

    Ok(all_items)
}
