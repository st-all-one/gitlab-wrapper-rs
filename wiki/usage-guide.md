# Guia de Uso — Todos os Resources

Exemplos práticos de como usar cada um dos 25 resources do `gitlab-wrapper-rs`.

> **Convenções:** Todos os métodos seguem o padrão `Result<T, GitLabError>`.
> Métodos de listagem retornam `Vec<T>` (coletados eager).
> Use `?` para propagar erros ou `match` para tratamento granular.

---

## Índice

1. [Projects](#projects)
2. [Groups](#groups)
3. [Users](#users)
4. [Issues](#issues)
5. [Merge Requests](#merge-requests)
6. [Branches](#branches)
7. [Commits](#commits)
8. [Tags](#tags)
9. [Repository Files](#repository-files)
10. [Wikis](#wikis)
11. [Labels](#labels)
12. [Milestones](#milestones)
13. [Members](#members)
14. [Notes (Comentários)](#notes-comentários)
15. [Discussions](#discussions)
16. [Todos](#todos)
17. [Search](#search)
18. [Events](#events)
19. [Pipelines](#pipelines)
20. [Jobs](#jobs)
21. [Pipeline Schedules](#pipeline-schedules)
22. [Runners](#runners)
23. [Releases](#releases)
24. [Deploy Keys](#deploy-keys)
25. [Environments](#environments)

---

## Projects

```rust
use gitlab_wrapper::*;

let gl = /* ... */;

// Listar projetos com filtros
let projects = gl.projects.list(Some(&ProjectFilter {
    membership: Some(true),
    per_page: Some(50),
    search: Some("api".into()),
    order_by: Some("last_activity_at".into()),
    ..Default::default()
}))?;

for p in &projects {
    println!("{}: {} ({})", p.id, p.name, p.visibility.as_deref().unwrap_or("N/A"));
}

// Auto-paginar todas as páginas
let all = gl.projects.list_all(None)?;

// Buscar por ID
let project = gl.projects.get(42)?;
println!("Projeto: {} — {}", project.name, project.description.as_deref().unwrap_or(""));

// Buscar por caminho (encoding automático)
let project = gl.projects.get_by_path("group/subgroup/my-project")?;

// Criar projeto
use gitlab_wrapper::CreateProjectPayload;
let new_project = gl.projects.create(&CreateProjectPayload {
    name: "meu-novo-projeto".into(),
    description: Some("Projeto via API".into()),
    visibility: Some("internal".into()),
    initialize_with_readme: Some(true),
    ..Default::default()
})?;

// Atualizar
gl.projects.update(42, &UpdateProjectPayload {
    description: Some("Descrição atualizada".into()),
    ..Default::default()
})?;

// Arquivar/desarquivar
gl.projects.archive(42)?;
gl.projects.unarchive(42)?;

// Fork
gl.projects.fork(42, None)?;

// Transferir
gl.projects.transfer(42, 10)?;

// Excluir
gl.projects.delete(42)?;
```

## Groups

```rust
// Listar grupos
let groups = gl.groups.list(Some(&GroupFilter {
    top_level_only: Some(true),
    ..Default::default()
}))?;

// Buscar
let group = gl.groups.get(1)?;
println!("Grupo: {} (caminho: {})", group.full_name.as_deref().unwrap_or(""), group.full_path.as_deref().unwrap_or(""));

// Buscar por caminho (encoding automático)
let group = gl.groups.get_by_path("parent/subgroup")?;

// Criar subgrupo
let subgroup = gl.groups.create(&CreateGroupPayload {
    name: "Equipe Backend".into(),
    path: "equipe-backend".into(),
    parent_id: Some(1),
    visibility: Some("private".into()),
})?;

// Atualizar
gl.groups.update(1, &UpdateGroupPayload {
    description: Some("Grupo principal".into()),
    ..Default::default()
})?;

// Subgrupos
let subs = gl.groups.subgroups(1)?;
let descendants = gl.groups.descendant_groups(1)?;

// Projetos do grupo
let group_projects = gl.groups.projects(1)?;

// Excluir
gl.groups.delete(1)?;
```

## Users

```rust
// Listar usuários ativos
let users = gl.users.list(Some(&UserFilter {
    active: Some(true),
    ..Default::default()
}))?;

// Usuário atual
let me = gl.users.get_current()?;
println!("Logado como: {} ({})", me.name, me.username);

// Buscar por ID
let user = gl.users.get(42)?;

// Status
let status = gl.users.status(42)?;
println!("Status: {} — {}", status.emoji.as_deref().unwrap_or(""), status.message.as_deref().unwrap_or(""));

// Definir próprio status
gl.users.set_status(Some("rocket"), Some("Em lançamento!"))?;

// Preferências
let prefs = gl.users.preferences()?;

// Ativar/desativar/banir (requer admin)
gl.users.deactivate(42)?;
gl.users.activate(42)?;
gl.users.ban(42)?;
gl.users.unban(42)?;
```

## Issues

```rust
use gitlab_wrapper::*;

// Listar issues globalmente
let issues = gl.issues.list(Some(&IssueFilter {
    state: Some("opened".into()),
    labels: Some("bug,priority-high".into()),
    ..Default::default()
}))?;

// Listar issues de um projeto
let project_issues = gl.issues.list_for_project(1, Some(&IssueFilter {
    state: Some("opened".into()),
    ..Default::default()
}))?;

// Buscar issue por IID (escopo do projeto)
let issue = gl.issues.get(1, 5)?;
println!("Issue #{}: {} ({})", issue.iid, issue.title, issue.state.as_deref().unwrap_or(""));

// Criar issue
let new_issue = gl.issues.create(1, &CreateIssuePayload {
    title: "Falha ao autenticar".into(),
    description: Some("Passos para reproduzir...".into()),
    labels: Some("bug,critical".into()),
    assignee_ids: Some(vec![42]),
    milestone_id: Some(3.into()),
    ..Default::default()
})?;

// Atualizar (fechar)
gl.issues.update(1, 5, &UpdateIssuePayload {
    state_event: Some("close".into()),
    ..Default::default()
})?;

// Inscrever/desinscrever
gl.issues.subscribe(1, 5)?;
gl.issues.unsubscribe(1, 5)?;

// Time tracking
gl.issues.set_time_estimate(1, 5, "3h")?;
gl.issues.add_spent_time(1, 5, "1h30m")?;
gl.issues.reset_time_estimate(1, 5)?;
gl.issues.reset_spent_time(1, 5)?;

// Mover para outro projeto
gl.issues.move_issue(1, 5, 2)?;

// Issues por grupo
let group_issues = gl.issues.get_by_group(1, None)?;

// Excluir
gl.issues.delete(1, 5)?;
```

## Merge Requests

```rust
use gitlab_wrapper::*;

// Listar MRs abertos
let mrs = gl.merge_requests.list(Some(&MergeRequestFilter {
    state: Some("opened".into()),
    ..Default::default()
}))?;

// Listar MRs de um projeto
let project_mrs = gl.merge_requests.list_for_project(1, None)?;

// Buscar MR por IID
let mr = gl.merge_requests.get(1, 7)?;
println!("MR !{}: {} — {}", mr.iid, mr.title, mr.state.as_deref().unwrap_or(""));

// Criar MR
let new_mr = gl.merge_requests.create(1, &CreateMergeRequestPayload {
    source_branch: "feature/login-fix".into(),
    target_branch: "main".into(),
    title: "Corrige falha de autenticação".into(),
    description: Some("Closes #5".into()),
    remove_source_branch: Some(true),
    ..Default::default()
})?;

// Aprovar/desaprovar
gl.merge_requests.approve(1, 7)?;
gl.merge_requests.unapprove(1, 7)?;

// Fazer merge
gl.merge_requests.merge(1, 7, None)?;

// Rebase
gl.merge_requests.rebase(1, 7)?;

// Cancelar merge automático
gl.merge_requests.cancel_merge_when_pipeline_succeeds(1, 7)?;

// Commits e changes
let commits = gl.merge_requests.commits(1, 7)?;
let changes = gl.merge_requests.changes(1, 7)?;

// Listar por grupo
let group_mrs = gl.merge_requests.list_by_group(1, None)?;
```

## Branches

```rust
// Listar branches
let branches = gl.branches.list(1)?;

// Buscar pelo nome
let main = gl.branches.get(1, "main")?;
println!("Branch: {} — último commit: {}", main.name, main.commit.as_ref().map(|c| c.id.to_string()).unwrap_or_default());

// Criar
let branch = gl.branches.create(1, &CreateBranchPayload {
    branch: "feature/nova".into(),
    ref_: "main".into(),
})?;

// Deletar
gl.branches.delete(1, "feature/nova")?;

// Deletar merged
gl.branches.delete_merged(1)?;
```

## Commits

```rust
// Listar commits
let commits = gl.commits.list(1, Some(&CommitFilter {
    ref_name: Some("main".into()),
    since: Some("2025-01-01T00:00:00Z".into()),
    until: Some("2025-12-31T23:59:59Z".into()),
    ..Default::default()
}))?;

for c in &commits {
    println!("{} — {} ({})", c.short_id.as_deref().unwrap_or(""), c.title.as_deref().unwrap_or(""), c.committed_date.as_deref().unwrap_or(""));
}

// Buscar por SHA
let commit = gl.commits.get(1, "a1b2c3d4")?;
println!("Autor: {} <{}>", commit.author_name.as_deref().unwrap_or(""), commit.author_email.as_deref().unwrap_or(""));

// Cherry-pick
gl.commits.cherry_pick(1, "a1b2c3d4", "main")?;

// Revert
gl.commits.revert(1, "a1b2c3d4", "main")?;

// Diff
let diffs = gl.commits.diff(1, "a1b2c3d4")?;

// Comentários
let comments = gl.commits.comments(1, "a1b2c3d4")?;
gl.commits.add_comment(1, "a1b2c3d4", "Revisei o código")?;
```

## Tags

```rust
// Listar tags
let tags = gl.tags.list(1)?;
for t in &tags {
    println!("{}", t.name);
}

// Buscar
let tag = gl.tags.get(1, "v1.0.0")?;

// Criar
let new_tag = gl.tags.create(1, &CreateTagPayload {
    tag_name: "v2.0.0".into(),
    ref_: "main".into(),
    message: Some("Versão 2.0.0 — lançamento".into()),
    ..Default::default()
})?;

// Deletar
gl.tags.delete(1, "v1.0.0")?;
```

## Repository Files

```rust
// Buscar metadata do arquivo
let file = gl.repository_files.get(1, "README.md", "main")?;
println!("{} — {} bytes", file.file_name.as_deref().unwrap_or(""), file.size.unwrap_or(0));

// Conteúdo raw
let content = gl.repository_files.raw(1, "README.md", "main")?;
println!("{}", content);

// Blame
let blame = gl.repository_files.blame(1, "src/main.rs", "main")?;

// Criar arquivo
gl.repository_files.create(1, "src/config.rs", &CreateFilePayload {
    branch: "main".into(),
    content: "pub const NAME: &str = \"my-app\";".into(),
    commit_message: "Adiciona config".into(),
})?;

// Atualizar
gl.repository_files.update(1, "src/config.rs", &UpdateFilePayload {
    branch: "main".into(),
    content: "pub const NAME: &str = \"new-name\";".into(),
    commit_message: "Atualiza config".into(),
})?;

// Deletar
gl.repository_files.delete(1, "src/config.rs", "main", "Remove config")?;
```

## Wikis

```rust
// Listar páginas
let pages = gl.wikis.list(1)?;
for p in &pages {
    println!("Página: {} (slug: {})", p.title, p.slug);
}

// Buscar por slug
let home = gl.wikis.get(1, "home")?;
println!("{}", home.content.as_deref().unwrap_or(""));

// Criar
gl.wikis.create(1, &CreateWikiPagePayload {
    title: "Guia de Contribuição".into(),
    content: "# Como contribuir\n\nSiga as diretrizes.".into(),
    format: Some("markdown".into()),
})?;

// Atualizar
gl.wikis.update(1, "guia-de-contribuicao", &UpdateWikiPagePayload {
    content: Some("Conteúdo atualizado".into()),
    ..Default::default()
})?;

// Deletar
gl.wikis.delete(1, "guia-de-contribuicao")?;
```

## Labels

```rust
// Labels de projeto
let project_labels = gl.labels.list_project_labels(1)?;
for l in &project_labels {
    println!("{} ({}) — {}", l.name, l.color.as_deref().unwrap_or(""), l.description.as_deref().unwrap_or(""));
}

// Buscar label específica
let label = gl.labels.get_project_label(1, 5)?;

// Criar label de projeto
gl.labels.create_project_label(1, &CreateLabelPayload {
    name: "prioridade:alta".into(),
    color: "#FF0000".into(),
    description: Some("Issues de alta prioridade".into()),
    ..Default::default()
})?;

// Atualizar
gl.labels.update_project_label(1, &UpdateLabelPayload {
    color: Some("#CC0000".into()),
    ..Default::default()
})?;

// Promover para group label
let group_label = gl.labels.promote_project_label(1, "prioridade:alta")?;

// Deletar
gl.labels.delete_project_label(1, "prioridade:alta")?;

// Labels de grupo
let group_labels = gl.labels.list_group_labels(1)?;
gl.labels.get_group_label(1, 5)?;
gl.labels.create_group_label(1, &CreateLabelPayload { name: "grupo:dev".into(), color: "#00FF00".into(), ..Default::default() })?;
gl.labels.update_group_label(1, &UpdateLabelPayload { description: Some("Equipe de desenvolvimento".into()), ..Default::default() })?;
gl.labels.delete_group_label(1, "grupo:dev")?;
```

## Milestones

```rust
// Milestones de projeto
let milestones = gl.milestones.list_project_milestones(1, None)?;
for ms in &milestones {
    println!("{}: {} — entrega: {}", ms.title, ms.state.as_deref().unwrap_or(""), ms.due_date.as_deref().unwrap_or(""));
}

// Buscar
let ms = gl.milestones.get_project_milestone(1, 3)?;

// Criar
gl.milestones.create_project_milestone(1, &CreateMilestonePayload {
    title: "Sprint 42".into(),
    description: Some("Features do segundo semestre".into()),
    due_date: Some("2026-12-20".into()),
    ..Default::default()
})?;

// Issues e MRs do milestone
let ms_issues = gl.milestones.list_project_milestone_issues(1, 3)?;
let ms_mrs = gl.milestones.list_project_milestone_merge_requests(1, 3)?;

// O mesmo para milestones de grupo
let group_ms = gl.milestones.list_group_milestones(1, None)?;
gl.milestones.get_group_milestone(1, 3)?;
gl.milestones.create_group_milestone(1, &CreateMilestonePayload { title: "Framework Q3".into(), ..Default::default() })?;
gl.milestones.list_group_milestone_issues(1, 3)?;
gl.milestones.list_group_milestone_merge_requests(1, 3)?;
gl.milestones.delete_group_milestone(1, 3)?;
```

## Members

```rust
// Membros de projeto
let members = gl.members.list_project_members(1)?;
for m in &members {
    println!("{} ({}) — level: {}", m.name.as_deref().unwrap_or(""), m.username.as_deref().unwrap_or(""), m.access_level.unwrap_or(0));
}

// Adicionar
gl.members.add_project_member(1, &AddMemberPayload {
    user_id: 42,
    access_level: 30, // 10=Guest, 20=Reporter, 30=Developer, 40=Maintainer, 50=Owner
})?;

// Herdados (inclui membros de grupos pais)
let inherited = gl.members.list_project_inherited_members(1)?;

// O mesmo para grupos
gl.members.list_group_members(1)?;
gl.members.get_group_member(1, 42)?;
gl.members.add_group_member(1, &AddMemberPayload { user_id: 42, access_level: 30 })?;
gl.members.delete_group_member(1, 42)?;
```

## Notes (Comentários)

```rust
// Notas de issue
let notes = gl.notes.list_issue_notes(1, 5)?;
for n in &notes {
    println!("[{}] {}", n.created_at.as_deref().unwrap_or(""), n.body.as_deref().unwrap_or(""));
}

// Criar nota em issue
gl.notes.create_issue_note(1, 5, &CreateNotePayload {
    body: "Corrigido na branch feature/fix.".into(),
    ..Default::default()
})?;

// Buscar nota específica
let note = gl.notes.get_issue_note(1, 5, 123)?;

// Atualizar
gl.notes.update_issue_note(1, 5, 123, &UpdateNotePayload {
    body: "**Corrigido!** Verificar em produção.".into(),
    ..Default::default()
})?;

// Deletar
gl.notes.delete_issue_note(1, 5, 123)?;

// Mesmas operações para MR, commit, snippet e wiki
gl.notes.list_mr_notes(1, 7)?;
gl.notes.create_mr_note(1, 7, &CreateNotePayload { body: "Revisado.".into(), ..Default::default() })?;
gl.notes.delete_mr_note(1, 7, 123)?;

gl.notes.list_commit_notes(1, "a1b2c3d4")?;
gl.notes.create_commit_note(1, "a1b2c3d4", &CreateNotePayload { body: "Comentário no commit.".into(), ..Default::default() })?;
gl.notes.update_commit_note(1, "a1b2c3d4", 123, &UpdateNotePayload { body: "Atualizado.".into(), ..Default::default() })?;
gl.notes.delete_commit_note(1, "a1b2c3d4", 123)?;

gl.notes.list_snippet_notes(1, 42)?;
gl.notes.list_wiki_notes(1, "home")?;
gl.notes.create_wiki_note(1, "home", &CreateNotePayload { body: "Nota na wiki.".into(), ..Default::default() })?;
```

## Discussions

```rust
// Discussões de issue
let discussions = gl.discussions.list_issue_discussions(1, 5)?;
for d in &discussions {
    println!("Discussão {} (resolvida: {:?})", d.id, d.resolved);
}

// Criar discussão
gl.discussions.create_issue_discussion(1, 5, &CreateDiscussionPayload {
    body: "Precisamos revisar esta abordagem.".into(),
})?;

// Buscar discussão específica
let disc = gl.discussions.get_issue_discussion(1, 5, "abc123")?;

// Adicionar nota
gl.discussions.add_issue_discussion_note(1, 5, "abc123", &CreateNotePayload {
    body: "Concordo. Vou ajustar.".into(),
})?;

// Atualizar nota
gl.discussions.update_issue_discussion_note(1, 5, "abc123", 456, "Texto atualizado")?;

// Deletar nota
gl.discussions.delete_issue_discussion_note(1, 5, "abc123", 456)?;

// Resolver/reabrir
gl.discussions.resolve_issue_discussion(1, 5, "abc123", true)?;

// Mesmas operações para MRs e commits
gl.discussions.list_mr_discussions(1, 7)?;
gl.discussions.list_commit_discussions(1, "a1b2c3d4")?;
gl.discussions.resolve_mr_discussion(1, 7, "def456", false)?;
gl.discussions.resolve_commit_discussion(1, "a1b2c3d4", "ghi789", true)?;
```

## Todos

```rust
// Listar todos do usuário atual
let todos = gl.todos.list(Some(&TodoFilter {
    ..Default::default()
}))?;
println!("Você tem {} tarefas pendentes", todos.len());

// Marcar como done
gl.todos.mark_done(42)?;

// Marcar todos como done
gl.todos.mark_all_done()?;
```

## Search

```rust
// Busca global
let results = gl.search.global("projects", "api-gateway")?;
for r in &results {
    println!("{}: {}", r.id, r.filename.as_deref().unwrap_or("N/A"));
}

// Busca em grupo
let group_results = gl.search.in_group(1, "issues", "bug crítico")?;

// Busca em projeto
let project_results = gl.search.in_project(1, "merge_requests", "feature")?;
```

## Events

```rust
// Eventos do usuário atual
let events = gl.events.list(Some(&EventFilter {
    ..Default::default()
}))?;
for e in &events {
    println!("[{}] {} em {}", e.created_at.as_deref().unwrap_or(""), e.action_name.as_deref().unwrap_or(""), e.target_type.as_deref().unwrap_or(""));
}

// Eventos de um usuário específico
let user_events = gl.events.list_user_events(42, None)?;

// Eventos de um projeto
let project_events = gl.events.list_project_events(1, None)?;
```

## Pipelines

```rust
// Listar pipelines
let pipelines = gl.pipelines.list(1, Some(&PipelineFilter {
    status: Some("failed".into()),
    ref_: Some("main".into()),
    ..Default::default()
}))?;

for p in &pipelines {
    println!("Pipeline #{}: {} ({})", p.id, p.status.as_deref().unwrap_or(""), p.ref_.as_deref().unwrap_or(""));
}

// Buscar
let pipeline = gl.pipelines.get(1, 123)?;

// Latest
let latest = gl.pipelines.get_latest(1)?;

// Criar
gl.pipelines.create(1, &CreatePipelinePayload {
    ref_: "main".into(),
    variables: Some(vec![PipelineVariable {
        key: Some("DEPLOY_ENV".into()),
        value: Some("staging".into()),
        variable_type: None,
    }]),
})?;

// Retry, cancel, delete
gl.pipelines.retry(1, 123)?;
gl.pipelines.cancel(1, 123)?;
gl.pipelines.delete(1, 123)?;

// Variables
let vars = gl.pipelines.variables(1, 123)?;

// Test reports
let report = gl.pipelines.test_report(1, 123)?;
let summary = gl.pipelines.test_report_summary(1, 123)?;
```

## Jobs

```rust
// Listar jobs de um projeto
let jobs = gl.jobs.list(1, None)?;

// Listar jobs de uma pipeline
let pipeline_jobs = gl.jobs.list_by_pipeline(1, 123, None)?;
for j in &pipeline_jobs {
    println!("Job #{}: {} — {} ({:?}s)", j.id, j.name.as_deref().unwrap_or(""), j.status.as_deref().unwrap_or(""), j.duration);
}

// Buscar job
let job = gl.jobs.get(1, 456)?;

// Trace
let trace = gl.jobs.trace(1, 456)?;
println!("Log: {}", trace);

// Artifacts
let artifacts = gl.jobs.artifacts(1, 456)?;

// Ações
gl.jobs.cancel(1, 456)?;
gl.jobs.retry(1, 456)?;
gl.jobs.play(1, 456)?;
gl.jobs.erase(1, 456)?;
```

## Pipeline Schedules

```rust
// Listar agendamentos
let schedules = gl.pipeline_schedules.list(1)?;
for s in &schedules {
    println!("Schedule: {} — cron: {} (ativo: {:?})", s.description.as_deref().unwrap_or(""), s.cron.as_deref().unwrap_or(""), s.active);
}

// Buscar
let schedule = gl.pipeline_schedules.get(1, 3)?;

// Criar
gl.pipeline_schedules.create(1, &CreatePipelineSchedulePayload {
    description: "Deploy noturno".into(),
    ref_: "main".into(),
    cron: "0 2 * * *".into(),
    cron_timezone: Some("America/Sao_Paulo".into()),
    active: Some(true),
})?;

// Atualizar
gl.pipeline_schedules.update(1, 3, &UpdatePipelineSchedulePayload {
    active: Some(false),
    ..Default::default()
})?;

// Take ownership
gl.pipeline_schedules.take_ownership(1, 3)?;

// Variáveis
gl.pipeline_schedules.create_variable(1, 3, "DEPLOY_ENV", "production")?;
gl.pipeline_schedules.update_variable(1, 3, 1, "staging")?;
gl.pipeline_schedules.delete_variable(1, 3, 1)?;

// Deletar
gl.pipeline_schedules.delete(1, 3)?;
```

## Runners

```rust
// Listar runners
let runners = gl.runners.list()?;
for r in &runners {
    println!("Runner #{}: {} ({})", r.id, r.description.as_deref().unwrap_or(""), r.status.as_deref().unwrap_or(""));
}

// Buscar
let runner = gl.runners.get(42)?;

// Registrar
gl.runners.create(&CreateRunnerPayload {
    description: Some("server-prod-01".into()),
    run_untagged: Some(true),
    ..Default::default()
})?;

// Atualizar
gl.runners.update(42, &UpdateRunnerPayload {
    active: Some(false),
    ..Default::default()
})?;

// Listar jobs
let runner_jobs = gl.runners.list_jobs(42)?;

// Deletar
gl.runners.delete(42)?;
```

## Releases

```rust
// Listar releases
let releases = gl.releases.list(1)?;
for r in &releases {
    println!("Release: {} — \"{}\"", r.tag_name, r.name.as_deref().unwrap_or(""));
}

// Buscar por tag
let release = gl.releases.get(1, "v1.0.0")?;

// Criar
gl.releases.create(1, &CreateReleasePayload {
    name: "Versão 2.0".into(),
    tag_name: "v2.0.0".into(),
    description: Some("## Novidades\n- Suporte OAuth\n- Melhorias".into()),
    ref_: "main".into(),
    ..Default::default()
})?;

// Asset links
gl.releases.create_link(1, "v2.0.0", &CreateReleaseLinkPayload {
    name: "Binário Linux".into(),
    url: "https://releases.example.com/app.tar.gz".into(),
    ..Default::default()
})?;
gl.releases.delete_link(1, "v2.0.0", 1)?;
```

## Deploy Keys

```rust
// Listar chaves
let keys = gl.deploy_keys.list(1)?;
for k in &keys {
    println!("Chave: {} ({})", k.title.as_deref().unwrap_or(""), k.fingerprint.as_deref().unwrap_or(""));
}

// Buscar
let key = gl.deploy_keys.get(1, 5)?;

// Criar
gl.deploy_keys.create(1, &CreateDeployKeyPayload {
    title: "Servidor Produção".into(),
    key: "ssh-rsa AAAAB3NzaC1yc...".into(),
    can_push: Some(false),
})?;

// Atualizar
gl.deploy_keys.update(1, 5, &UpdateDeployKeyPayload {
    title: Some("Servidor Produção v2".into()),
    can_push: Some(true),
})?;

// Ativar
gl.deploy_keys.enable(1, 5)?;

// Deletar
gl.deploy_keys.delete(1, 5)?;
```

## Environments

```rust
// Listar ambientes
let envs = gl.environments.list(1)?;
for e in &envs {
    println!("Ambiente: {} ({})", e.name, e.state.as_deref().unwrap_or(""));
}

// Buscar
let env = gl.environments.get(1, 5)?;

// Criar
gl.environments.create(1, &CreateEnvironmentPayload {
    name: "staging".into(),
    external_url: Some("https://staging.example.com".into()),
})?;

// Parar (encerrar)
gl.environments.stop(1, 5)?;

// Deletar
gl.environments.delete(1, 5)?;
```

---

## Resumo de Métodos por Resource

| Resource | `list` | `get` | `create` | `update` | `delete` | Métodos Extras |
|---|---|---|---|---|---|---|
| Projects | ✅ | ✅ | ✅ | ✅ | ✅ | `archive`, `unarchive`, `fork`, `transfer`, `list_all` |
| Groups | ✅ | ✅ | ✅ | ✅ | ✅ | `subgroups`, `descendant_groups`, `projects` |
| Users | ✅ | ✅ | ✅ | ✅ | ✅ | `get_current`, `status`, `set_status`, `preferences`, `deactivate`, `activate`, `ban`, `unban` |
| Issues | ✅ | ✅ | ✅ | ✅ | ✅ | `subscribe`, `unsubscribe`, `move_issue`, `set_time_estimate`, `add_spent_time`, `reset_time_estimate`, `reset_spent_time`, `get_by_group` |
| Merge Requests | ✅ | ✅ | ✅ | ✅ | ✅ | `merge`, `approve`, `unapprove`, `rebase`, `cancel_merge_when_pipeline_succeeds`, `commits`, `changes`, `list_by_group` |
| Branches | ✅ | ✅ | ✅ | — | ✅ | `delete_merged` |
| Commits | ✅ | ✅ | ✅ | — | — | `cherry_pick`, `revert`, `diff`, `refs`, `comments`, `add_comment` |
| Tags | ✅ | ✅ | ✅ | — | ✅ | — |
| Repository Files | — | ✅ | ✅ | ✅ | ✅ | `raw`, `blame` |
| Wikis | ✅ | ✅ | ✅ | ✅ | ✅ | — |
| Labels | ✅ | ✅ | ✅ | ✅ | ✅ | `promote_project_label` |
| Milestones | ✅ | ✅ | ✅ | ✅ | ✅ | `list_*_milestone_issues`, `list_*_milestone_merge_requests` |
| Members | ✅ | ✅ | ✅ | ✅ | ✅ | `list_*_inherited_members` |
| Notes | ✅ | ✅ | ✅ | ✅ | ✅ | CRUD completo para issue/MR/commit/snippet/wiki |
| Discussions | ✅ | ✅ | ✅ | ✅ | ✅ | `add_note`, `update_note`, `delete_note`, `resolve` |
| Todos | ✅ | — | — | — | ✅ | `mark_all_done` |
| Search | ✅ | — | — | — | — | `in_group`, `in_project` |
| Events | ✅ | — | — | — | — | `list_user_events`, `list_project_events` |
| Pipelines | ✅ | ✅ | ✅ | — | ✅ | `retry`, `cancel`, `variables`, `test_report`, `test_report_summary`, `get_latest` |
| Jobs | ✅ | ✅ | — | — | — | `trace`, `artifacts`, `cancel`, `retry`, `play`, `erase`, `list_by_pipeline` |
| Pipeline Schedules | ✅ | ✅ | ✅ | ✅ | ✅ | `take_ownership`, `create_variable`, `update_variable`, `delete_variable` |
| Runners | ✅ | ✅ | ✅ | ✅ | ✅ | `list_jobs` |
| Releases | ✅ | ✅ | ✅ | ✅ | ✅ | `create_link`, `delete_link` |
| Deploy Keys | ✅ | ✅ | ✅ | ✅ | ✅ | `enable` |
| Environments | ✅ | ✅ | ✅ | ✅ | ✅ | `stop` |
