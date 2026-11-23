//! Testes deterministas de comportamento público: codificação, ordem de filtros,
//! paginação observável, erro 429 e estabilidade de identificadores em erros.
//!
//! Nenhum teste depende de relógio, aleatoriedade, ordem de `HashMap` ou de rede
//! externa — o servidor é sempre um `wiremock::MockServer` local.

use std::collections::BTreeMap;

use gitlab_wrapper::utils::encoding::{encode_query_param, filter_to_query};
use gitlab_wrapper::{
    ErrorCategory, ErrorContext, GitLabClient, GitLabConfig, GitLabError, PaginationParams,
};
use serde::Serialize;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Constrói cliente apontando para um `MockServer` local.
fn client_for(server: &MockServer) -> GitLabClient {
    GitLabClient::new(GitLabConfig {
        base_url: server.uri(),
        token: Some("deterministic-token".into()),
        ..Default::default()
    })
    .expect("configuração local válida")
}

// ---------------------------------------------------------------------------
// 1. Ordem estável de filter_to_query
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct FlatFilter {
    zebra: u32,
    alpha: u32,
    middle: u32,
}

#[derive(Serialize)]
struct NestedFilter {
    zebra: InnerFilter,
    alpha: InnerFilter,
}

#[derive(Serialize)]
struct InnerFilter {
    second: &'static str,
    first: &'static str,
}

#[derive(Serialize)]
struct RepeatedKeys {
    tag: Vec<&'static str>,
}

#[test]
fn filter_to_query_em_ordem_lexicografica_de_chaves() {
    let filter = FlatFilter {
        zebra: 3,
        alpha: 1,
        middle: 2,
    };
    let actual = filter_to_query(Some(&filter));
    let expected = vec![
        ("alpha".to_string(), "1".to_string()),
        ("middle".to_string(), "2".to_string()),
        ("zebra".to_string(), "3".to_string()),
    ];
    assert_eq!(actual, expected);

    // A mesma entrada deve produzir exatamente a mesma sequência em toda chamada.
    for _ in 0..8 {
        assert_eq!(filter_to_query(Some(&filter)), expected);
    }
}

#[test]
fn filter_to_query_em_ordem_lexicografica_com_chaves_aninhadas() {
    let filter = NestedFilter {
        zebra: InnerFilter {
            second: "z2",
            first: "z1",
        },
        alpha: InnerFilter {
            second: "a2",
            first: "a1",
        },
    };
    assert_eq!(
        filter_to_query(Some(&filter)),
        vec![
            ("alpha[first]".to_string(), "a1".to_string()),
            ("alpha[second]".to_string(), "a2".to_string()),
            ("zebra[first]".to_string(), "z1".to_string()),
            ("zebra[second]".to_string(), "z2".to_string()),
        ]
    );
}

#[test]
fn filter_to_query_preserva_ordem_de_valores_de_array() {
    let filter = RepeatedKeys {
        tag: vec!["delta", "alpha", "charlie", "bravo"],
    };
    assert_eq!(
        filter_to_query(Some(&filter)),
        vec![
            ("tag".to_string(), "delta".to_string()),
            ("tag".to_string(), "alpha".to_string()),
            ("tag".to_string(), "charlie".to_string()),
            ("tag".to_string(), "bravo".to_string()),
        ]
    );
}

#[test]
fn filter_to_query_ignora_nulos_e_omite_filtro_ausente() {
    let filter = BTreeMap::from([
        ("null_value".to_string(), serde_json::Value::Null),
        ("value".to_string(), serde_json::Value::from(false)),
    ]);
    assert_eq!(
        filter_to_query(Some(&filter)),
        vec![("value".to_string(), "false".to_string())]
    );
    assert_eq!(
        filter_to_query(None::<&FlatFilter>),
        Vec::<(String, String)>::new()
    );
}

// ---------------------------------------------------------------------------
// 2. Codificação determinista (ASCII, Unicode, caracteres especiais)
// ---------------------------------------------------------------------------

