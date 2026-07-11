# Referência da API

Documentação completa de todas as structs, enums e métodos do `gitlab-wrapper-rs`.

---

## Índice

- [GitLabClient](#gitlabclient)
- [GitLabConfig / ResolvedConfig](#gitlabconfig--resolvedconfig)
- [Resources (25 structs)](#resources-25-structs)
- [GitLabError](#gitlaberror)
- [ErrorCategory](#errorcategory)
- [ErrorContext](#errorcontext)
- [PaginationInfo](#paginationinfo)
- [Módulo OAuth](#módulo-oauth)

---

## GitLabClient

Struct principal do cliente. Segue o padrão **Factory**: construtor público que retorna `Result`.

```rust
pub struct GitLabClient { /* private fields */ }
```

### Acesso aos Resources

O `GitLabClient` implementa `Deref<Target = ResourceGroup>`, então todos os resources
são acessíveis diretamente via `.`:

```rust
let gl = GitLabClient::new(config)?;
gl.projects.list(None)?;   // ResourceGroup::projects
gl.issues.list(None)?;     // ResourceGroup::issues
// etc.
```

### ResourceGroup

```rust
pub struct ResourceGroup {
    pub branches: BranchesResource,
    pub commits: CommitsResource,
    pub deploy_keys: DeployKeysResource,
    pub discussions: DiscussionsResource,
    pub environments: EnvironmentsResource,
    pub events: EventsResource,
    pub groups: GroupsResource,
    pub issues: IssuesResource,
    pub jobs: JobsResource,
    pub labels: LabelsResource,
    pub members: MembersResource,
    pub merge_requests: MergeRequestsResource,
    pub milestones: MilestonesResource,
    pub notes: NotesResource,
    pub pipeline_schedules: PipelineSchedulesResource,
    pub pipelines: PipelinesResource,
    pub projects: ProjectsResource,
    pub releases: ReleasesResource,
    pub repository_files: RepositoryFilesResource,
    pub runners: RunnersResource,
    pub search: SearchResource,
    pub tags: TagsResource,
    pub todos: TodosResource,
    pub users: UsersResource,
    pub wikis: WikisResource,
}
```

### Métodos

```rust
impl GitLabClient {
    pub fn new(config: GitLabConfig) -> Result<Self, GitLabError>;
    pub fn config(&self) -> &ResolvedConfig;
}
```

| Método | Descrição |
|--------|-----------|
| `new(config)` | Cria nova instância. Valida `base_url` não vazia. |
| `config()` | Retorna referência à configuração resolvida (token redactado no Debug). |

---

## GitLabConfig / ResolvedConfig

### GitLabConfig (input do usuário)

```rust
pub struct GitLabConfig {
    pub base_url: String,                    // obrigatório
    pub token: Option<String>,               // opcional (acesso anônimo)
    pub auth_method: Option<AuthMethod>,     // padrão: Bearer
    pub sudo: Option<String>,                // impersonação
    pub timeout: Option<Duration>,           // padrão: 30s
    pub max_rps: Option<u32>,                // padrão: 10
}
```

### ResolvedConfig (configuração imutável)

```rust
pub struct ResolvedConfig {
    pub base_url: String,
    pub token: Option<String>,
    pub auth_method: AuthMethod,
    pub sudo: Option<String>,
    pub timeout: Duration,
    pub max_rps: u32,
}
```

### AuthMethod

```rust
pub enum AuthMethod {
    Header,   // PRIVATE-TOKEN header
    Bearer,   // Authorization: Bearer (padrão)
}
```

---

## Resources (25 structs)

Cada resource struct contém métodos que retornam `Result<T, GitLabError>`.

### ProjectsResource

```rust
pub fn list(&self, filter: Option<&ProjectFilter>) -> Result<Vec<Project>, GitLabError>;
pub fn list_all(&self, filter: Option<&ProjectFilter>) -> Result<Vec<Project>, GitLabError>;
pub fn get(&self, project_id: u64) -> Result<Project, GitLabError>;
pub fn get_by_path(&self, path: &str) -> Result<Project, GitLabError>;
pub fn create(&self, payload: &CreateProjectPayload) -> Result<Project, GitLabError>;
pub fn update(&self, project_id: u64, payload: &UpdateProjectPayload) -> Result<Project, GitLabError>;
pub fn delete(&self, project_id: u64) -> Result<(), GitLabError>;
pub fn archive(&self, project_id: u64) -> Result<Project, GitLabError>;
pub fn unarchive(&self, project_id: u64) -> Result<Project, GitLabError>;
pub fn fork(&self, project_id: u64, namespace: Option<&str>) -> Result<Project, GitLabError>;
pub fn transfer(&self, project_id: u64, namespace_id: u64) -> Result<Project, GitLabError>;
```

### GroupsResource

```rust
pub fn list(&self, filter: Option<&GroupFilter>) -> Result<Vec<Group>, GitLabError>;
pub fn get(&self, group_id: u64) -> Result<Group, GitLabError>;
pub fn get_by_path(&self, path: &str) -> Result<Group, GitLabError>;
pub fn create(&self, payload: &CreateGroupPayload) -> Result<Group, GitLabError>;
pub fn update(&self, group_id: u64, payload: &UpdateGroupPayload) -> Result<Group, GitLabError>;
pub fn delete(&self, group_id: u64) -> Result<(), GitLabError>;
pub fn subgroups(&self, group_id: u64) -> Result<Vec<Group>, GitLabError>;
pub fn descendant_groups(&self, group_id: u64) -> Result<Vec<Group>, GitLabError>;
pub fn projects(&self, group_id: u64) -> Result<Vec<Project>, GitLabError>;
```

### UsersResource

```rust
pub fn list(&self, filter: Option<&UserFilter>) -> Result<Vec<User>, GitLabError>;
pub fn get(&self, user_id: u64) -> Result<User, GitLabError>;
pub fn get_current(&self) -> Result<User, GitLabError>;
pub fn create(&self, payload: &CreateUserPayload) -> Result<User, GitLabError>;
pub fn update(&self, user_id: u64, payload: &UpdateUserPayload) -> Result<User, GitLabError>;
pub fn delete(&self, user_id: u64) -> Result<(), GitLabError>;
pub fn status(&self, user_id: u64) -> Result<UserStatus, GitLabError>;
pub fn set_status(&self, emoji: Option<&str>, message: Option<&str>) -> Result<UserStatus, GitLabError>;
pub fn preferences(&self) -> Result<UserPreferences, GitLabError>;
pub fn set_preferences(&self, prefs: &serde_json::Value) -> Result<UserPreferences, GitLabError>;
pub fn deactivate(&self, user_id: u64) -> Result<(), GitLabError>;
pub fn activate(&self, user_id: u64) -> Result<(), GitLabError>;
pub fn ban(&self, user_id: u64) -> Result<(), GitLabError>;
pub fn unban(&self, user_id: u64) -> Result<(), GitLabError>;
```

### IssuesResource

```rust
pub fn list(&self, filter: Option<&IssueFilter>) -> Result<Vec<Issue>, GitLabError>;
pub fn list_for_project(&self, project_id: u64, filter: Option<&IssueFilter>) -> Result<Vec<Issue>, GitLabError>;
pub fn get(&self, project_id: u64, issue_iid: u32) -> Result<Issue, GitLabError>;
pub fn create(&self, project_id: u64, payload: &CreateIssuePayload) -> Result<Issue, GitLabError>;
pub fn update(&self, project_id: u64, issue_iid: u32, payload: &UpdateIssuePayload) -> Result<Issue, GitLabError>;
pub fn delete(&self, project_id: u64, issue_iid: u32) -> Result<(), GitLabError>;
pub fn subscribe(&self, project_id: u64, issue_iid: u32) -> Result<Issue, GitLabError>;
pub fn unsubscribe(&self, project_id: u64, issue_iid: u32) -> Result<Issue, GitLabError>;
pub fn set_time_estimate(&self, project_id: u64, issue_iid: u32, duration: &str) -> Result<Issue, GitLabError>;
pub fn add_spent_time(&self, project_id: u64, issue_iid: u32, duration: &str) -> Result<Issue, GitLabError>;
pub fn reset_time_estimate(&self, project_id: u64, issue_iid: u32) -> Result<Issue, GitLabError>;
pub fn reset_spent_time(&self, project_id: u64, issue_iid: u32) -> Result<Issue, GitLabError>;
pub fn move_issue(&self, project_id: u64, issue_iid: u32, to_project_id: u64) -> Result<Issue, GitLabError>;
pub fn get_by_group(&self, group_id: u64, filter: Option<&IssueFilter>) -> Result<Vec<Issue>, GitLabError>;
```

### MergeRequestsResource

```rust
pub fn list(&self, filter: Option<&MergeRequestFilter>) -> Result<Vec<MergeRequest>, GitLabError>;
pub fn list_for_project(&self, project_id: u64, filter: Option<&MergeRequestFilter>) -> Result<Vec<MergeRequest>, GitLabError>;
pub fn get(&self, project_id: u64, mr_iid: u32) -> Result<MergeRequest, GitLabError>;
pub fn create(&self, project_id: u64, payload: &CreateMergeRequestPayload) -> Result<MergeRequest, GitLabError>;
pub fn update(&self, project_id: u64, mr_iid: u32, payload: &UpdateMergeRequestPayload) -> Result<MergeRequest, GitLabError>;
pub fn delete(&self, project_id: u64, mr_iid: u32) -> Result<(), GitLabError>;
pub fn merge(&self, project_id: u64, mr_iid: u32, payload: Option<&MergePayload>) -> Result<MergeRequest, GitLabError>;
pub fn approve(&self, project_id: u64, mr_iid: u32) -> Result<MergeRequest, GitLabError>;
pub fn unapprove(&self, project_id: u64, mr_iid: u32) -> Result<(), GitLabError>;
pub fn rebase(&self, project_id: u64, mr_iid: u32) -> Result<(), GitLabError>;
pub fn cancel_merge_when_pipeline_succeeds(&self, project_id: u64, mr_iid: u32) -> Result<MergeRequest, GitLabError>;
pub fn commits(&self, project_id: u64, mr_iid: u32) -> Result<Vec<Commit>, GitLabError>;
pub fn changes(&self, project_id: u64, mr_iid: u32) -> Result<serde_json::Value, GitLabError>;
pub fn list_by_group(&self, group_id: u64, filter: Option<&MergeRequestFilter>) -> Result<Vec<MergeRequest>, GitLabError>;
```

### BranchesResource, CommitsResource, TagsResource, RepositoryFilesResource, WikisResource

(consulte o [Guia de Uso](./usage-guide.md) para assinaturas completas)

### LabelsResource, MilestonesResource, MembersResource

(CRUD completo para projeto e grupo com escopo separado)

### NotesResource

CRUD completo para 5 noteables: issue, merge request, commit, snippet, wiki.

### DiscussionsResource

CRUD completo de discussões para issue, merge request e commit, incluindo:
- get / create / list
- add_note / update_note / delete_note
- resolve / unresolve

### TodosResource

```rust
pub fn list(&self, filter: Option<&TodoFilter>) -> Result<Vec<Todo>, GitLabError>;
pub fn mark_done(&self, todo_id: u64) -> Result<Todo, GitLabError>;
pub fn mark_all_done(&self) -> Result<Vec<Todo>, GitLabError>;
```

### SearchResource

```rust
pub fn global(&self, scope: &str, search: &str) -> Result<Vec<SearchResultItem>, GitLabError>;
pub fn in_group(&self, group_id: u64, scope: &str, search: &str) -> Result<Vec<SearchResultItem>, GitLabError>;
pub fn in_project(&self, project_id: u64, scope: &str, search: &str) -> Result<Vec<SearchResultItem>, GitLabError>;
```

### EventsResource

```rust
pub fn list(&self, filter: Option<&EventFilter>) -> Result<Vec<Event>, GitLabError>;
pub fn list_user_events(&self, user_id: u64, filter: Option<&EventFilter>) -> Result<Vec<Event>, GitLabError>;
pub fn list_project_events(&self, project_id: u64, filter: Option<&EventFilter>) -> Result<Vec<Event>, GitLabError>;
```

### PipelinesResource

```rust
pub fn list(&self, project_id: u64, filter: Option<&PipelineFilter>) -> Result<Vec<Pipeline>, GitLabError>;
pub fn get(&self, project_id: u64, pipeline_id: u64) -> Result<Pipeline, GitLabError>;
pub fn get_latest(&self, project_id: u64) -> Result<Pipeline, GitLabError>;
pub fn create(&self, project_id: u64, payload: &CreatePipelinePayload) -> Result<Pipeline, GitLabError>;
pub fn retry(&self, project_id: u64, pipeline_id: u64) -> Result<Pipeline, GitLabError>;
pub fn cancel(&self, project_id: u64, pipeline_id: u64) -> Result<Pipeline, GitLabError>;
pub fn delete(&self, project_id: u64, pipeline_id: u64) -> Result<(), GitLabError>;
pub fn variables(&self, project_id: u64, pipeline_id: u64) -> Result<Vec<PipelineVariable>, GitLabError>;
pub fn test_report(&self, project_id: u64, pipeline_id: u64) -> Result<serde_json::Value, GitLabError>;
pub fn test_report_summary(&self, project_id: u64, pipeline_id: u64) -> Result<serde_json::Value, GitLabError>;
```

### JobsResource

```rust
pub fn list(&self, project_id: u64, filter: Option<&JobFilter>) -> Result<Vec<Job>, GitLabError>;
pub fn list_by_pipeline(&self, project_id: u64, pipeline_id: u64, filter: Option<&JobFilter>) -> Result<Vec<Job>, GitLabError>;
pub fn get(&self, project_id: u64, job_id: u64) -> Result<Job, GitLabError>;
pub fn trace(&self, project_id: u64, job_id: u64) -> Result<String, GitLabError>;
pub fn cancel(&self, project_id: u64, job_id: u64) -> Result<Job, GitLabError>;
pub fn retry(&self, project_id: u64, job_id: u64) -> Result<Job, GitLabError>;
pub fn play(&self, project_id: u64, job_id: u64) -> Result<Job, GitLabError>;
pub fn erase(&self, project_id: u64, job_id: u64) -> Result<Job, GitLabError>;
pub fn artifacts(&self, project_id: u64, job_id: u64) -> Result<Vec<u8>, GitLabError>;
```

### PipelineSchedulesResource

```rust
pub fn list(&self, project_id: u64) -> Result<Vec<PipelineSchedule>, GitLabError>;
pub fn get(&self, project_id: u64, schedule_id: u64) -> Result<PipelineSchedule, GitLabError>;
pub fn create(&self, project_id: u64, payload: &CreatePipelineSchedulePayload) -> Result<PipelineSchedule, GitLabError>;
pub fn update(&self, project_id: u64, schedule_id: u64, payload: &UpdatePipelineSchedulePayload) -> Result<PipelineSchedule, GitLabError>;
pub fn delete(&self, project_id: u64, schedule_id: u64) -> Result<(), GitLabError>;
pub fn take_ownership(&self, project_id: u64, schedule_id: u64) -> Result<PipelineSchedule, GitLabError>;
pub fn create_variable(&self, project_id: u64, schedule_id: u64, key: &str, value: &str) -> Result<PipelineScheduleVariable, GitLabError>;
pub fn update_variable(&self, project_id: u64, schedule_id: u64, variable_id: u64, value: &str) -> Result<PipelineScheduleVariable, GitLabError>;
pub fn delete_variable(&self, project_id: u64, schedule_id: u64, variable_id: u64) -> Result<(), GitLabError>;
```

### RunnersResource, ReleasesResource, DeployKeysResource, EnvironmentsResource

(consulte o [Guia de Uso](./usage-guide.md) para assinaturas completas)

---

## GitLabError

Enum de erros seguindo o formato **RFC 7807** (Problem Details).

```rust
pub enum GitLabError {
    Api {
        category: ErrorCategory,
        status: u16,
        detail: String,
        instance: String,    // UUID v7
        context: Box<ErrorContext>,
    },
    RateLimited {
        retry_after: Option<u64>,
        context: Box<ErrorContext>,
    },
    Timeout {
        duration: Duration,
        context: Box<ErrorContext>,
    },
    Http(reqwest::Error),
    Url(String),
    Serialization(serde_json::Error),
    Config(String),
}
```

### Métodos

| Método | Descrição |
|--------|-----------|
| `api(category, status, detail, context) -> GitLabError` | Construtor com UUID v7 automático |
| `category() -> Option<ErrorCategory>` | Extrai categoria do erro |

---

## ErrorCategory

```rust
pub enum ErrorCategory {
    AuthenticationFailed,
    AuthorizationDenied,
    ResourceNotFound,
    ValidationError,
    Conflict,
    RateLimited,
    SpamDetected,
    NotModified,
    Timeout,
    NetworkError,
    ParseError,
    InternalError,
}
```

### Mapeamento HTTP → Categoria

| HTTP | Categoria |
|------|-----------|
| 304 | `NotModified` |
| 400 | `SpamDetected` |
| 401 | `AuthenticationFailed` |
| 403 | `AuthorizationDenied` |
| 404 | `ResourceNotFound` |
| 409 | `Conflict` |
| 422 | `ValidationError` |
| 429 | `RateLimited` |
| 500 | `InternalError` / `ParseError` |
| 503 | `NetworkError` |
| 504 | `Timeout` |

---

## ErrorContext

```rust
pub struct ErrorContext {
    pub operation: Option<String>,
    pub http_status: Option<u16>,
    pub api_errors: Option<Vec<String>>,
    pub response_body: Option<String>,
}
```

---

## PaginationInfo

```rust
pub struct PaginationInfo {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub total: Option<u64>,
    pub total_pages: Option<u32>,
    pub next_page: Option<u32>,
    pub prev_page: Option<u32>,
    pub next_cursor: Option<String>,  // keyset pagination
}
```

---

## Módulo OAuth

Disponível em `gitlab_wrapper::oauth`.

### PKCE

```rust
pub fn generate_code_verifier() -> String;        // CSPRNG via getrandom
pub fn generate_code_challenge(verifier: &str) -> String;  // SHA-256 base64url
```

### Authorization Code

```rust
pub fn authorization_code_url(options: &AuthCodeUrlOptions) -> String;
pub fn exchange_authorization_code(options: &ExchangeCodeOptions) -> Result<OAuthTokenResponse, GitLabError>;
```

### Device Grant

```rust
pub fn request_device_authorization(options: &DeviceAuthOptions) -> Result<DeviceAuthResponse, GitLabError>;
pub fn poll_for_token(options: &PollTokenOptions) -> Result<OAuthTokenResponse, GitLabError>;
pub fn get_token(options: &GetTokenOptions) -> Result<OAuthTokenResponse, GitLabError>;
```

### Refresh / Revoke

```rust
pub fn refresh_token(options: &RefreshTokenOptions) -> Result<OAuthTokenResponse, GitLabError>;
pub fn revoke_token(options: &RevokeTokenOptions) -> Result<(), GitLabError>;
```

### Tipos OAuth

```rust
pub struct OAuthTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub refresh_token: Option<String>,
    pub scope: String,
    pub created_at: u64,
    pub expires_in: u64,
}

pub struct DeviceAuthResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: Option<String>,
    pub expires_in: u64,
    pub interval: u64,
}

pub struct OAuthErrorResponse {
    pub error: String,
    pub error_description: Option<String>,
}
```

---

## Notas Finais

- **Síncrono (blocking):** O wrapper usa `reqwest::blocking`. Threads são bloqueadas durante chamadas HTTP.
  Para async, use `tokio::task::spawn_blocking` ou aguarde uma versão async futura.
- **Tipos:** Todas as structs de domínio derivam `Debug, Clone, Serialize, Deserialize`.
- **Filtros:** Structs de filtro implementam `Default` para construção parcial com `..Default::default()`.
- **Re-export:** Todas as structs de tipo e resource são re-exportadas de `gitlab_wrapper`.
