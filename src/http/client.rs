use std::sync::Mutex;
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::StatusCode;

use crate::core::config::{AuthMethod, ResolvedConfig};
use crate::core::constants::{DEFAULT_PER_PAGE, API_VERSION, USER_AGENT_VALUE};
use crate::core::errors::{ErrorCategory, ErrorContext, GitLabError};
use crate::http::pagination::{
    extract_error_messages, extract_retry_after, parse_pagination_headers, PaginationInfo,
};
use crate::http::rate_limiter::SlidingWindow;
use crate::utils::encoding::encode_query_param;

#[derive(Debug)]
pub(crate) struct HttpClient {
    client: reqwest::blocking::Client,
    config: ResolvedConfig,
    rate_limiter: Mutex<SlidingWindow>,
}

impl HttpClient {
    pub fn new(config: ResolvedConfig) -> Self {
        let rate_limiter = SlidingWindow::new(config.max_rps, Duration::from_secs(1));
        let client = reqwest::blocking::Client::builder()
            .timeout(config.timeout)
            .user_agent(USER_AGENT_VALUE)
            .build()
            .expect("Failed to build HTTP client");
        Self {
            client,
            config,
            rate_limiter: Mutex::new(rate_limiter),
        }
    }

    pub(crate) fn build_url(&self, path: &str, query: &[(String, String)]) -> Result<String, GitLabError> {
        let base = self.config.base_url.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        let mut url = format!("{}/api/{}/{}", base, API_VERSION, path);

        if !query.is_empty() {
            let mut first = true;
            for (key, val) in query {
                if val.is_empty() {
                    continue;
                }
                if first {
                    url.push('?');
                    first = false;
                } else {
                    url.push('&');
                }
                url.push_str(&encode_query_param(key));
                url.push('=');
                url.push_str(&encode_query_param(val));
            }
        }

        Ok(url)
    }

    fn build_headers(&self, extra: Option<&HeaderMap>) -> Result<HeaderMap, GitLabError> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        if let Some(ref token) = self.config.token {
            match self.config.auth_method {
                AuthMethod::Header => {
                    headers.insert(
                        "PRIVATE-TOKEN",
                        HeaderValue::from_str(token)
                            .map_err(|e| GitLabError::Config(format!("Invalid token: {e}")))?,
                    );
                }
                AuthMethod::Bearer => {
                    let value = format!("Bearer {}", token);
                    headers.insert(
                        AUTHORIZATION,
                        HeaderValue::from_str(&value)
                            .map_err(|e| GitLabError::Config(format!("Invalid bearer token: {e}")))?,
                    );
                }
            }
        }

        if let Some(ref sudo) = self.config.sudo {
            headers.insert(
                "SUDO",
                HeaderValue::from_str(sudo)
                    .map_err(|e| GitLabError::Config(format!("Invalid sudo header: {e}")))?,
            );
        }

        if let Some(extra) = extra {
            headers.extend(extra.iter().map(|(k, v)| (k.clone(), v.clone())));
        }