#[test]
fn encode_query_param_casos_representativos() {
    let cases = [
        ("simples", "simples"),
        ("com espaço", "com+espa%C3%A7o"),
        ("a&b=c?d#e/f", "a%26b%3Dc%3Fd%23e%2Ff"),
        (
            "café & © 2026 — 🚀",
            "caf%C3%A9+%26+%C2%A9+2026+%E2%80%94+%F0%9F%9A%80",
        ),
        ("linha\nquebrada\ttab", "linha%0Aquebrada%09tab"),
    ];
    for (input, expected) in cases {
        assert_eq!(encode_query_param(input), expected, "entrada: {input}");
    }
    assert_eq!(encode_query_param(""), "");
}

// ---------------------------------------------------------------------------
// 3. Paginação observada por comportamento público
// ---------------------------------------------------------------------------

/// Gera uma página com `quantity` projetos sintéticos, sem depender de relógio.
fn projects_json(quantity: usize, offset: usize) -> serde_json::Value {
    let mut items = Vec::with_capacity(quantity);
    for index in 0..quantity {
        let project_id = offset + index + 1;
        items.push(serde_json::json!({
            "id": project_id,
            "name": format!("projeto-{project_id}"),
            "path": format!("projeto-{project_id}")
        }));
    }
    serde_json::Value::Array(items)
}

#[tokio::test]
async fn parametros_de_paginacao_publicos_sao_estaveis() {
    let params = PaginationParams {
        page: Some(2),
        per_page: Some(50),
        pagination: Some("offset".into()),
        order_by: Some("id".into()),
        sort: Some("asc".into()),
    };
    let actual = filter_to_query(Some(&params));
    assert_eq!(
        actual,
        vec![
            ("order_by".to_string(), "id".to_string()),
            ("page".to_string(), "2".to_string()),
            ("pagination".to_string(), "offset".to_string()),
            ("per_page".to_string(), "50".to_string()),
            ("sort".to_string(), "asc".to_string()),
        ]
    );
}

