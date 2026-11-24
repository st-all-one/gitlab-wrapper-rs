//! Testes de integração do binário `glw` com API stub via `wiremock`.
//!
//! Cada caso usa HOME isolado, ambiente controlado e servidor HTTP local:
//! não há rede externa, relógio ou ordem dependente entre testes.
//! Contrato confirmado pelo dono do binário: `args` do `raw` em batch é um
//! objeto `{"method":"GET","path":"..."}` ou um array de dois strings; o
//! `config` é utilitário de texto e ignora `--format`.

#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr,
    clippy::unwrap_used,
    reason = "suíte de integração: asserts e fixtures são legítimos aqui"
)]

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

type TestResult = Result<(), Box<dyn std::error::Error>>;

const FIXTURES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/cli");

/// Diretório temporário único por teste (sem corrida entre suítes paralelas).
fn temp_home() -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let serial = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("glw-cli-{}-{serial}", std::process::id()));
    let _ignored = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("criar diretório temporário");
    dir
}

/// `Command` com ambiente controlado: nenhuma credencial do desenvolvedor vaza
/// para o binário sob teste.
fn glw(home: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_glw"));
    command.env_clear();
    command.current_dir(home);
    command.env("HOME", home);
    if let Some(value) = std::env::var_os("PATH") {
        command.env("PATH", value);
    }
    command.env("LANG", "C.UTF-8");
    command
}

fn run(home: &Path, args: &[&str]) -> std::io::Result<Output> {
    glw(home)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
}

/// Executa com stdin conectado, semelhante a `echo '...' | glw batch`.
fn run_stdin(home: &Path, args: &[&str], input: &[u8]) -> std::io::Result<Output> {
    let mut child = glw(home)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(input)?;
    }
    child.wait_with_output()
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn fixture(name: &str) -> PathBuf {
    Path::new(FIXTURES).join(name)
}

/// Instala mock para `GET /api/v4/user` com o usuário de teste.
async fn mock_current_user(server: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/api/v4/user"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 7,
            "username": "cli-tester",
            "name": "CLI Tester"
        })))
        .mount(server)
        .await;
}

/// Configura `base_url` apontando para o servidor local.
fn base_args(server: &MockServer) -> Vec<String> {
    vec!["--base-url".into(), server.uri()]
}

fn args_refs(args: &[String]) -> Vec<&str> {
    args.iter().map(String::as_str).collect()
}

fn field<'a>(value: &'a serde_json::Value, keys: &[&str]) -> Option<&'a serde_json::Value> {
    keys.iter()
        .try_fold(value, |current, key| current.get(*key))
}

/// Aceita a resposta crua da API (`{...}`) ou um envelope (`{"data": {...}}`).
fn unwrap_data(value: &serde_json::Value) -> &serde_json::Value {
    if let Some(data) = value.get("data")
        && (data.is_object() || data.is_array())
    {
        return data;
    }
    value
}

fn header_is(request: &Request, name: &str, expected: &str) -> bool {
    request
        .headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value == expected)
}

fn query_pairs(request: &Request) -> Vec<(String, String)> {
    request
        .url
        .query_pairs()
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect()
}

// ---------------------------------------------------------------------------
// (a) --help
// ---------------------------------------------------------------------------

#[test]
fn help_lists_all_subcommands() -> TestResult {
    let home = temp_home();
    let out = run(&home, &["--help"])?;
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    let text = stdout(&out);
    for subcommand in [
        "projects",
        "issues",
        "merge-requests",
        "users",
        "pipelines",
        "groups",
        "whoami",
        "raw",
        "config",
        "prime",
        "help",
    ] {
        assert!(
            text.contains(subcommand),
            "--help não lista `{subcommand}`: {text}"
        );
    }
    Ok(())
}

