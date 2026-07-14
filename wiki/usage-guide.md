# Guia de Uso — Todos os 93 Resources

Exemplos práticos de como usar cada um dos **93 resources** do `gitlab-wrapper-rs`.

> **Convenções:** Todos os métodos seguem `Result<T, GitLabError>`.
> Métodos de listagem retornam `Vec<T>` (coletados eager).
> Use `?` para propagar erros ou `match` para tratamento granular.

```rust
use gitlab_wrapper::*;
let gl = GitLabClient::new(GitLabConfig {
    base_url: "https://gitlab.com".into(),
    token: Some(std::env::var("GITLAB_TOKEN")?),
    auth_method: Some(AuthMethod::Bearer),
    ..Default::default()
})?;
```

---

## Índice

1. [Core: Projects, Groups, Users](#1-core-projects-groups-users)
2. [Repository: Branches, Commits, Tags, Files, Tree](#2-repository)
3. [Issues & Merge Requests](#3-issues--merge-requests)
4. [CI/CD: Pipelines, Jobs, Variables, Runners](#4-cicd)
5. [Security & Access Control](#5-security--access-control)
6. [Releases & Packages](#6-releases--packages)
7. [Wikis & Snippets](#7-wikis--snippets)
8. [Groups Features](#8-groups-features)
9. [System & Admin](#9-system--admin)
10. [Templates](#10-templates)
11. [Project Features](#11-project-features)
12. [Epics & Boards](#12-epics--boards)
13. [Vulnerabilities](#13-vulnerabilities)
14. [Other: Events, Todos, Search, Labels, etc](#14-other)

---

## 1. Core: Projects, Groups, Users

### Projects

```rust
// Listar projetos
let projects = gl.projects.list(None).await?;
let projects = gl.projects.list(Some(&ProjectFilter {
    membership: Some(true),
    per_page: Some(20),
    ..Default::default()
})).await?;

// Obter projeto
let p = gl.projects.get(42).await?;
let p = gl.projects.get_by_path("grupo/projeto").await?;

// Criar projeto
let p = gl.projects.create(&CreateProjectPayload {
    name: "meu-projeto".into(),
    description: Some("Descrição".into()),
    initialize_with_readme: Some(true),
    visibility: Some("private".into()),
    path: None, namespace_id: None, topics: None,
}).await?;

// Atualizar
gl.projects.update(42, &UpdateProjectPayload {
    name: Some("novo-nome".into()),
    description: None, visibility: None, topics: None, default_branch: None,
}).await?;

// Ações
gl.projects.archive(42).await?;
gl.projects.unarchive(42).await?;
gl.projects.star(42).await?;
gl.projects.unstar(42).await?;
gl.projects.fork(42, Some("novo-namespace")).await?;
gl.projects.transfer(42, 10).await?;
gl.projects.share(42, 5, 30).await?;  // group_id=5, access=DEVELOPER
gl.projects.unshare(42, 5).await?;

// Idiomas
let langs = gl.projects.languages(42).await?;

// Listar forks
let forks = gl.projects.list_forks(42, None).await?;

// Upload
let data = std::fs::read("foto.png")?;
gl.projects.upload_file(42, "foto.png", data).await?;
gl.projects.upload_avatar(42, "avatar.png", data).await?;

// Excluir
gl.projects.delete(42).await?;
```

### Groups

```rust
// Listar
let groups = gl.groups.list(None).await?;
let groups = gl.groups.list(Some(&GroupFilter {
    search: Some("my-group".into()),
    ..Default::default()
})).await?;

// CRUD
let g = gl.groups.get(10).await?;
let g = gl.groups.get_by_path("grupo-pai/subgrupo").await?;
let g = gl.groups.create(&CreateGroupPayload {
    name: "novo-grupo".into(),
    path: "novo-grupo".into(),
    description: Some("Grupo de teste".into()),
    visibility: Some("private".into()),
    parent_id: None,
}).await?;
gl.groups.update(10, &UpdateGroupPayload {
    name: Some("nome-atualizado".into()),
    description: None, visibility: None,
}).await?;
gl.groups.delete(10).await?;

// Hierarquia
let subs = gl.groups.subgroups(10).await?;
let desc = gl.groups.descendant_groups(10).await?;

// Projetos do grupo
let projs = gl.groups.projects(10).await?;
let shared = gl.groups.shared_projects(10).await?;

// Membros SAML / provisionados
let saml = gl.groups.saml_users(10).await?;
let prov = gl.groups.provisioned_users(10).await?;
```

### Users

```rust
// Listar/buscar
let users = gl.users.list(Some(&UserFilter {
    search: Some("joao".into()),
    ..Default::default()
})).await?;

// CRUD
let me = gl.users.get_current().await?;
let u = gl.users.get(me.id).await?;
gl.users.create(&CreateUserPayload {
    email: "novo@user.com".into(),
    username: "novouser".into(),
    name: "Novo Usuário".into(),
    password: "senha123".into(),
    skip_confirmation: Some(true),
}).await?;
gl.users.update(me.id, &UpdateUserPayload {
    name: Some("Nome Atualizado".into()),
    email: None, username: None, password: None,
}).await?;
gl.users.delete(me.id).await?;

// Status
let status = gl.users.status(me.id).await?;
gl.users.set_status(Some("rocket"), Some("Em reunião")).await?;

// Moderação
gl.users.deactivate(me.id).await?;
gl.users.activate(me.id).await?;
gl.users.ban(me.id).await?;
gl.users.unban(me.id).await?;

// Preferências
let prefs = gl.users.preferences().await?;
gl.users.set_preferences(&serde_json::json!({
    "view_diffs_file_by_file": true
})).await?;
```

---

## 2. Repository

### Branches

```rust
let branches = gl.branches.list(42).await?;
let branch = gl.branches.get(42, "main").await?;
gl.branches.create(42, &CreateBranchPayload {
    branch: "feature-x".into(),
    ref_: "main".into(),
}).await?;
gl.branches.delete(42, "feature-x").await?;
gl.branches.delete_merged(42).await?;
```

### Commits

```rust
let commits = gl.commits.list(42, None).await?;
let commits = gl.commits.list(42, Some(&CommitFilter {
    ref_name: Some("main".into()),
    since: Some("2024-01-01T00:00:00Z".into()),
    ..Default::default()
})).await?;

let c = gl.commits.get(42, "abc123def").await?;
let diff = gl.commits.diff(42, "abc123def").await?;
let refs = gl.commits.refs(42, "abc123def").await?;
let comments = gl.commits.comments(42, "abc123def").await?;

gl.commits.cherry_pick(42, "abc123def", "release-1.0").await?;
gl.commits.revert(42, "abc123def", "main").await?;
gl.commits.add_comment(42, "abc123def", "Ótimo commit!").await?;

// Status / signature
let statuses = gl.commits.statuses(42, "abc123def").await?;
let sig = gl.commits.signature(42, "abc123def").await?;
let mrs = gl.commits.merge_requests(42, "abc123def").await?;
```

### Tags

```rust
let tags = gl.tags.list(42).await?;
let t = gl.tags.get(42, "v1.0").await?;
gl.tags.create(42, &CreateTagPayload {
    tag_name: "v2.0".into(),
    ref_: "main".into(),
    message: Some("Release 2.0".into()),
    release_description: Some("Notas de release".into()),
}).await?;
gl.tags.delete(42, "v2.0").await?;
let sig = gl.tags.signature(42, "v1.0").await?;
```

### Repository Files

```rust
let file = gl.repository_files.get(42, "README.md", "main").await?;
let raw = gl.repository_files.raw(42, "README.md", "main").await?;
let blame = gl.repository_files.blame(42, "README.md", "main").await?;

gl.repository_files.create(42, "docs/guia.md", &CreateFilePayload {
    branch: "main".into(),
    content: "# Guia\nConteúdo".into(),
    commit_message: "Adiciona guia".into(),
    encoding: None, author_email: None, author_name: None,
}).await?;

gl.repository_files.update(42, "docs/guia.md", &UpdateFilePayload {
    branch: "main".into(),
    content: "# Guia Atualizado".into(),
    commit_message: "Atualiza guia".into(),
    encoding: None, author_email: None, author_name: None,
    last_commit_id: None,
}).await?;

gl.repository_files.delete(42, "docs/antigo.md", "main", "Remove arquivo antigo").await?;
```

### Repository Tree

```rust
let tree = gl.repository_tree.list(42, None).await?;
let tree = gl.repository_tree.list(42, Some(&TreeFilter {
    path: Some("src".into()),
    ref_: Some("main".into()),
    recursive: Some(true),
    ..Default::default()
})).await?;

let item = gl.repository_tree.get(42, "src/main.rs", "main").await?;
```

---

## 3. Issues & Merge Requests

### Issues

```rust
// Listar
let issues = gl.issues.list(Some(&IssueFilter {
    state: Some("opened".into()),
    labels: Some("bug".into()),
    ..Default::default()
})).await?;
let issues = gl.issues.list_for_project(42, None).await?;
let issues = gl.issues.get_by_group(10, None).await?;

// CRUD
let i = gl.issues.get(42, 5).await?;
let i = gl.issues.create(42, &CreateIssuePayload {
    title: "Bug crítico".into(),
    description: Some("Passos para reproduzir...".into()),
    labels: Some("bug,prioridade-alta".into()),
    assignee_ids: Some(vec![1, 2]),
    weight: Some(3),
    due_date: None, confidential: None, milestone_id: None,
}).await?;
gl.issues.update(42, 5, &UpdateIssuePayload {
    title: Some("Título atualizado".into()),
    state_event: Some("close".into()),
    ..Default::default()
}).await?;
gl.issues.delete(42, 5).await?;

// Inscrição
gl.issues.subscribe(42, 5).await?;
gl.issues.unsubscribe(42, 5).await?;
let sub = gl.issues.subscription(42, 5).await?;

// Tempo
gl.issues.set_time_estimate(42, 5, "3h30m").await?;
gl.issues.add_spent_time(42, 5, "1h").await?;
gl.issues.reset_time_estimate(42, 5).await?;
gl.issues.reset_spent_time(42, 5).await?;

// Ações
gl.issues.move_issue(42, 5, 100).await?;
gl.issues.reorder(42, 5, Some(3), None).await?;

// Informações adicionais
let closed = gl.issues.closed_by(42, 5).await?;
let parts = gl.issues.participants(42, 5).await?;
let rel_mrs = gl.issues.related_merge_requests(42, 5).await?;
```

### Merge Requests

```rust
// Listar
let mrs = gl.merge_requests.list(None).await?;
let mrs = gl.merge_requests.list_for_project(42, None).await?;
let mrs = gl.merge_requests.list_by_group(10, None).await?;

// CRUD
let mr = gl.merge_requests.get(42, 5).await?;
let mr = gl.merge_requests.create(42, &CreateMergeRequestPayload {
    source_branch: "feature-x".into(),
    target_branch: "main".into(),
    title: "MR: Feature X".into(),
    description: Some("Descrição do MR".into()),
    assignee_ids: None, reviewer_ids: None,
    milestone_id: None, labels: None,
    remove_source_branch: None, squash: None, draft: None,
}).await?;
gl.merge_requests.update(42, 5, &UpdateMergeRequestPayload {
    title: Some("Novo título".into()),
    state_event: Some("merge".into()),
    ..Default::default()
}).await?;
gl.merge_requests.delete(42, 5).await?;

// Merge / aprovação
gl.merge_requests.merge(42, 5, None).await?;
gl.merge_requests.cancel_merge_when_pipeline_succeeds(42, 5).await?;
gl.merge_requests.approve(42, 5).await?;
gl.merge_requests.unapprove(42, 5).await?;
gl.merge_requests.rebase(42, 5).await?;

// Dados do MR
let commits = gl.merge_requests.commits(42, 5).await?;
let changes = gl.merge_requests.changes(42, 5).await?;
let pipes = gl.merge_requests.pipelines(42, 5).await?;
gl.merge_requests.create_pipeline(42, 5).await?;
let parts = gl.merge_requests.participants(42, 5).await?;

// Inscrição
gl.merge_requests.subscribe(42, 5).await?;
gl.merge_requests.unsubscribe(42, 5).await?;
gl.merge_requests.subscription(42, 5).await?;

// Tempo
gl.merge_requests.set_time_estimate(42, 5, "2d").await?;
gl.merge_requests.add_spent_time(42, 5, "4h").await?;
gl.merge_requests.reset_time_estimate(42, 5).await?;
gl.merge_requests.reset_spent_time(42, 5).await?;
```

### Notes (Comentários)

```rust
// Issue notes
let notes = gl.notes.list_issue_notes(42, 5).await?;
let n = gl.notes.create_issue_note(42, 5, &CreateNotePayload {
    body: "Comentário na issue".into(),
    confidential: None,
}).await?;
gl.notes.update_issue_note(42, 5, 123, &UpdateNotePayload {
    body: "Comentário editado".into(),
    confidential: None,
}).await?;
gl.notes.delete_issue_note(42, 5, 123).await?;

// MR notes — mesmo padrão: create_mr_note, list_mr_notes, etc.
// Commit notes — create_commit_note, list_commit_notes, etc.
// Snippet notes — create_snippet_note, list_snippet_notes, etc.
// Wiki notes — create_wiki_note, list_wiki_notes, etc.
```

### Discussions

```rust
// Issue discussions
let discs = gl.discussions.list_issue_discussions(42, 5).await?;
let d = gl.discussions.create_issue_discussion(42, 5, &CreateDiscussionPayload {
    body: "Iniciando discussão".into(),
}).await?;
gl.discussions.add_issue_discussion_note(42, 5, "disc-id", &CreateNotePayload {
    body: "Resposta".into(),
    confidential: None,
}).await?;
gl.discussions.resolve_issue_discussion(42, 5, "disc-id", true).await?;

// MR discussions — list_mr_discussions, create_mr_discussion, etc.
// Commit discussions — list_commit_discussions, create_commit_discussion, etc.
```

### Draft Notes

```rust
let drafts = gl.draft_notes.list(42, 5).await?;
gl.draft_notes.create(42, 5, &CreateDraftNotePayload {
    note: "Revisão pendente...".into(),
    resolve_discussion: None,
    position: None,
}).await?;
gl.draft_notes.update(42, 5, 1, &UpdateDraftNotePayload {
    note: Some("Revisão atualizada".into()),
    resolve_discussion: None,
    position: None,
}).await?;
gl.draft_notes.delete(42, 5, 1).await?;
gl.draft_notes.publish(42, 5).await?;
```

### Emoji Reactions (Award Emoji)

```rust
// Issue emoji
let emojis = gl.emoji.list_issue_emoji(42, 5).await?;
gl.emoji.create_issue_emoji(42, 5, &CreateEmojiPayload {
    name: "thumbsup".into(),
}).await?;
gl.emoji.delete_issue_emoji(42, 5, 1).await?;

// MR emoji — list_mr_emoji, create_mr_emoji, delete_mr_emoji
// Snippet emoji — list_snippet_emoji, create_snippet_emoji, delete_snippet_emoji
```

### Resource Events

```rust
let state_events = gl.resource_events.list_issue_state_events(42, 5).await?;
let label_events = gl.resource_events.list_issue_label_events(42, 5).await?;
let ms_events = gl.resource_events.list_issue_milestone_events(42, 5).await?;
let w_events = gl.resource_events.list_issue_weight_events(42, 5).await?;
let it_events = gl.resource_events.list_issue_iteration_events(42, 5).await?;

// MR events também disponíveis:
let mr_state = gl.resource_events.list_mr_state_events(42, 5).await?;
let mr_label = gl.resource_events.list_mr_label_events(42, 5).await?;
```

### Issue Links

```rust
let links = gl.issue_links.list(42, 5).await?;
gl.issue_links.create(42, 5, &CreateIssueLinkPayload {
    target_project_id: None,
    target_issue_iid: 10,
    link_type: Some("blocks".into()),
}).await?;
gl.issue_links.delete(42, 5, 1).await?;
```

---

## 4. CI/CD

### Pipelines

```rust
let pipelines = gl.pipelines.list(42, None).await?;
let p = gl.pipelines.get(42, 100).await?;
let p = gl.pipelines.get_latest(42).await?;

gl.pipelines.create(42, &CreatePipelinePayload {
    ref_: "main".into(),
    variables: None,
}).await?;

gl.pipelines.retry(42, 100).await?;
gl.pipelines.cancel(42, 100).await?;
gl.pipelines.delete(42, 100).await?;

let vars = gl.pipelines.variables(42, 100).await?;
let report = gl.pipelines.test_report(42, 100).await?;
let summary = gl.pipelines.test_report_summary(42, 100).await?;
```

### Jobs

```rust
let jobs = gl.jobs.list(42, None).await?;
let jobs = gl.jobs.list_by_pipeline(42, 100, None).await?;
let j = gl.jobs.get(42, 500).await?;
let trace = gl.jobs.trace(42, 500).await?;

gl.jobs.cancel(42, 500).await?;
gl.jobs.retry(42, 500).await?;
gl.jobs.play(42, 500).await?;
gl.jobs.erase(42, 500).await?;

let artifacts = gl.jobs.artifacts(42, 500).await?;
```

### Job Artifacts

```rust
gl.job_artifacts.keep(42, 500).await?;
gl.job_artifacts.delete(42, 500).await?;
gl.job_artifacts.delete_all(42).await?;
gl.job_artifacts.download_by_ref(42, "main", "test-job").await?;
```

### Pipeline Schedules

```rust
let schedules = gl.pipeline_schedules.list(42).await?;
let s = gl.pipeline_schedules.get(42, 10).await?;

gl.pipeline_schedules.create(42, &CreatePipelineSchedulePayload {
    description: "Schedule diário".into(),
    ref_: "main".into(),
    cron: "0 6 * * *".into(),
    cron_timezone: Some("America/Sao_Paulo".into()),
    active: Some(true),
}).await?;

gl.pipeline_schedules.update(42, 10, &UpdatePipelineSchedulePayload {
    description: Some("Schedule atualizado".into()),
    cron: Some("0 8 * * *".into()),
    active: Some(true),
    ..Default::default()
}).await?;

gl.pipeline_schedules.delete(42, 10).await?;
gl.pipeline_schedules.take_ownership(42, 10).await?;

// Variáveis do schedule
gl.pipeline_schedules.create_variable(42, 10, "TOKEN", "secret").await?;
gl.pipeline_schedules.update_variable(42, 10, "TOKEN", "novo-secret").await?;
gl.pipeline_schedules.delete_variable(42, 10, "TOKEN").await?;
```

### Pipeline Triggers

```rust
let triggers = gl.pipeline_triggers.list(42, None).await?;
let t = gl.pipeline_triggers.get(42, 5).await?;
gl.pipeline_triggers.create(42, &CreatePipelineTriggerPayload {
    description: "Trigger CI/CD".into(),
    token: None,
}).await?;
gl.pipeline_triggers.update(42, 5, &UpdatePipelineTriggerPayload {
    description: Some("Trigger atualizado".into()),
    token: None,
}).await?;
gl.pipeline_triggers.delete(42, 5).await?;
gl.pipeline_triggers.take_ownership(42, 5).await?;
```

### CI Lint

```rust
let result = gl.ci_lint.validate(42, &CiLintPayload {
    content: r#"
job:
  script: echo "Hello"
  rules:
    - if: $CI_MERGE_REQUEST_ID
"#.into(),
    include_merged_yaml: Some(true),
}).await?;
println!("Status: {:?}, Erros: {:?}", result.status, result.errors);
```

### CI/CD Variables (Projeto)

```rust
let vars = gl.variables.list(42, None).await?;
let v = gl.variables.get(42, "MY_KEY").await?;

gl.variables.create(42, &CreateCiVariablePayload {
    key: "DEPLOY_TOKEN".into(),
    value: "secret123".into(),
    variable_type: Some("env_var".into()),
    protected: Some(true),
    masked: Some(true),
    environment_scope: Some("production".into()),
    ..Default::default()
}).await?;

gl.variables.update(42, "DEPLOY_TOKEN", &UpdateCiVariablePayload {
    value: Some("novo-secret".into()),
    ..Default::default()
}).await?;

gl.variables.delete(42, "OLD_KEY").await?;
```

### Runners

```rust
let runners = gl.runners.list().await?;
let r = gl.runners.get(10).await?;

gl.runners.create(&CreateRunnerPayload {
    runner_type: "instance_type".into(),
    description: Some("Meu runner".into()),
    tag_list: Some(vec!["docker".into(), "linux".into()]),
    run_untagged: Some(false),
    locked: Some(true),
    access_level: Some("ref_protected".into()),
    maximum_timeout: Some(3600),
}).await?;

gl.runners.update(10, &UpdateRunnerPayload {
    description: Some("Runner atualizado".into()),
    active: Some(true),
    ..Default::default()
}).await?;

gl.runners.delete(10).await?;
let jobs = gl.runners.list_jobs(10).await?;
```

### Environments

```rust
let envs = gl.environments.list(42).await?;
let e = gl.environments.get(42, 5).await?;

gl.environments.create(42, &CreateEnvironmentPayload {
    name: "staging".into(),
    external_url: Some("https://staging.example.com".into()),
    slug: None,
    tier: Some("staging".into()),
}).await?;

gl.environments.update(42, 5, &UpdateEnvironmentPayload {
    name: Some("staging-v2".into()),
    external_url: Some("https://staging-v2.example.com".into()),
    ..Default::default()
}).await?;

gl.environments.delete(42, 5).await?;
gl.environments.stop(42, 5).await?;
```

### Deployments

```rust
let deploys = gl.deployments.list(42, None).await?;
let deploys = gl.deployments.list(42, Some(&DeploymentFilter {
    status: Some("success".into()),
    environment: Some("production".into()),
    ..Default::default()
})).await?;
let d = gl.deployments.get(42, 10).await?;
gl.deployments.approve(42, 10).await?;
```

### Merge Trains

```rust
let trains = gl.merge_trains.list(42, None).await?;
let trains = gl.merge_trains.list(42, Some(&MergeTrainFilter {
    scope: Some("active".into()),
    ..Default::default()
})).await?;
let t = gl.merge_trains.get(42, 5).await?;
```

---

## 5. Security & Access Control

### Members

```rust
// Projeto
let members = gl.members.list_project_members(42).await?;
let all = gl.members.list_project_inherited_members(42).await?;
let m = gl.members.get_project_member(42, 1).await?;
gl.members.add_project_member(42, &AddMemberPayload {
    user_id: 5,
    access_level: 30, // DEVELOPER
    expires_at: None,
}).await?;
gl.members.update_project_member(42, 5, &UpdateMemberPayload {
    access_level: 40, // MAINTAINER
    expires_at: None,
}).await?;
gl.members.delete_project_member(42, 5).await?;

// Grupo — mesmo padrão: list_group_members, add_group_member, etc.
```

### Access Requests

```rust
let requests = gl.access_requests.list(42, None).await?;
gl.access_requests.request(42).await?;
gl.access_requests.approve(42, 5).await?;
gl.access_requests.deny(42, 5).await?;
```

### Access Tokens

```rust
// Projeto
let tokens = gl.access_tokens.list_project_tokens(42).await?;
let t = gl.access_tokens.get_project_token(42, 1).await?;
gl.access_tokens.create_project_token(42, &CreateAccessTokenPayload {
    name: "CI Token".into(),
    scopes: Some(vec!["api".into(), "read_repository".into()]),
    expires_at: Some("2025-12-31".into()),
    access_level: None,
}).await?;
gl.access_tokens.revoke_project_token(42, 1).await?;

// Grupo — list_group_tokens, create_group_token, etc.
```

### Personal Access Tokens

```rust
let tokens = gl.personal_access_tokens.list(None).await?;
let tokens = gl.personal_access_tokens.list(Some(&PersonalAccessTokenFilter {
    state: Some("active".into()),
    ..Default::default()
})).await?;
let t = gl.personal_access_tokens.get(5).await?;
gl.personal_access_tokens.revoke(5).await?;
```

### Deploy Keys

```rust
let keys = gl.deploy_keys.list(42).await?;
let k = gl.deploy_keys.get(42, 1).await?;
gl.deploy_keys.create(42, &CreateDeployKeyPayload {
    title: "Server Key".into(),
    key: "ssh-rsa AAA...".into(),
    can_push: Some(false),
}).await?;
gl.deploy_keys.update(42, 1, &UpdateDeployKeyPayload {
    title: Some("Server Key (atualizada)".into()),
    can_push: Some(true),
}).await?;
gl.deploy_keys.delete(42, 1).await?;
gl.deploy_keys.enable(42, 1).await?;
```

### Deploy Tokens

```rust
// Projeto
let tokens = gl.deploy_tokens.list_project_tokens(42).await?;
gl.deploy_tokens.create_project_token(42, &CreateDeployTokenPayload {
    name: "Deploy Token".into(),
    scopes: Some(vec!["read_repository".into()]),
    expires_at: None,
    username: None,
}).await?;
gl.deploy_tokens.revoke_project_token(42, 1).await?;

// Grupo — list_group_tokens, create_group_token, revoke_group_token
// Todos — list_all()
let all = gl.deploy_tokens.list_all().await?;
```

### Keys (SSH por fingerprint)

```rust
let key = gl.keys.get_by_fingerprint("SHA256:xxxxxxxx").await?;
```

### Protected Branches

```rust
let branches = gl.protected_branches.list(42, None).await?;
let b = gl.protected_branches.get(42, "main").await?;
gl.protected_branches.protect(42, &ProtectBranchPayload {
    name: "main".into(),
    push_access_level: Some(40),  // MAINTAINER
    merge_access_level: Some(40),
    allow_force_push: None,
    code_owner_approval_required: None,
    user_id: None,
    group_id: None,
}).await?;
gl.protected_branches.unprotect(42, "main").await?;
```

### Protected Tags

```rust
let tags = gl.protected_tags.list(42, None).await?;
let t = gl.protected_tags.get(42, "v*").await?;
gl.protected_tags.protect(42, &ProtectTagPayload {
    name: "v*".into(),
    create_access_level: Some(40),
    user_id: None,
    group_id: None,
}).await?;
gl.protected_tags.unprotect(42, "v*").await?;
```

### Protected Environments

```rust
let penvs = gl.protected_environments.list(42, None).await?;
let pe = gl.protected_environments.get(42, "production").await?;
gl.protected_environments.protect(42, &ProtectEnvironmentPayload {
    name: "production".into(),
    deploy_access_levels: vec![
        serde_json::json!({ "access_level": 40 }) // MAINTAINER
    ],
    required_approval_count: None,
}).await?;
gl.protected_environments.unprotect(42, "production").await?;
```

---

## 6. Releases & Packages

### Releases

```rust
let releases = gl.releases.list(42).await?;
let r = gl.releases.get(42, "v1.0").await?;

gl.releases.create(42, &CreateReleasePayload {
    tag_name: "v1.0".into(),
    description: "Release inicial".into(),
    name: Some("Versão 1.0".into()),
    ref_: None,
    released_at: None,
    milestones: None,
}).await?;

gl.releases.update(42, "v1.0", &UpdateReleasePayload {
    description: Some("Descrição atualizada".into()),
    name: None,
    released_at: None,
    milestones: None,
}).await?;

gl.releases.delete(42, "v1.0").await?;

// Release links
let links = gl.releases.list_links(42, "v1.0").await?;
let link = gl.releases.get_link(42, "v1.0", 1).await?;
gl.releases.create_link(42, "v1.0", &CreateReleaseLinkPayload {
    name: "Download".into(),
    url: "https://releases.example.com/pkg.tar.gz".into(),
    filepath: None,
    link_type: None,
}).await?;
gl.releases.update_link(42, "v1.0", 1, Some("Novo link"), Some("https://...")).await?;
gl.releases.delete_link(42, "v1.0", 1).await?;

// Latest release
let latest = gl.releases.get_latest(42).await?;
let asset = gl.releases.download_asset(42, "v1.0", "bin/app").await?;
```

### Packages

```rust
let packages = gl.packages.list(42).await?;
let pkg = gl.packages.get(42, 1).await?;
gl.packages.delete(42, 1).await?;
let files = gl.packages.list_files(42, 1).await?;
```

### Package Types (por tipo de linguagem)

```rust
// Maven
let versions = gl.package_maven.list_versions(42, None).await?;

// NPM
let versions = gl.package_npm.list_versions(42, None).await?;

// PyPI
let versions = gl.package_pypi.list_versions(42, None).await?;

// NuGet
let versions = gl.package_nuget.list_versions(42, None).await?;

// RubyGems
let versions = gl.package_rubygems.list_versions(42, None).await?;

// Composer
let versions = gl.package_composer.list_versions(42, None).await?;

// Outros: package_conan, package_go_proxy, package_helm,
// package_debian, package_terraform_modules
```

### Container Registry

```rust
let repos = gl.container_registry.list_repositories(42).await?;
gl.container_registry.delete_repository(42, 1).await?;
let tags = gl.container_registry.list_tags(42, 1).await?;
let tag = gl.container_registry.get_tag(42, 1, "latest").await?;
gl.container_registry.delete_tag(42, 1, "old-tag").await?;
```

---

## 7. Wikis & Snippets

### Wikis (Projeto)

```rust
let pages = gl.wikis.list(42).await?;
let page = gl.wikis.get(42, "home").await?;

gl.wikis.create(42, &CreateWikiPagePayload {
    title: "Guia do Projeto".into(),
    content: "# Bem-vindo\nEsta é a wiki do projeto.".into(),
    format: Some("markdown".into()),
}).await?;

gl.wikis.update(42, "guia-do-projeto", &UpdateWikiPagePayload {
    title: Some("Guia do Projeto".into()),
    content: Some("# Atualizado\nNovo conteúdo.".into()),
    format: None,
}).await?;

gl.wikis.delete(42, "pagina-antiga").await?;

// Upload de anexo
let img = std::fs::read("diagrama.png")?;
let att = gl.wikis.upload_attachment(42, "diagrama.png", img).await?;
```

### Group Wikis

```rust
let pages = gl.group_wikis.list(10, None).await?;
let page = gl.group_wikis.get(10, "home").await?;

gl.group_wikis.create(10, &CreateWikiPagePayload {
    title: "Wiki do Grupo".into(),
    content: "# Wiki do Grupo".into(),
    format: None,
}).await?;

gl.group_wikis.update(10, "wiki-do-grupo", &UpdateWikiPagePayload {
    content: Some("# Atualizado".into()),
    title: None, format: None,
}).await?;

gl.group_wikis.delete(10, "pagina-antiga").await?;

let img = std::fs::read("logo.png")?;
gl.group_wikis.upload_attachment(10, "logo.png", img).await?;
```

### Snippets

```rust
let snippets = gl.snippets.list(42, None).await?;
let s = gl.snippets.get(42, 1).await?;

gl.snippets.create(42, &CreateSnippetPayload {
    title: "Script de deploy".into(),
    file_name: "deploy.sh".into(),
    content: "#!/bin/bash\necho 'Deploying...'".into(),
    visibility: Some("private".into()),
    description: Some("Script para deploy automático".into()),
}).await?;

gl.snippets.update(42, 1, &UpdateSnippetPayload {
    title: Some("Script atualizado".into()),
    content: Some("#!/bin/bash\necho 'New deploy'".into()),
    visibility: None, file_name: None, description: None,
}).await?;

gl.snippets.delete(42, 1).await?;
let raw = gl.snippets.raw(42, 1).await?;
```

---

## 8. Groups Features

### Group Variables

```rust
let vars = gl.group_variables.list(10, None).await?;
let v = gl.group_variables.get(10, "MY_KEY").await?;
gl.group_variables.create(10, &CreateGroupVariablePayload {
    key: "GROUP_SECRET".into(),
    value: "secret123".into(),
    variable_type: None, protected: None,
    masked: None, environment_scope: None, description: None,
}).await?;
gl.group_variables.update(10, "GROUP_SECRET", &UpdateGroupVariablePayload {
    value: Some("new-secret".into()),
    ..Default::default()
}).await?;
gl.group_variables.delete(10, "OLD_KEY").await?;
```

### Group Webhooks

```rust
let hooks = gl.group_webhooks.list(10, None).await?;
gl.group_webhooks.create(10, &CreateGroupHookPayload {
    url: "https://hooks.example.com/gitlab".into(),
    push_events: Some(true),
    merge_requests_events: Some(true),
    ..Default::default()
}).await?;
gl.group_webhooks.update(10, 1, &UpdateGroupHookPayload {
    url: Some("https://hooks.example.com/v2".into()),
    ..Default::default()
}).await?;
gl.group_webhooks.delete(10, 1).await?;
```

### Group Push Rules

```rust
let rule = gl.group_push_rules.get(10).await?;
gl.group_push_rules.create(10, &CreateGroupPushRulePayload {
    deny_delete_tag: Some(true),
    commit_message_regex: Some("JIRA-\\d+".into()),
    ..Default::default()
}).await?;
gl.group_push_rules.update(10, &UpdateGroupPushRulePayload {
    deny_delete_tag: Some(false),
    ..Default::default()
}).await?;
gl.group_push_rules.delete(10).await?;
```

### Group Iterations

```rust
let iterations = gl.group_iterations.list(10, None).await?;
```

---

## 9. System & Admin

### Application Settings

```rust
let settings = gl.settings.get().await?;
gl.settings.update(&serde_json::json!({
    "default_project_visibility": "private"
})).await?;
```

### License

```rust
let license = gl.license.get().await?;
gl.license.create(&CreateLicensePayload {
    license: "LICENSE_KEY_STRING".into(),
}).await?;
gl.license.delete().await?;
```

### Audit Events

```rust
let events = gl.audit_events.list().await?;
let event = gl.audit_events.get(5).await?;
```

### Broadcast Messages

```rust
let msgs = gl.broadcast_messages.list().await?;
let msg = gl.broadcast_messages.get(1).await?;
gl.broadcast_messages.create(&CreateBroadcastMessagePayload {
    message: "Manutenção programada para sexta-feira".into(),
    starts_at: None,
    ends_at: None,
    color: None,
    font: None,
    target_access_levels: None,
    broadcast_type: None,
    dismissable: None,
}).await?;
gl.broadcast_messages.update(1, &UpdateBroadcastMessagePayload {
    message: Some("Mensagem atualizada".into()),
    ..Default::default()
}).await?;
gl.broadcast_messages.delete(1).await?;
```

### System Hooks

```rust
let hooks = gl.system_hooks.list().await?;
let h = gl.system_hooks.get(1).await?;
gl.system_hooks.create(&CreateSystemHookPayload {
    url: "https://hooks.example.com/system".into(),
    token: None,
    push_events: None,
    tag_push_events: None,
    merge_requests_events: None,
    enable_ssl_verification: None,
}).await?;
gl.system_hooks.delete(1).await?;
```

### Notification Settings

```rust
// Global
let ns = gl.notification_settings.get_global().await?;
gl.notification_settings.update_global(&UpdateNotificationPayload {
    level: Some("participating".into()),
    ..Default::default()
}).await?;

// Projeto
let ns = gl.notification_settings.get_project(42).await?;
gl.notification_settings.update_project(42, &UpdateNotificationPayload {
    level: Some("watch".into()),
    ..Default::default()
}).await?;

// Grupo
let ns = gl.notification_settings.get_group(10).await?;
gl.notification_settings.update_group(10, &UpdateNotificationPayload {
    level: Some("disabled".into()),
    ..Default::default()
}).await?;
```

### Custom Attributes

```rust
// Projeto
let attrs = gl.custom_attributes.list_project(42).await?;
let attr = gl.custom_attributes.get_project(42, "chave").await?;
gl.custom_attributes.set_project(42, "chave", &SetCustomAttributePayload {
    value: "valor".into(),
}).await?;
gl.custom_attributes.delete_project(42, "chave").await?;

// Grupo — list_group, get_group, set_group, delete_group
// Usuário — list_user, get_user, set_user, delete_user
```

### Invitations

```rust
// Projeto
let invites = gl.invitations.list_project(42, None).await?;
gl.invitations.create_project(42, &CreateInvitationPayload {
    email: "novo@membro.com".into(),
    access_level: 30, // DEVELOPER
    expires_at: None,
}).await?;
gl.invitations.delete_project(42, "novo@membro.com").await?;

// Grupo — list_group, create_group, delete_group
```

### Namespaces

```rust
let namespaces = gl.namespaces.list(None).await?;
let ns = gl.namespaces.get(10).await?;
```

### Markdown

```rust
let result = gl.markdown.render(&MarkdownPayload {
    text: "# Título\n\n**Negrito** e *itálico*".into(),
    gfm: Some(true),
    project: Some("grupo/projeto".into()),
}).await?;
println!("HTML: {}", result.html);
```

### Topics

```rust
let topics = gl.topics.list(None).await?;
let t = gl.topics.get(1).await?;
gl.topics.create(&CreateTopicPayload {
    name: "rust".into(),
    title: Some("Rust".into()),
    description: Some("Linguagem de programação Rust".into()),
    avatar: None,
}).await?;
gl.topics.update(1, &UpdateTopicPayload {
    name: Some("rust-lang".into()),
    title: Some("Rust Programming Language".into()),
    ..Default::default()
}).await?;
gl.topics.delete(1).await?;
```

---

## 10. Templates

### Dockerfile Templates

```rust
let templates = gl.dockerfile_templates.list(None).await?;
let t = gl.dockerfile_templates.get("Rust").await?;
println!("Template: {}\n{}", t.name, t.content.unwrap_or_default());
```

### Gitignore Templates

```rust
let templates = gl.gitignore_templates.list(None).await?;
let t = gl.gitignore_templates.get("Rust").await?;
```

### CI YAML Templates

```rust
let templates = gl.ci_yml_templates.list(None).await?;
let t = gl.ci_yml_templates.get("Node").await?;
```

### License Templates

```rust
let templates = gl.license_templates.list(None).await?;
let t = gl.license_templates.get("mit").await?;
```

### Project Templates

```rust
// Listar templates disponíveis para um projeto
let dockerfiles = gl.project_templates.list(42, "dockerfiles").await?;
let gitignores = gl.project_templates.list(42, "gitignores").await?;
let ci_ymls = gl.project_templates.list(42, "gitlab_ci_ymls").await?;
let licenses = gl.project_templates.list(42, "licenses").await?;

// Obter template específico
let t = gl.project_templates.get(42, "dockerfiles", "Node").await?;
```

---

## 11. Project Features

### Feature Flags

```rust
let flags = gl.feature_flags.list(42, None).await?;
let f = gl.feature_flags.get(42, "new-feature").await?;
gl.feature_flags.create(42, &CreateFeatureFlagPayload {
    name: "new-checkout".into(),
    version: Some("new_version_flag".into()),
    active: Some(true),
    strategies: None,
}).await?;
gl.feature_flags.update(42, "new-checkout", &UpdateFeatureFlagPayload {
    active: Some(false),
    ..Default::default()
}).await?;
gl.feature_flags.delete(42, "old-flag").await?;
```

### Freeze Periods

```rust
let periods = gl.freeze_periods.list(42).await?;
gl.freeze_periods.create(42, &CreateFreezePeriodPayload {
    freeze_start: "0 22 * * 5".into(),  // sexta 22h
    freeze_end: "0 6 * * 1".into(),     // segunda 6h
    cron_timezone: Some("America/Sao_Paulo".into()),
}).await?;
gl.freeze_periods.update(42, 1, &UpdateFreezePeriodPayload {
    freeze_start: Some("0 20 * * 5".into()),
    freeze_end: Some("0 8 * * 1".into()),
    cron_timezone: None,
}).await?;
gl.freeze_periods.delete(42, 1).await?;
```

### Remote Mirrors

```rust
let mirrors = gl.remote_mirrors.list(42).await?;
gl.remote_mirrors.create(42, &CreateRemoteMirrorPayload {
    url: "https://github.com/user/repo.git".into(),
    enabled: Some(true),
    only_protected_branches: Some(true),
    keep_divergent_refs: None,
}).await?;
gl.remote_mirrors.update(42, 1, &UpdateRemoteMirrorPayload {
    enabled: Some(false),
    ..Default::default()
}).await?;
```

### Import/Export

```rust
gl.import_export.schedule_export(42).await?;
let status = gl.import_export.export_status(42).await?;
let download = gl.import_export.download_export(42).await?;

// Importar projeto
let imported = gl.import_export.import(&ImportPayload {
    name: Some("projeto-importado".into()),
    path: None,
    namespace_id: None,
    override_params: None,
    file: None,
}).await?;

let imp_status = gl.import_export.import_status(42).await?;
```

### Integrations

```rust
let integrations = gl.integrations.list(42).await?;
gl.integrations.update(42, "slack", &serde_json::json!({
    "webhook": "https://hooks.slack.com/services/xxx"
})).await?;
gl.integrations.disable(42, "slack").await?;
```

### Badges

```rust
// Projeto
let badges = gl.badges.list_project_badges(42).await?;
let b = gl.badges.get_project_badge(42, 1).await?;
gl.badges.create_project_badge(42, &CreateBadgePayload {
    name: Some("Pipeline Status".into()),
    link_url: "https://gitlab.com/group/project/-/pipelines".into(),
    image_url: "https://gitlab.com/group/project/badges/main/pipeline.svg".into(),
}).await?;
gl.badges.update_project_badge(42, 1, &UpdateBadgePayload {
    name: Some("Pipeline (main)".into()),
    link_url: None,
    image_url: None,
}).await?;
gl.badges.delete_project_badge(42, 1).await?;

// Grupo — list_group_badges, get_group_badge, etc.
```

### Pages

```rust
let settings = gl.pages.get_settings(42).await?;
gl.pages.update_settings(42, &serde_json::json!({
    "force_https": true
})).await?;
```

### Error Tracking

```rust
let settings = gl.error_tracking.get_settings(42).await?;
gl.error_tracking.update_settings(42, &serde_json::json!({
    "active": true,
    "project_name": "Sentry Project",
    "sentry_dsn": "https://key@sentry.io/project"
})).await?;
```

### External Status Checks

```rust
let checks = gl.external_status_checks.list(42).await?;
gl.external_status_checks.create(42, &serde_json::json!({
    "name": "QA Check",
    "external_url": "https://qa.example.com/check"
})).await?;
gl.external_status_checks.delete(42, 1).await?;
```

### Issues Statistics

```rust
// Global
let stats = gl.issues_statistics.get_global().await?;

// Projeto
let stats = gl.issues_statistics.get_project(42).await?;

// Grupo
let stats = gl.issues_statistics.get_group(10).await?;
```

---

## 12. Epics & Boards

### Epics

```rust
let epics = gl.epics.list(10, None).await?;
let e = gl.epics.get(10, 5).await?;
gl.epics.create(10, &CreateEpicPayload {
    title: "Épico Q1 2025".into(),
    description: Some("Features do primeiro trimestre".into()),
    labels: None,
    start_date: None,
    due_date: None,
}).await?;
gl.epics.update(10, 5, &UpdateEpicPayload {
    title: Some("Épico Q1 2025 - Atualizado".into()),
    state_event: Some("close".into()),
    ..Default::default()
}).await?;
gl.epics.delete(10, 5).await?;

// Epic Issues
gl.epics.assign_issue(10, 5, 100).await?;
gl.epics.unassign_issue(10, 5, 100).await?;

// Child Epics
gl.epics.add_child_epic(10, 5, &CreateEpicLinkPayload {
    target_epic_iid: 6,
    link_type: None,
}).await?;
gl.epics.remove_child_epic(10, 5, 6).await?;

// Related Epics
let related = gl.epics.list_related_epics(10, 5).await?;
gl.epics.create_related_epic(10, 5, &CreateEpicLinkPayload {
    target_epic_iid: 7,
    link_type: None,
}).await?;
gl.epics.delete_related_epic(10, 5, 1).await?;
```

### Boards

```rust
// Projeto
let boards = gl.boards.list_project_boards(42).await?;
let b = gl.boards.get_project_board(42, 1).await?;
let lists = gl.boards.list_project_board_lists(42, 1).await?;
let list = gl.boards.get_project_board_list(42, 1, 10).await?;
gl.boards.create_project_board_list(42, 1, &CreateBoardListPayload {
    label_id: 5,
}).await?;
gl.boards.update_project_board_list(42, 1, 10, &UpdateBoardListPayload {
    position: Some(1),
    max_issue_count: None,
    max_issue_weight: None,
}).await?;
gl.boards.delete_project_board_list(42, 1, 10).await?;

// Grupo
let boards = gl.boards.list_group_boards(10).await?;
let b = gl.boards.get_group_board(10, 1).await?;
```

---

## 13. Vulnerabilities

```rust
let vuln = gl.vulnerabilities.get(1).await?; // Premium/Ultimate
let vulns = gl.vulnerabilities.list_project(42, None).await?;
let findings = gl.vulnerabilities.list_findings(42, None).await?;
gl.vulnerabilities.create_export(42).await?;
let export = gl.vulnerabilities.export_status(42, 1).await?;
```

---

## 14. Other

### Events

```rust
let events = gl.events.list(None).await?;
let events = gl.events.list(Some(&EventFilter {
    action: Some("pushed".into()),
    per_page: Some(20),
    ..Default::default()
})).await?;
let user_events = gl.events.list_user_events(1, None).await?;
let project_events = gl.events.list_project_events(42, None).await?;
```

### Todos

```rust
let todos = gl.todos.list(None).await?;
let todos = gl.todos.list(Some(&TodoFilter {
    state: Some("pending".into()),
    ..Default::default()
})).await?;
gl.todos.mark_done(5).await?;
gl.todos.mark_all_done().await?;
```

### Search

```rust
let results = gl.search.global("projects", "meu-projeto").await?;
let results = gl.search.in_group(10, "issues", "bug").await?;
let results = gl.search.in_project(42, "merge_requests", "feature").await?;
```

### Labels

```rust
// Projeto
let labels = gl.labels.list_project_labels(42).await?;
gl.labels.create_project_label(42, &CreateLabelPayload {
    name: "bug".into(),
    color: "#FF0000".into(),
    description: Some("Bug reports".into()),
    priority: Some(1),
}).await?;
let label = gl.labels.get_project_label(42, 5).await?;
gl.labels.update_project_label(42, &UpdateLabelPayload {
    name: Some("bug-critical".into()),
    color: Some("#CC0000".into()),
    ..Default::default()
}).await?;
gl.labels.delete_project_label(42, "bug-critical").await?;
gl.labels.promote_project_label(42, "bug").await?;

// Grupo — list_group_labels, create_group_label, etc.
```

### Milestones

```rust
// Projeto
let milestones = gl.milestones.list_project_milestones(42, None).await?;
gl.milestones.create_project_milestone(42, &CreateMilestonePayload {
    title: "Sprint 1".into(),
    description: Some("Primeira sprint".into()),
    due_date: Some("2025-02-01".into()),
    start_date: Some("2025-01-15".into()),
}).await?;
gl.milestones.update_project_milestone(42, 5, &UpdateMilestonePayload {
    title: Some("Sprint 1 - Extendida".into()),
    state_event: Some("activate".into()),
    ..Default::default()
}).await?;
gl.milestones.delete_project_milestone(42, 5).await?;

let issues = gl.milestones.list_project_milestone_issues(42, 5).await?;
let mrs = gl.milestones.list_project_milestone_merge_requests(42, 5).await?;

// Grupo — list_group_milestones, create_group_milestone, etc.
```

---

> **Dica:** Todos os filtros aceitam `None` para usar valores padrão.
> Structs de payload usam `..Default::default()` para campos opcionais.
> Consulte a [Referência da API](./api-reference.md) para a lista completa de campos.