        Ok(headers)
    }

    fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::blocking::Response,
        operation: &str,
    ) -> Result<T, GitLabError> {
        let status = response.status();
        let http_status = status.as_u16();

        if status.is_success() || status == StatusCode::NOT_MODIFIED {
            if http_status == 204 {
                return serde_json::from_value(serde_json::Value::Null)
                    .map_err(GitLabError::from);
            }
            return response.json::<T>().map_err(|e| {
                log::error!(target: "gitlab_wrapper::http", "{} failed to parse JSON: {}", operation, e);
                GitLabError::Api {
                    category: ErrorCategory::ParseError,
                    status: http_status,
                    detail: format!("Failed to parse response: {}", e),
                    instance: String::new(),
                    context: Box::new(ErrorContext {
                        operation: Some(operation.to_string()),
                        http_status: Some(http_status),
                        ..Default::default()
                    }),
                }
            });
        }

        let body = response.text().unwrap_or_default();
        let api_errors = extract_error_messages(&body);
        let category = ErrorCategory::from_status(http_status).unwrap_or(ErrorCategory::InternalError);

        log::error!(target: "gitlab_wrapper::http", "{} failed: status={}, category={}", operation, http_status, category);

        if http_status == 429 {
            let retry_after = extract_retry_after(&body);
            return Err(GitLabError::RateLimited {
                retry_after,
                context: Box::new(ErrorContext {
                    operation: Some(operation.to_string()),
                    http_status: Some(http_status),
                    response_body: Some(body.clone()),
                    api_errors,
                }),
            });
        }

        Err(GitLabError::api(category, http_status, body.clone(), ErrorContext {
            operation: Some(operation.to_string()),
            http_status: Some(http_status),
            response_body: Some(body),
            api_errors,
        }))
    }

    pub fn get<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        query: &[(String, String)],
        operation: &str,
    ) -> Result<T, GitLabError> {
        self.rate_limiter.lock().unwrap_or_else(|e| e.into_inner()).acquire();
        let url = self.build_url(path, query)?;
        let headers = self.build_headers(None)?;

        log::debug!(target: "gitlab_wrapper::http", "GET {} - {}", operation, path);

        let resp = self.client.get(&url).headers(headers).send().map_err(|e| {
            log::error!(target: "gitlab_wrapper::http", "GET {} failed: {}", operation, e);
            GitLabError::from(e)
        })?;

        self.handle_response(resp, operation)
    }

    pub fn get_with_headers<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        query: &[(String, String)],
        operation: &str,
    ) -> Result<(T, PaginationInfo), GitLabError> {
        self.rate_limiter.lock().unwrap_or_else(|e| e.into_inner()).acquire();
        let url = self.build_url(path, query)?;
        let headers = self.build_headers(None)?;

        log::debug!(target: "gitlab_wrapper::http", "GET {} (with headers) - {}", operation, path);

        let resp = self.client.get(&url).headers(headers).send().map_err(|e| {
            log::error!(target: "gitlab_wrapper::http", "GET {} failed: {}", operation, e);
            GitLabError::from(e)
        })?;

        let pagination = parse_pagination_headers(resp.headers());
        let status = resp.status();
        let http_status = status.as_u16();

        if status.is_success() {
            let data: T = resp.json().map_err(|e| {
                log::error!(target: "gitlab_wrapper::http", "GET {} failed to parse JSON: {}", operation, e);
                GitLabError::Api {
                    category: ErrorCategory::ParseError,
                    status: http_status,
                    detail: format!("Failed to parse response: {}", e),
                    instance: String::new(),
                    context: Box::new(ErrorContext {
                        operation: Some(operation.to_string()),
                        http_status: Some(http_status),
                        ..Default::default()
                    }),
                }
            })?;
            return Ok((data, pagination));
        }

        let body = resp.text().unwrap_or_default();
        let api_errors = extract_error_messages(&body);
        let category = ErrorCategory::from_status(http_status).unwrap_or(ErrorCategory::InternalError);

        log::error!(target: "gitlab_wrapper::http", "{} failed: status={}, category={}", operation, http_status, category);

        Err(GitLabError::api(category, http_status, body.clone(), ErrorContext {
            operation: Some(operation.to_string()),
            http_status: Some(http_status),
            response_body: Some(body),
            api_errors,
        }))
    }

    pub fn post<T: serde::de::DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
        operation: &str,
    ) -> Result<T, GitLabError> {
        self.rate_limiter.lock().unwrap_or_else(|e| e.into_inner()).acquire();
        let url = self.build_url(path, &[])?;
        let headers = self.build_headers(None)?;

        log::debug!(target: "gitlab_wrapper::http", "POST {} - {}", operation, path);

        let resp = self.client.post(&url).headers(headers).json(body).send().map_err(|e| {
            log::error!(target: "gitlab_wrapper::http", "POST {} failed: {}", operation, e);
            GitLabError::from(e)
        })?;

        self.handle_response(resp, operation)
    }

    pub fn put<T: serde::de::DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
        operation: &str,
    ) -> Result<T, GitLabError> {
        self.rate_limiter.lock().unwrap_or_else(|e| e.into_inner()).acquire();
        let url = self.build_url(path, &[])?;
        let headers = self.build_headers(None)?;

        log::debug!(target: "gitlab_wrapper::http", "PUT {} - {}", operation, path);

        let resp = self.client.put(&url).headers(headers).json(body).send().map_err(|e| {
            log::error!(target: "gitlab_wrapper::http", "PUT {} failed: {}", operation, e);
            GitLabError::from(e)
        })?;

        self.handle_response(resp, operation)
    }

    pub fn delete(
        &self,
        path: &str,
        query: &[(String, String)],
        operation: &str,
    ) -> Result<(), GitLabError> {
        self.rate_limiter.lock().unwrap_or_else(|e| e.into_inner()).acquire();
        let url = self.build_url(path, query)?;
        let headers = self.build_headers(None)?;

        log::debug!(target: "gitlab_wrapper::http", "DELETE {} - {}", operation, path);

        let resp = self.client.delete(&url).headers(headers).send().map_err(|e| {
            log::error!(target: "gitlab_wrapper::http", "DELETE {} failed: {}", operation, e);
            GitLabError::from(e)
        })?;

        let status = resp.status();
        if status.is_success() || status == StatusCode::NO_CONTENT {
            return Ok(());
        }

        let body = resp.text().unwrap_or_default();
        let http_status = status.as_u16();
        let category = ErrorCategory::from_status(http_status).unwrap_or(ErrorCategory::InternalError);

        Err(GitLabError::api(
            category,
            http_status,
            body.clone(),
            ErrorContext {
                operation: Some(operation.to_string()),
                http_status: Some(http_status),
                response_body: Some(body),
                ..Default::default()
            },
        ))
    }

    pub fn delete_with_body<T: serde::de::DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
        operation: &str,
    ) -> Result<T, GitLabError> {
        self.rate_limiter.lock().unwrap_or_else(|e| e.into_inner()).acquire();
        let url = self.build_url(path, &[])?;
        let headers = self.build_headers(None)?;

        log::debug!(target: "gitlab_wrapper::http", "DELETE {} (with body) - {}", operation, path);

        let resp = self.client.delete(&url).headers(headers).json(body).send().map_err(|e| {
            log::error!(target: "gitlab_wrapper::http", "DELETE {} failed: {}", operation, e);
            GitLabError::from(e)
        })?;

        self.handle_response(resp, operation)
    }

    pub fn get_raw(
        &self,
        path: &str,
        query: &[(String, String)],
        operation: &str,
    ) -> Result<Vec<u8>, GitLabError> {
        self.rate_limiter.lock().unwrap_or_else(|e| e.into_inner()).acquire();
        let url = self.build_url(path, query)?;
        let headers = self.build_headers(None)?;

        log::debug!(target: "gitlab_wrapper::http", "GET raw {} - {}", operation, path);

        let resp = self.client.get(&url).headers(headers).send().map_err(|e| {
            log::error!(target: "gitlab_wrapper::http", "GET raw {} failed: {}", operation, e);
            GitLabError::from(e)
        })?;

        let status = resp.status();
        let http_status = status.as_u16();

        if status.is_success() {
            return resp.bytes().map(|b| b.to_vec()).map_err(|e| {
                GitLabError::Api {
                    category: ErrorCategory::ParseError,
                    status: http_status,
                    detail: format!("Failed to read response body: {}", e),
                    instance: String::new(),
                    context: Box::new(ErrorContext {
                        operation: Some(operation.to_string()),
                        http_status: Some(http_status),
                        ..Default::default()
                    }),
                }
            });
        }

        let body = resp.text().unwrap_or_default();
        let category = ErrorCategory::from_status(http_status).unwrap_or(ErrorCategory::InternalError);
        Err(GitLabError::api(
            category,
            http_status,
            body.clone(),
            ErrorContext {
                operation: Some(operation.to_string()),
                http_status: Some(http_status),
                response_body: Some(body),
                ..Default::default()
            },
        ))
    }

    pub fn get_raw_text(
        &self,
        path: &str,
        query: &[(String, String)],
        operation: &str,
    ) -> Result<String, GitLabError> {
        self.rate_limiter.lock().unwrap_or_else(|e| e.into_inner()).acquire();
        let url = self.build_url(path, query)?;
        let headers = self.build_headers(None)?;

        log::debug!(target: "gitlab_wrapper::http", "GET raw text {} - {}", operation, path);

        let resp = self.client.get(&url).headers(headers).send().map_err(|e| {
            log::error!(target: "gitlab_wrapper::http", "GET raw text {} failed: {}", operation, e);
            GitLabError::from(e)
        })?;

        let status = resp.status();
        let http_status = status.as_u16();

        if status.is_success() {
            return resp.text().map_err(|e| {
                GitLabError::Api {
                    category: ErrorCategory::ParseError,
                    status: http_status,
                    detail: format!("Failed to read response body: {}", e),
                    instance: String::new(),
                    context: Box::new(ErrorContext {
                        operation: Some(operation.to_string()),
                        http_status: Some(http_status),
                        ..Default::default()
                    }),
                }
            });
        }

        let body = resp.text().unwrap_or_default();
        let category = ErrorCategory::from_status(http_status).unwrap_or(ErrorCategory::InternalError);
        Err(GitLabError::api(
            category,
            http_status,
            body.clone(),
            ErrorContext {
                operation: Some(operation.to_string()),
                http_status: Some(http_status),
                response_body: Some(body),
                ..Default::default()
            },
        ))
    }

    pub fn paginate_all<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        query: &[(String, String)],
        operation: &str,
    ) -> Result<Vec<T>, GitLabError> {
        use crate::http::pagination::paginate_all as auto_paginate;
        auto_paginate(
            |page: u32| {
                let mut paged_query = query.to_vec();
                paged_query.push(("per_page".to_string(), DEFAULT_PER_PAGE.to_string()));
                paged_query.push(("page".to_string(), page.to_string()));
                self.get_with_headers(path, &paged_query, operation)
            },
            operation,
        )
    }
}