#[test]
fn help_documents_credentials_and_usage() -> TestResult {
    let home = temp_home();
    for args in [
        vec!["--help"],
        vec!["whoami", "--help"],
        vec!["config", "check", "--help"],
    ] {
        let out = run(&home, &args)?;
        assert!(out.status.success(), "stderr: {}", stderr(&out));
        let text = stdout(&out);
        assert!(text.contains("GLW_TOKEN"), "ajuda {args:?}: {text}");
        assert!(text.contains("token_file"), "ajuda {args:?}: {text}");
        assert!(text.contains("glw config check"), "ajuda {args:?}: {text}");
    }
    Ok(())
}

#[test]
fn prime_is_static_and_documents_auth_and_flow() -> TestResult {
    let home = temp_home();
    let first = run(&home, &["prime"])?;
    let second = run(&home, &["prime"])?;
    assert!(first.status.success(), "stderr: {}", stderr(&first));
    assert_eq!(stdout(&first), stdout(&second));
    let text = stdout(&first);
    assert!(text.contains("GLW PRIME v1"), "protocolo: {text}");
    assert!(text.contains("AUTH:"), "protocolo: {text}");
    assert!(text.contains("glw config check"), "protocolo: {text}");
    assert!(text.contains("EXIT:"), "protocolo: {text}");

    let long = run(&home, &["prime", "--long"])?;
    assert!(long.status.success(), "stderr: {}", stderr(&long));
    let long_text = stdout(&long);
    assert!(
        long_text.contains("GRAMMAR:"),
        "protocolo longo: {long_text}"
    );
    assert!(long_text.contains("VERBS:"), "protocolo longo: {long_text}");
    Ok(())
}

#[test]
fn missing_required_arguments_report_remediation() -> TestResult {
    let home = temp_home();
    let out = run(&home, &["raw"])?;
    assert_eq!(out.status.code(), Some(2), "stderr: {}", stderr(&out));
    let text = stderr(&out);
    assert!(text.contains("--path <PATH>"), "erro: {text}");
    assert!(text.contains("--help"), "erro: {text}");
    Ok(())
}

#[test]
fn invalid_params_report_usage_and_help_hint() -> TestResult {
    let home = temp_home();
    let out = run(
        &home,
        &["projects", "list", "--params", "invalido", "--dry-run"],
    )?;
    assert_eq!(out.status.code(), Some(2), "stderr: {}", stderr(&out));
    let text = stderr(&out);
    assert!(text.contains("parâmetro inválido"), "erro: {text}");
    assert!(text.contains("glw help"), "erro: {text}");
    Ok(())
}

// ---------------------------------------------------------------------------
// (b) whoami com token via env e saída JSON
// ---------------------------------------------------------------------------

#[tokio::test]
async fn whoami_uses_env_token_and_emits_json() -> TestResult {
    let server = MockServer::start().await;
    mock_current_user(&server).await;
    let home = temp_home();

    let base = base_args(&server);
    let mut args = args_refs(&base);
    args.extend_from_slice(&["--format", "json", "--token", "env-token", "whoami"]);

    let out = run(&home, &args)?;
    assert!(out.status.success(), "stderr: {}", stderr(&out));

    let value: serde_json::Value = serde_json::from_str(stdout(&out).trim())?;
    let data = unwrap_data(&value);
    assert_eq!(
        field(data, &["username"]).and_then(serde_json::Value::as_str),
        Some("cli-tester")
    );

    let requests = server
        .received_requests()
        .await
        .expect("servidor local ativo");
    assert_eq!(
        requests.len(),
        1,
        "whoami deve fazer uma requisição: {requests:?}"
    );
    assert_eq!(requests[0].url.path(), "/api/v4/user");
    assert!(
        header_is(&requests[0], "authorization", "Bearer env-token"),
        "cabeçalho de autenticação inesperado: {:?}",
        requests[0].headers
    );
    Ok(())
}