#[tokio::test]
async fn paginacao_automatica_segue_x_next_page_e_para_no_fim() {
    let server = MockServer::start().await;
    let client = client_for(&server);

    Mock::given(method("GET"))
        .and(path("/api/v4/projects"))
        .and(query_param("page", "1"))
        .and(query_param("per_page", "20"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(projects_json(20, 0))
                .append_header("x-page", "1")
                .append_header("x-per-page", "20")
                .append_header("x-total", "25")
                .append_header("x-total-pages", "2")
                .append_header("x-next-page", "2"),
        )
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v4/projects"))
        .and(query_param("page", "2"))
        .and(query_param("per_page", "20"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(projects_json(5, 20))
                .append_header("x-page", "2")
                .append_header("x-per-page", "20")
                .append_header("x-total", "25")
                .append_header("x-total-pages", "2")
                .append_header("x-next-page", ""),
        )
        .mount(&server)
        .await;

    let projects = client
        .projects
        .list_all(None)
        .await
        .expect("paginação deve completar com duas requisições");
    assert_eq!(projects.len(), 25);
    assert_eq!(projects.first().map(|p| p.id), Some(1));
    assert_eq!(projects.last().map(|p| p.id), Some(25));

    let requests = server
        .received_requests()
        .await
        .expect("servidor local ativo");
    assert_eq!(
        requests.len(),
        2,
        "paginação deve parar ao esgotar x-next-page"
    );
    assert_eq!(
        requests[0]
            .url
            .query_pairs()
            .find(|(k, _)| k == "page")
            .map(|(_, v)| v.into_owned()),
        Some("1".into())
    );
    assert_eq!(
        requests[1]
            .url
            .query_pairs()
            .find(|(k, _)| k == "page")
            .map(|(_, v)| v.into_owned()),
        Some("2".into())
    );
}

// ---------------------------------------------------------------------------
// 4. Erro 429 com retry_after e categoria observáveis
// ---------------------------------------------------------------------------

#[tokio::test]
async fn erro_429_expõe_retry_after_e_categoria() {
    let server = MockServer::start().await;
    let client = client_for(&server);

    Mock::given(method("GET"))
        .and(path("/api/v4/projects/7"))
        .respond_with(
            ResponseTemplate::new(429)
                .set_body_json(serde_json::json!({
                    "message": { "base": ["Rate limit exceeded"] },
                    "retry_after": 17
                }))
                .append_header("retry-after", "17"),
        )
        .mount(&server)
        .await;

    let error = client
        .projects
        .get(7)
        .await
        .expect_err("HTTP 429 deve ser erro");

    match &error {
        GitLabError::RateLimited {
            retry_after,
            context,
        } => {
            assert_eq!(*retry_after, Some(17));
            assert_eq!(context.http_status, Some(429));
            assert_eq!(context.operation.as_deref(), Some("projects.get"));
            assert_eq!(
                context.api_errors.as_deref(),
                Some(["Rate limit exceeded".to_string()].as_slice())
            );
        }
        other => panic!("esperado GitLabError::RateLimited, obtido {other:?}"),
    }
    assert_eq!(error.category(), Some(ErrorCategory::RateLimited));
    assert!(error.to_string().contains("17"));
    assert_eq!(
        ErrorCategory::from_status(429),
        Some(ErrorCategory::RateLimited)
    );
    assert_eq!(ErrorCategory::RateLimited.http_status(), 429);
}

#[tokio::test]
async fn erro_429_sem_corpo_json_retorna_retry_after_none() {
    let server = MockServer::start().await;
    let client = client_for(&server);

    Mock::given(method("GET"))
        .and(path("/api/v4/projects/8"))
        .respond_with(ResponseTemplate::new(429).set_body_string("too many requests"))
        .mount(&server)
        .await;

    let error = client
        .projects
        .get(8)
        .await
        .expect_err("HTTP 429 deve ser erro");

    match &error {
        GitLabError::RateLimited {
            retry_after,
            context,
        } => {
            assert_eq!(*retry_after, None);
            assert_eq!(context.http_status, Some(429));
            assert_eq!(context.api_errors, None);
        }
        other => panic!("esperado GitLabError::RateLimited, obtido {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// 5. Identificadores de erro: forma UUID v7, sem floats e sem vazamento de Float
// ---------------------------------------------------------------------------

#[test]
fn instancia_de_erro_e_uuid_v7_com_categoria_estavel() {
    let error = GitLabError::api(
        ErrorCategory::RateLimited,
        429,
        "rate limited",
        ErrorContext {
            operation: Some("projects.get".into()),
            ..Default::default()
        },
    );
    let GitLabError::Api {
        instance,
        category,
        status,
        ..
    } = &error
    else {
        panic!("esperado GitLabError::Api");
    };
    assert_eq!(instance.len(), 36, "UUID canônico tem 36 caracteres");
    assert_eq!(
        instance.as_bytes()[14],
        b'7',
        "UUID v7 tem '7' na posição da versão"
    );
    assert!(
        instance
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() || byte == b'-')
    );
    assert!(!instance.contains('.'));
    assert_eq!(*category, ErrorCategory::RateLimited);
    assert_eq!(*status, 429);

    // Campos de erro usam inteiros e strings, nunca ponto flutuante.
    let display = error.to_string();
    assert!(!display.contains("NaN") && !display.contains("inf"));
}

#[test]
fn modo_de_depuracao_de_config_redaciona_token_nunca_o_expõe() {
    let config = GitLabConfig {
        base_url: "https://gitlab.example.com".into(),
        token: Some("super-segredo".into()),
        ..Default::default()
    };
    let debug = format!("{config:?}");
    assert!(debug.contains("REDACTED"));
    assert!(!debug.contains("super-segredo"));
}
