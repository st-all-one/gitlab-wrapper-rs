//! Fluxo de código de autorização OAuth 2.0 para GitLab.
//!
//! Permite gerar URLs de autorização e trocar o código de autorização
//! por um token de acesso, com suporte opcional a PKCE (RFC 7636).

use crate::core::errors::{ErrorCategory, ErrorContext, GitLabError};
use crate::types::OAuthTokenResponse;
use crate::utils::encoding::encode_query_param;
use std::sync::LazyLock;
use reqwest::blocking::Client;

// Cliente HTTP dedicado — NÃO passa pelo rate limiter do `HttpClient`.
// Isso é intencional: fluxos OAuth são chamadas esporádicas (não em loops).
static OAUTH_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::new()
});

/// Opções para gerar a URL de autorização OAuth.
pub struct AuthCodeUrlOptions {
    /// URL base da instância do GitLab (ex.: `https://gitlab.com`).
    pub base_url: String,
    /// ID do cliente OAuth registrado.
    pub client_id: String,
    /// URI de redirecionamento registrada no cliente OAuth.
    pub redirect_uri: String,
    /// Escopos solicitados (separados por espaço).
    pub scope: String,
    /// Valor de estado para proteção contra CSRF.
    pub state: String,
    /// Desafio PKCE (`code_challenge`) opcional para o fluxo com PKCE.
    pub code_challenge: Option<String>,
}

/// Gera a URL de autorização OAuth para o fluxo de código de autorização.
///
/// Monta a URL no formato `<base>/oauth/authorize` com os parâmetros
/// `client_id`, `redirect_uri`, `response_type=code`, `scope` e `state`.
/// Se `code_challenge` estiver presente, adiciona os parâmetros
/// `code_challenge` e `code_challenge_method=S256`.
///
/// ## Params
/// - `options`: Opções de configuração para a URL de autorização.
///
/// ## Returns
/// `String` — URL de autorização totalmente qualificada.
pub fn authorization_code_url(options: &AuthCodeUrlOptions) -> String {
    let base = options.base_url.trim_end_matches('/');
    let mut url = format!(
        "{}/oauth/authorize?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
        base,
        encode_query_param(&options.client_id),
        encode_query_param(&options.redirect_uri),
        encode_query_param(&options.scope),
        encode_query_param(&options.state),
    );

    if let Some(ref challenge) = options.code_challenge {
        url.push_str(&format!("&code_challenge={}", encode_query_param(challenge)));
        url.push_str("&code_challenge_method=S256");
    }

    url
}

/// Opções para trocar o código de autorização por um token de acesso.
pub struct ExchangeCodeOptions {
    /// URL base da instância do GitLab (ex.: `https://gitlab.com`).
    pub base_url: String,
    /// ID do cliente OAuth registrado.
    pub client_id: String,
    /// Segredo do cliente OAuth (opcional, usado em fluxos confidenciais).
    pub client_secret: Option<String>,
    /// Código de autorização recebido no redirecionamento.
    pub code: String,
    /// URI de redirecionamento (deve coincidir com o usado na URL de autorização).
    pub redirect_uri: String,
    /// Verificador PKCE opcional para o fluxo com PKCE.
    pub code_verifier: Option<String>,
}

/// Troca um código de autorização por um token de acesso OAuth.
///
/// Envia uma requisição POST para `<base>/oauth/token` com os parâmetros
/// `client_id`, `code`, `redirect_uri` e `grant_type=authorization_code`.
/// Opcionalmente inclui `client_secret` e `code_verifier`.
///
/// ## Params
/// - `options`: Opções de configuração para a troca do código.
///
/// ## Returns
/// `Result<OAuthTokenResponse, GitLabError>` — resposta contendo o token de acesso
/// e demais informações em caso de sucesso.
///
/// ## Errors
/// Retorna `GitLabError` em caso de falha de rede, erro de parse da resposta
/// ou erro de autenticação retornado pelo GitLab.
pub fn exchange_authorization_code(
    options: &ExchangeCodeOptions,
) -> Result<OAuthTokenResponse, GitLabError> {
    let base = options.base_url.trim_end_matches('/');
    let url = format!("{}/oauth/token", base);

    let mut form = vec![
        ("client_id".to_string(), options.client_id.clone()),
        ("code".to_string(), options.code.clone()),
        ("redirect_uri".to_string(), options.redirect_uri.clone()),
        ("grant_type".to_string(), "authorization_code".to_string()),
    ];

    if let Some(ref secret) = options.client_secret {
        form.push(("client_secret".to_string(), secret.clone()));
    }

    if let Some(ref verifier) = options.code_verifier {
        form.push(("code_verifier".to_string(), verifier.clone()));
    }

    let resp = OAUTH_CLIENT.post(&url).form(&form).send().map_err(|e| {
        GitLabError::api(
            ErrorCategory::NetworkError,
            503,
            format!("OAuth request failed: {e}"),
            ErrorContext {
                operation: Some("oauth.exchange_authorization_code".into()),
                ..Default::default()
            },
        )
    })?;

    let status = resp.status();
    if status.is_success() {
        resp.json().map_err(|e| {
            GitLabError::api(
                ErrorCategory::ParseError,
                500,
                format!("Failed to parse OAuth response: {e}"),
                ErrorContext::default(),
            )
        })
    } else {
        let body = resp.text().unwrap_or_default();
        Err(GitLabError::api(
            ErrorCategory::AuthenticationFailed,
            status.as_u16(),
            body.clone(),
            ErrorContext {
                operation: Some("oauth.exchange_authorization_code".into()),
                http_status: Some(status.as_u16()),
                response_body: Some(body),
                ..Default::default()
            },
        ))
    }
}