#[tokio::test]
async fn whoami_accepts_token_from_gitlab_token_env() -> TestResult {
    let server = MockServer::start().await;
    mock_current_user(&server).await;
    let home = temp_home();

    let base = base_args(&server);
    let mut args = args_refs(&base);
    args.extend_from_slice(&["--format", "json", "whoami"]);

    let out = glw(&home)
        .env("GITLAB_TOKEN", "legacy-env-token")
        .args(args)
        .stdin(Stdio::null())
        .output()?;
    assert!(out.status.success(), "stderr: {}", stderr(&out));

    let requests = server
        .received_requests()
        .await
        .expect("servidor local ativo");
    assert_eq!(requests.len(), 1, "whoami deve fazer uma requisição");
    assert!(
        header_is(&requests[0], "authorization", "Bearer legacy-env-token"),
        "GITLAB_TOKEN não foi usado: {:?}",
        requests[0].headers
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// (c) projects list com --params k=v
// ---------------------------------------------------------------------------

#[tokio::test]
async fn projects_list_params_kv_become_query_pairs() -> TestResult {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v4/projects"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {"id": 11, "name": "alpha", "path": "alpha"}
        ])))
        .mount(&server)
        .await;
    let home = temp_home();

    let base = base_args(&server);
    let mut args = args_refs(&base);
    args.extend_from_slice(&[
        "--format",
        "json",
        "projects",
        "list",
        "--params",
        "per_page=5,page=2",
    ]);

    let out = run(&home, &args)?;
    assert!(out.status.success(), "stderr: {}", stderr(&out));

    let value: serde_json::Value = serde_json::from_str(stdout(&out).trim())?;
    let data = unwrap_data(&value);
    let items = data.as_array().expect("lista de projetos em JSON");
    assert_eq!(items.len(), 1);
    assert_eq!(
        field(&items[0], &["id"]).and_then(serde_json::Value::as_u64),
        Some(11)
    );

    let requests = server
        .received_requests()
        .await
        .expect("servidor local ativo");
    assert_eq!(requests.len(), 1, "esperada uma requisição");
    let pairs = query_pairs(&requests[0]);
    assert!(
        pairs
            .iter()
            .any(|(key, value)| key == "per_page" && value == "5"),
        "query sem per_page=5: {pairs:?}"
    );
    assert!(
        pairs
            .iter()
            .any(|(key, value)| key == "page" && value == "2"),
        "query sem page=2: {pairs:?}"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// (d) stdin/heredoc para batch
// ---------------------------------------------------------------------------

#[tokio::test]
async fn batch_reads_heredoc_from_stdin() -> TestResult {
    let server = MockServer::start().await;
    mock_current_user(&server).await;
    let home = temp_home();

    let input = b"{\"op\":\"whoami\",\"args\":{},\"params\":{}}\n";
    let base = base_args(&server);
    let mut args = args_refs(&base);
    args.extend_from_slice(&["--format", "json", "batch"]);

    let out = run_stdin(&home, &args, input)?;
    assert!(out.status.success(), "stderr: {}", stderr(&out));

    let text = stdout(&out);
    let line = text
        .lines()
        .next()
        .expect("batch deve emitir uma linha NDJSON");
    let value: serde_json::Value = serde_json::from_str(line)?;
    assert_eq!(
        value.get("ok").and_then(serde_json::Value::as_bool),
        Some(true)
    );
    assert_eq!(
        value.get("op").and_then(serde_json::Value::as_str),
        Some("whoami")
    );
    assert!(value.get("result").is_some(), "resultado ausente: {value}");

    let requests = server
        .received_requests()
        .await
        .expect("servidor local ativo");
    assert_eq!(requests.len(), 1, "uma operação no lote = uma requisição");
    Ok(())
}

// ---------------------------------------------------------------------------
// (e) --params como objeto JSON
// ---------------------------------------------------------------------------

#[tokio::test]
async fn params_accepts_json_object() -> TestResult {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v4/projects"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
        .mount(&server)
        .await;
    let home = temp_home();

    let base = base_args(&server);
    let mut args = args_refs(&base);
    args.extend_from_slice(&[
        "--format",
        "json",
        "projects",
        "list",
        "--params",
        r#"{"per_page":5,"page":2}"#,
    ]);

    let out = run(&home, &args)?;
    assert!(out.status.success(), "stderr: {}", stderr(&out));

    let requests = server
        .received_requests()
        .await
        .expect("servidor local ativo");
    assert_eq!(requests.len(), 1, "esperada uma requisição");
    let pairs = query_pairs(&requests[0]);
    assert!(
        pairs
            .iter()
            .any(|(key, value)| key == "per_page" && value == "5"),
        "query sem per_page=5: {pairs:?}"
    );
    assert!(
        pairs
            .iter()
            .any(|(key, value)| key == "page" && value == "2"),
        "query sem page=2: {pairs:?}"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// (f) precedência de token-file sobre env
// ---------------------------------------------------------------------------

#[tokio::test]
async fn token_file_takes_precedence_over_env_token() -> TestResult {
    let server = MockServer::start().await;
    mock_current_user(&server).await;
    let home = temp_home();
    let token_path = fixture("token.txt");

    let base = base_args(&server);
    let mut args = args_refs(&base);
    args.extend_from_slice(&[
        "--token-file",
        token_path.to_str().expect("caminho UTF-8"),
        "--format",
        "json",
        "whoami",
    ]);

    let out = glw(&home)
        .env("GLW_TOKEN", "env-token")
        .args(args)
        .stdin(Stdio::null())
        .output()?;
    assert!(out.status.success(), "stderr: {}", stderr(&out));

    let requests = server
        .received_requests()
        .await
        .expect("servidor local ativo");
    assert_eq!(requests.len(), 1, "whoami deve fazer uma requisição");
    assert!(
        header_is(&requests[0], "authorization", "Bearer token-from-file"),
        "token-file deve prevalecer sobre GLW_TOKEN: {:?}",
        requests[0].headers
    );
    Ok(())
}

#[tokio::test]
async fn explicit_token_flag_takes_precedence_over_token_file() -> TestResult {
    let server = MockServer::start().await;
    mock_current_user(&server).await;
    let home = temp_home();

    let base = base_args(&server);
    let token_path = fixture("token.txt");
    let mut args = args_refs(&base);
    args.extend_from_slice(&[
        "--token",
        "flag-token",
        "--token-file",
        token_path.to_str().expect("caminho UTF-8"),
        "--format",
        "json",
        "whoami",
    ]);

    let out = run(&home, &args)?;
    assert!(out.status.success(), "stderr: {}", stderr(&out));

    let requests = server
        .received_requests()
        .await
        .expect("servidor local ativo");
    assert_eq!(requests.len(), 1, "whoami deve fazer uma requisição");
    assert!(
        header_is(&requests[0], "authorization", "Bearer flag-token"),
        "--token deve prevalecer sobre --token-file: {:?}",
        requests[0].headers
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// (g) config inválida → exit 2
// ---------------------------------------------------------------------------

#[test]
fn bad_config_returns_usage_exit_code() -> TestResult {
    let home = temp_home();
    let config = fixture("bad-config.toml");
    let config = config.to_str().expect("caminho UTF-8");

    let out = run(&home, &["--config", config, "whoami"])?;
    assert_eq!(out.status.code(), Some(2), "stderr: {}", stderr(&out));
    Ok(())
}

#[test]
fn config_check_reports_ok_for_valid_file() -> TestResult {
    let home = temp_home();
    let config = fixture("config.toml");
    let config = config.to_str().expect("caminho UTF-8");

    let out = run(&home, &["--config", config, "config", "check"])?;
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    Ok(())
}

#[test]
fn config_path_shows_resolved_default_location() -> TestResult {
    let home = temp_home();
    let out = run(&home, &["config", "path"])?;
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        stdout(&out).contains("config.toml"),
        "`config path` deve mostrar o arquivo: {}",
        stdout(&out)
    );
    Ok(())
}

#[test]
fn config_check_uses_profile_token_file_over_base_token() -> TestResult {
    let home = temp_home();
    let config = fixture("profile-token-file.toml");
    let config = config.to_str().expect("caminho UTF-8");

    let out = run(
        &home,
        &["--config", config, "--profile", "ci", "config", "check"],
    )?;
    let text = stdout(&out);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(text.contains("profile: ci"), "saída: {text}");
    assert!(text.contains("token: set"), "saída: {text}");
    assert!(
        text.contains("base_url: https://ci.example.com"),
        "saída: {text}"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// (h) API 404 → exit 3 e stderr claro
// ---------------------------------------------------------------------------

#[tokio::test]
async fn api_404_returns_api_exit_code_and_clear_stderr() -> TestResult {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v4/projects/999"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "message": "404 Project Not Found"
        })))
        .mount(&server)
        .await;
    let home = temp_home();

    let base = base_args(&server);
    let mut args = args_refs(&base);
    args.extend_from_slice(&["projects", "get", "999"]);

    let out = run(&home, &args)?;
    assert_eq!(out.status.code(), Some(3), "stderr: {}", stderr(&out));
    let text = stderr(&out).to_ascii_lowercase();
    assert!(
        text.contains("404") && text.contains("not found"),
        "erro deve citar status e descrição legível: {text}"
    );
    assert!(
        stdout(&out).trim().is_empty(),
        "erros não devem poluir stdout: {}",
        stdout(&out)
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// (i) --dry-run sem rede
// ---------------------------------------------------------------------------

#[tokio::test]
async fn dry_run_prints_request_without_network() -> TestResult {
    let server = MockServer::start().await;
    let home = temp_home();

    let base = base_args(&server);
    let mut args = args_refs(&base);
    args.extend_from_slice(&[
        "--dry-run",
        "projects",
        "list",
        "--params",
        "per_page=5,page=2",
    ]);

    let out = run(&home, &args)?;
    assert!(out.status.success(), "stderr: {}", stderr(&out));

    let text = stdout(&out);
    let value: serde_json::Value = serde_json::from_str(text.trim())?;
    assert_eq!(
        value.get("dry_run").and_then(serde_json::Value::as_bool),
        Some(true)
    );
    assert!(
        value
            .get("method")
            .and_then(serde_json::Value::as_str)
            .is_some(),
        "dry-run deve mostrar o método: {text}"
    );
    let path = value
        .get("path")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default();
    assert!(
        path.contains("projects"),
        "dry-run deve mostrar o path: {text}"
    );
    let query = value.get("query").and_then(serde_json::Value::as_object);
    assert!(
        query.is_some_and(|map| {
            map.get("per_page").and_then(serde_json::Value::as_str) == Some("5")
                && map.get("page").and_then(serde_json::Value::as_str) == Some("2")
        }),
        "dry-run deve mostrar os parâmetros resolvidos: {text}"
    );

    let requests = server
        .received_requests()
        .await
        .expect("servidor local ativo");
    assert!(
        requests.is_empty(),
        "dry-run não deve fazer requisições: {requests:?}"
    );
    Ok(())
}

#[test]
fn dry_run_does_not_leak_token() -> TestResult {
    let home = temp_home();
    let out = run(
        &home,
        &[
            "--token",
            "secret-token-should-not-leak",
            "--dry-run",
            "whoami",
        ],
    )?;
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        !stdout(&out).contains("secret-token-should-not-leak"),
        "dry-run não deve imprimir o token: {}",
        stdout(&out)
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Contratos adicionais de saída/erro
// ---------------------------------------------------------------------------

/// `@-` em `--token-file` lê o segredo do stdin, sem arquivo temporário.
#[tokio::test]
async fn token_file_at_stdin_is_supported() -> TestResult {
    let server = MockServer::start().await;
    mock_current_user(&server).await;
    let home = temp_home();

    let base = base_args(&server);
    let mut args = args_refs(&base);
    args.extend_from_slice(&["--token-file", "@-", "--format", "json", "whoami"]);

    let out = run_stdin(&home, &args, b"stdin-token\n")?;
    assert!(out.status.success(), "stderr: {}", stderr(&out));

    let requests = server
        .received_requests()
        .await
        .expect("servidor local ativo");
    assert_eq!(requests.len(), 1, "whoami deve fazer uma requisição");
    assert!(
        header_is(&requests[0], "authorization", "Bearer stdin-token"),
        "token lido do stdin não foi usado: {:?}",
        requests[0].headers
    );
    Ok(())
}

#[test]
fn unknown_subcommand_returns_usage_exit_code() -> TestResult {
    let home = temp_home();
    let out = run(&home, &["not-a-command"])?;
    assert_eq!(out.status.code(), Some(2), "stderr: {}", stderr(&out));
    Ok(())
}

#[test]
fn unreadable_token_file_returns_usage_or_io_exit_code() -> TestResult {
    let home = temp_home();
    let missing = home.join("missing-token.txt");
    let missing = missing.to_str().expect("caminho UTF-8");

    let out = run(&home, &["--token-file", missing, "whoami"])?;
    let code = out.status.code();
    assert!(
        code == Some(2) || code == Some(4),
        "token-file ausente deve falhar com 2 ou 4, obteve {code:?}: {}",
        stderr(&out)
    );
    Ok(())
}

#[test]
fn batch_envelope_uses_deterministic_field_names() -> TestResult {
    let home = temp_home();
    let input = br#"{"op":"raw","args":{"method":"GET","path":"/api/v4/user"},"params":{}}
"#;
    let out = run_stdin(&home, &["--dry-run", "batch"], input)?;
    assert!(out.status.success(), "stderr: {}", stderr(&out));

    let output_text = stdout(&out);
    let line = output_text
        .lines()
        .next()
        .expect("batch deve emitir uma linha");
    let value: serde_json::Value = serde_json::from_str(line)?;
    assert!(value.get("ok").is_some(), "campo `ok` ausente: {value}");
    assert!(value.get("op").is_some(), "campo `op` ausente: {value}");
    assert!(
        value.get("result").is_some() || value.get("error").is_some(),
        "campo `result` ou `error` ausente: {value}"
    );
    Ok(())
}

#[test]
fn batch_preserves_order_for_multiple_operations() -> TestResult {
    let home = temp_home();
    let input = concat!(
        "{\"op\":\"raw\",\"args\":[\"GET\",\"/api/v4/user\"],\"params\":{}}\n",
        "{\"op\":\"raw\",\"args\":[\"GET\",\"/api/v4/projects\"],\"params\":{}}\n"
    );
    let out = run_stdin(&home, &["--dry-run", "batch"], input.as_bytes())?;
    assert!(out.status.success(), "stderr: {}", stderr(&out));

    let values: Vec<serde_json::Value> = stdout(&out)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()?;
    assert_eq!(values.len(), 2, "uma linha de saída por operação");
    assert_eq!(
        values[0].get("op").and_then(serde_json::Value::as_str),
        Some("raw")
    );
    assert_eq!(
        values[1].get("op").and_then(serde_json::Value::as_str),
        Some("raw")
    );
    Ok(())
}

#[test]
fn fixture_operations_ndjson_is_valid() -> TestResult {
    let text = std::fs::read_to_string(fixture("operations.ndjson"))?;
    let lines = text.lines().filter(|line| !line.trim().is_empty()).count();
    assert_eq!(lines, 2, "fixture de lote deve conter duas operações");
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        let value: serde_json::Value = serde_json::from_str(line)?;
        assert!(
            value
                .get("op")
                .and_then(serde_json::Value::as_str)
                .is_some()
        );
    }
    Ok(())
}

#[test]
fn config_path_and_check_are_text_commands() -> TestResult {
    let home = temp_home();
    let config = fixture("config.toml");
    let config = config.to_str().expect("caminho UTF-8");

    let out = run(
        &home,
        &["--config", config, "--format", "json", "config", "check"],
    )?;
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    let text = stdout(&out);
    assert!(!text.trim().is_empty(), "config check deve imprimir texto");
    assert!(
        text.contains("ok") || text.contains("válid") || text.contains("valid"),
        "config check deve ser legível: {text}"
    );
    Ok(())
}
