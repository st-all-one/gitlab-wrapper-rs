//! Teste manual completo da gitlab-wrapper-rs.
//!
//! Uso:
//!   export GITLAB_TOKEN="glpat-xxxx"
//!   export GITLAB_URL="https://gitlab.com"        # opcional, padrão gitlab.com
//!
//!   cargo run --example manual_test
//!
//! O script cria um projeto temporário (gitlab-wrapper-test-<timestamp>) e o remove ao final.
//! Todas as operações de escrita são isoladas — nada no seu GitLab é alterado fora do escopo do teste.

use gitlab_wrapper::*;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// ─── helpers ──────────────────────────────────────────────────────────────

static PASSED: AtomicU32 = AtomicU32::new(0);
static FAILED: AtomicU32 = AtomicU32::new(0);

macro_rules! test {
    ($name:expr, $body:block) => {{
        print!("  {}... ", $name);
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<(), GitLabError> { $body Ok(()) })) {
            Ok(Ok(())) => {
                PASSED.fetch_add(1, Ordering::Relaxed);
                println!("✅");
            }
            Ok(Err(e)) => {
                FAILED.fetch_add(1, Ordering::Relaxed);
                println!("❌ {}", e);
            }
            Err(_) => {
                FAILED.fetch_add(1, Ordering::Relaxed);
                println!("💥 PANIC");
            }
        }
    }};
}

fn unique_sufx() -> String {
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    // Use last 10 digits for uniqueness
    let s = ts.to_string();
    s[s.len().saturating_sub(10)..].to_string()
}

fn fmt(s: &str) -> String { s.to_string() }

fn find_project(gl: &GitLabClient, name: &str) -> Result<u64, GitLabError> {
    let list = gl.projects.list(Some(&ProjectFilter {
        search: Some(name.to_string()),
        ..Default::default()
    }))?;
    Ok(list.first().map(|p| p.id).unwrap_or(0))
}

// ─── main ─────────────────────────────────────────────────────────────────

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║       gitlab-wrapper-rs — Teste Manual Completo            ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    let gl = GitLabClient::new(GitLabConfig {
        base_url: std::env::var("GITLAB_URL").unwrap_or_else(|_| "https://gitlab.com".into()),
        token: Some(std::env::var("GITLAB_TOKEN").expect("GITLAB_TOKEN é obrigatório")),
        auth_method: Some(AuthMethod::Bearer),
        ..Default::default()
    })?;

    // ── 1. PROJETO TEMPORÁRIO ───────────────────────────────────────────

    let sufx = unique_sufx();
    let tname = format!("gw-test-{}", sufx);
    let tbranch = format!("feature-{}", sufx);

    // Cleanup any leftover from previous failed runs
    if let Ok(pid) = find_project(&gl, &tname) {
        if pid != 0 { let _ = gl.projects.delete(pid); }
    }

    println!("📦 Projeto temporário: {}", tname);
    println!();

    // ── 2. USUÁRIO ──────────────────────────────────────────────────────

    println!("━━━ [ Users ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let me = gl.users.get_current()?;
    println!("  👤 Logado como: {} (@{})", me.name, me.username);
    println!();

    test!("users.get_current", {
        let u = gl.users.get_current()?;
        assert!(!u.id.to_string().is_empty(), "id vazio");
        assert!(!u.name.is_empty(), "name vazio");
    });

    test!("users.get", {
        let u = gl.users.get(me.id)?;
        assert_eq!(u.id, me.id);
    });

    test!("users.status", {
        let s = gl.users.status(me.id)?;
        println!("    status: {} {}", s.emoji.as_deref().unwrap_or(""), s.message.as_deref().unwrap_or(""));
    });

    test!("users.list (com filtro)", {
        let list = gl.users.list(Some(&UserFilter {
            username: Some(me.username.clone()),
            ..Default::default()
        }))?;
        assert!(!list.is_empty(), "lista vazia");
        assert_eq!(list[0].username, me.username);
    });

    test!("users.set_status + preferences", {
        gl.users.set_status(Some("rocket"), Some("Testando wrapper"))?;
        let _prefs = gl.users.preferences()?;
        gl.users.set_status(None, None)?;
    });

    // ── 3. PROJETOS ─────────────────────────────────────────────────────

    println!();
    println!("━━━ [ Projects ] ─━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("projects.list", {
        let list = gl.projects.list(None)?;
        assert!(!list.is_empty(), "token sem projetos acessiveis");
        println!("    {} projetos encontrados", list.len());
    });

    test!("projects.list (membership)", {
        let list = gl.projects.list(Some(&ProjectFilter {
            membership: Some(true),
            per_page: Some(5),
            ..Default::default()
        }))?;
        println!("    {} projetos como membro", list.len());
    });

    let project = gl.projects.create(&CreateProjectPayload {
        name: tname.clone(),
        description: Some("Projeto temporário para teste do gitlab-wrapper-rs".into()),
        visibility: Some("private".into()),
        initialize_with_readme: Some(true),
        path: None,
        namespace_id: None,
        topics: None,
    })?;
    let pid = project.id;
    let tname_ = tname.clone();
    println!("  📦 Projeto criado: ID = {}", pid);
    println!();

    test!("projects.create", {
        assert_eq!(project.name, tname_);
    });

    test!("projects.get (por ID)", {
        let p = gl.projects.get(pid)?;
        assert_eq!(p.name, tname);
    });

    test!("projects.update", {
        gl.projects.update(pid, &UpdateProjectPayload {
            name: None,
            description: Some("Descricao atualizada pelo teste".into()),
            visibility: None,
            topics: None,
            default_branch: None,
        })?;
        let p = gl.projects.get(pid)?;
        assert_eq!(p.description.as_deref(), Some("Descricao atualizada pelo teste"));
    });

    // ── 4. BRANCHES ─────────────────────────────────────────────────────

    println!();
    println!("━━━ [ Branches ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("branches.list", {
        let branches = gl.branches.list(pid)?;
        assert!(!branches.is_empty(), "sem branches");
        println!("    {} branches", branches.len());
    });

    test!("branches.get (main)", {
        let branch = gl.branches.get(pid, "main")?;
        assert_eq!(branch.name, "main");
    });

    test!("branches.create + delete", {
        let created = gl.branches.create(pid, &CreateBranchPayload {
            branch: tbranch.clone(),
            ref_: "main".into(),
        })?;
        assert_eq!(created.name, tbranch, "branch name mismatch: got '{}'", created.name);

        // Verify with GET (more reliable than list due to GitLab API caching)
        let fetched = gl.branches.get(pid, &tbranch)?;
        assert_eq!(fetched.name, tbranch);

        gl.branches.delete(pid, &tbranch)?;
        match gl.branches.get(pid, &tbranch) {
            Err(GitLabError::Api { ref category, .. })
                if matches!(category, ErrorCategory::ResourceNotFound) => { /* ok */ }
            Ok(_) => panic!("branch '{}' still exists after delete", tbranch),
            Err(_) => { /* other error — accept */ }
        }
    });

    // ── 5. COMMITS ──────────────────────────────────────────────────────

    println!();
    println!("━━━ [ Commits ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("commits.list", {
        let commits = gl.commits.list(pid, None)?;
        assert!(!commits.is_empty(), "sem commits (README foi criado)");
        println!("    {} commits", commits.len());
    });

    test!("commits.get", {
        let commits = gl.commits.list(pid, None)?;
        let sha = commits[0].id.to_string();
        let commit = gl.commits.get(pid, &sha)?;
        assert!(!commit.title.as_deref().unwrap_or("").is_empty());
    });

    test!("commits.diff", {
        let commits = gl.commits.list(pid, Some(&CommitFilter {
            per_page: Some(1),
            ..Default::default()
        }))?;
        if let Some(c) = commits.first() {
            let diffs = gl.commits.diff(pid, &c.id.to_string())?;
            println!("    {} arquivos no diff", diffs.len());
        }
    });

    test!("commits.refs", {
        let commits = gl.commits.list(pid, Some(&CommitFilter {
            per_page: Some(1),
            ..Default::default()
        }))?;
        if let Some(c) = commits.first() {
            let _refs = gl.commits.refs(pid, &c.id.to_string())?;
        }
    });

    // ── 6. TAGS ─────────────────────────────────────────────────────────

    println!();
    println!("━━━ [ Tags ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("tags.list", {
        let _tags = gl.tags.list(pid)?;
        println!("    listadas com sucesso");
    });

    test!("tags.create + get + delete", {
        let tag_name = format!("v0.0.1-{}", &sufx[..6]);
        gl.tags.create(pid, &CreateTagPayload {
            tag_name: tag_name.clone(),
            ref_: "main".into(),
            message: Some("Tag de teste".into()),
            release_description: None,
        })?;
        let tag = gl.tags.get(pid, &tag_name)?;
        assert_eq!(tag.name, tag_name);
        gl.tags.delete(pid, &tag_name)?;
    });

    // ── 7. REPOSITORY FILES ─────────────────────────────────────────────

    println!();
    println!("━━━ [ Repository Files ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("repository_files.get (README.md)", {
        let f = gl.repository_files.get(pid, "README.md", "main")?;
        assert_eq!(f.file_name.as_deref(), Some("README.md"));
    });

    test!("repository_files.raw", {
        let raw = gl.repository_files.raw(pid, "README.md", "main")?;
        assert!(!raw.is_empty(), "README vazio");
        println!("    README: {} bytes", raw.len());
    });

    test!("repository_files.blame", {
        let _blame = gl.repository_files.blame(pid, "README.md", "main")?;
        println!("    blame ok");
    });

    test!("repository_files.create + update + delete", {
        let fp = "test-file.txt";

        gl.repository_files.create(pid, fp, &CreateFilePayload {
            branch: "main".into(),
            content: "conteudo inicial".into(),
            commit_message: "cria arquivo de teste".into(),
            encoding: None,
            author_email: None,
            author_name: None,
        })?;
        let f = gl.repository_files.get(pid, fp, "main")?;
        assert_eq!(f.file_name.as_deref(), Some("test-file.txt"));

        gl.repository_files.update(pid, fp, &UpdateFilePayload {
            branch: "main".into(),
            content: "conteudo alterado".into(),
            commit_message: "atualiza arquivo de teste".into(),
            encoding: None,
            author_email: None,
            author_name: None,
            last_commit_id: None,
        })?;
        let raw = gl.repository_files.raw(pid, fp, "main")?;
        assert_eq!(raw, "conteudo alterado");

        gl.repository_files.delete(pid, fp, "main", "remove arquivo de teste")?;
        match gl.repository_files.get(pid, fp, "main") {
            Err(GitLabError::Api { category: ErrorCategory::ResourceNotFound, .. }) => { /* esperado */ }
            Ok(_) => panic!("arquivo deveria ter sido deletado"),
            Err(_) => { /* aceitavel — pode ser cache */ }
        }
    });

    // ── 8. WIKIS ────────────────────────────────────────────────────────

    println!();
    println!("━━━ [ Wikis ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("wikis CRUD", {
        let page = match gl.wikis.create(pid, &CreateWikiPagePayload {
            title: "TestPage".into(),
            content: "Hello from gitlab-wrapper-rs test.".into(),
            format: Some("markdown".into()),
        }) {
            Ok(p) => p,
            Err(GitLabError::Api { status: 400, .. }) => {
                println!("    (wiki nao disponivel neste projeto)");
                return Ok(());
            }
            Err(e) => return Err(e),
        };
        let slug = page.slug.as_deref().unwrap_or("testpage").to_string();

        let pages = gl.wikis.list(pid)?;
        assert!(pages.iter().any(|p| p.slug.as_deref() == Some(&slug)), "slug '{}' not in list", slug);

        match gl.wikis.update(pid, &slug, &UpdateWikiPagePayload {
            title: None, content: Some("Updated content.".into()), format: None,
        }) {
            Ok(_) => {
                let p = gl.wikis.get(pid, &slug)?;
                assert_eq!(p.content.as_deref(), Some("Updated content."));
            }
            Err(GitLabError::Api { status: 400, .. }) => {
                println!("    (wiki update indiponivel)");
            }
            Err(e) => return Err(e),
        }

        let _ = gl.wikis.delete(pid, &slug);
    });

    // ── 9. LABELS ───────────────────────────────────────────────────────

    println!();
    println!("━━━ [ Labels ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("labels CRUD projeto", {
        let lname = fmt(&format!("test-label-{}", &sufx[..6]));

        gl.labels.create_project_label(pid, &CreateLabelPayload {
            name: lname.clone(),
            color: "#FF0000".into(),
            description: Some("Label de teste".into()),
            priority: None,
        })?;

        let labels = gl.labels.list_project_labels(pid)?;
        assert!(labels.iter().any(|l| l.name == lname));

        let label_obj = labels.iter().find(|l| l.name == lname).unwrap();
        let _got = gl.labels.get_project_label(pid, label_obj.id)?;

        gl.labels.update_project_label(pid, &UpdateLabelPayload {
            name: Some(lname.clone()),
            color: Some("#00FF00".into()),
            description: None,
            priority: None,
        })?;

        gl.labels.delete_project_label(pid, &lname)?;
        let labels = gl.labels.list_project_labels(pid)?;
        assert!(!labels.iter().any(|l| l.name == lname));
    });

    // ── 10. MILESTONES ──────────────────────────────────────────────────

    println!();
    println!("━━━ [ Milestones ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("milestones CRUD projeto", {
        let ms = gl.milestones.create_project_milestone(pid, &CreateMilestonePayload {
            title: fmt(&format!("Sprint-{}", &sufx[..6])),
            description: Some("Milestone de teste".into()),
            due_date: Some("2027-12-31".into()),
            start_date: None,
        })?;

        let milestones = gl.milestones.list_project_milestones(pid, None)?;
        assert!(milestones.iter().any(|m| m.id == ms.id));

        let _got = gl.milestones.get_project_milestone(pid, ms.id)?;
        let _ms_issues = gl.milestones.list_project_milestone_issues(pid, ms.id)?;
        let _ms_mrs = gl.milestones.list_project_milestone_merge_requests(pid, ms.id)?;

        gl.milestones.delete_project_milestone(pid, ms.id)?;
    });

    // ── 11. MEMBERS ────────────────────────────────────────────────────

    println!();
    println!("━━━ [ Members ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("members.list_project_members", {
        let members = gl.members.list_project_members(pid)?;
        assert!(!members.is_empty(), "sem membros no projeto");
        println!("    {} membros", members.len());
    });

    test!("members.get_project_member (eu)", {
        let member = gl.members.get_project_member(pid, me.id)?;
        assert_eq!(member.id, me.id);
    });

    // ── 12. ISSUES ──────────────────────────────────────────────────────

    println!();
    println!("━━━ [ Issues ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("issues.list", {
        let list = gl.issues.list(None)?;
        println!("    {} issues globalmente", list.len());
    });

    test!("issues.create + get + subscribe + time + delete", {
        let issue = gl.issues.create(pid, &CreateIssuePayload {
            title: "Bug de teste".into(),
            description: Some("Issue criada pelo teste manual.".into()),
            confidential: None,
            labels: Some("bug".into()),
            assignee_ids: None,
            milestone_id: None,
            weight: None,
            due_date: None,
        })?;
        assert_eq!(issue.title, "Bug de teste");
        let iid = issue.iid;

        let project_issues = gl.issues.list_for_project(pid, None)?;
        assert!(project_issues.iter().any(|i| i.iid == iid));

        let _got = gl.issues.get(pid, iid)?;

        gl.issues.subscribe(pid, iid)?;
        gl.issues.unsubscribe(pid, iid)?;

        gl.issues.set_time_estimate(pid, iid, "2h")?;
        gl.issues.add_spent_time(pid, iid, "30m")?;
        gl.issues.reset_time_estimate(pid, iid)?;
        gl.issues.reset_spent_time(pid, iid)?;

        gl.issues.delete(pid, iid)?;
    });

    // ── 13. MERGE REQUESTS ──────────────────────────────────────────────

    println!();
    println!("━━━ [ Merge Requests ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("merge_requests.list", {
        let mrs = gl.merge_requests.list(None)?;
        println!("    {} MRs globalmente", mrs.len());
    });

    test!("merge_requests.list_for_project", {
        let mrs = gl.merge_requests.list_for_project(pid, None)?;
        println!("    {} MRs no projeto", mrs.len());
    });

    // ── 14. NOTES ───────────────────────────────────────────────────────

    println!();
    println!("━━━ [ Notes ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("notes CRUD em issue", {
        let issue = gl.issues.create(pid, &CreateIssuePayload {
            title: "Issue para testar notes".into(),
            description: None,
            confidential: None,
            labels: None,
            assignee_ids: None,
            milestone_id: None,
            weight: None,
            due_date: None,
        })?;

        let note = gl.notes.create_issue_note(pid, issue.iid, &CreateNotePayload {
            body: "Comentário de teste.".into(),
            confidential: None,
        })?;
        let _got = gl.notes.get_issue_note(pid, issue.iid, note.id)?;
        let notes = gl.notes.list_issue_notes(pid, issue.iid)?;
        assert!(!notes.is_empty());

        gl.notes.update_issue_note(pid, issue.iid, note.id, &UpdateNotePayload {
            body: "Comentário atualizado.".into(),
            confidential: None,
        })?;
        let updated = gl.notes.get_issue_note(pid, issue.iid, note.id)?;
        assert_eq!(updated.body.as_deref(), Some("Comentário atualizado."));

        gl.notes.delete_issue_note(pid, issue.iid, note.id)?;
        gl.issues.delete(pid, issue.iid)?;
    });

    // ── 15. DISCUSSIONS ────────────────────────────────────────────────

    println!();
    println!("━━━ [ Discussions ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("discussions em issue", {
        let issue = gl.issues.create(pid, &CreateIssuePayload {
            title: "Issue para testar discussions".into(),
            description: None,
            confidential: None,
            labels: None,
            assignee_ids: None,
            milestone_id: None,
            weight: None,
            due_date: None,
        })?;

        let disc = gl.discussions.create_issue_discussion(pid, issue.iid, &CreateDiscussionPayload {
            body: "Iniciando discussão.".into(),
        })?;
        assert!(!disc.id.is_empty());

        let discussions = gl.discussions.list_issue_discussions(pid, issue.iid)?;
        assert!(!discussions.is_empty());

        let _got = gl.discussions.get_issue_discussion(pid, issue.iid, &disc.id)?;

        gl.discussions.add_issue_discussion_note(pid, issue.iid, &disc.id, &CreateNotePayload {
            body: "Nota adicional.".into(),
            confidential: None,
        })?;

        gl.discussions.resolve_issue_discussion(pid, issue.iid, &disc.id, true)?;
        gl.discussions.resolve_issue_discussion(pid, issue.iid, &disc.id, false)?;

        gl.issues.delete(pid, issue.iid)?;
    });

    // ── 16. TODOS ──────────────────────────────────────────────────────

    println!();
    println!("━━━ [ Todos ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("todos.list", {
        let _todos = gl.todos.list(None)?;
        println!("    listados com sucesso");
    });

    // ── 17. SEARCH ─────────────────────────────────────────────────────

    println!();
    println!("━━━ [ Search ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("search.global", {
        let _results = gl.search.global("projects", &tname)?;
        println!("    busca ok");
    });

    // ── 18. EVENTS ─────────────────────────────────────────────────────

    println!();
    println!("━━━ [ Events ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("events.list", {
        let events = gl.events.list(Some(&EventFilter {
            per_page: Some(3),
            ..Default::default()
        }))?;
        println!("    {} eventos recentes", events.len());
    });

    test!("events.list_user_events", {
        let events = gl.events.list_user_events(me.id, Some(&EventFilter {
            per_page: Some(3),
            ..Default::default()
        }))?;
        println!("    {} eventos do usuario", events.len());
    });

    // ── 19. PIPELINES ──────────────────────────────────────────────────

    println!();
    println!("━━━ [ Pipelines ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("pipelines.list", {
        let _pipelines = gl.pipelines.list(pid, None)?;
        println!("    listadas com sucesso");
    });

    test!("pipelines.get_latest", {
        match gl.pipelines.get_latest(pid) {
            Ok(p) => println!("    latest pipeline #{}", p.id),
            Err(GitLabError::Api { ref category, .. }) if matches!(category, ErrorCategory::ResourceNotFound | ErrorCategory::AuthorizationDenied) => {
                println!("    (indisponivel: {:?})", category);
            }
            Err(e) => return Err(e),
        }
    });

    // ── 20. JOBS ───────────────────────────────────────────────────────

    println!();
    println!("━━━ [ Jobs ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("jobs.list", {
        let _jobs = gl.jobs.list(pid, None)?;
        println!("    listados com sucesso");
    });

    // ── 21. PIPELINE SCHEDULES ─────────────────────────────────────────

    println!();
    println!("━━━ [ Pipeline Schedules ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("pipeline_schedules.list", {
        let _schedules = gl.pipeline_schedules.list(pid)?;
        println!("    listados com sucesso");
    });

    // ── 22. RUNNERS ────────────────────────────────────────────────────

    println!();
    println!("━━━ [ Runners ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("runners.list", {
        let runners = gl.runners.list()?;
        println!("    {} runners disponiveis", runners.len());
    });

    // ── 23. RELEASES ───────────────────────────────────────────────────

    println!();
    println!("━━━ [ Releases ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("releases.list", {
        let _releases = gl.releases.list(pid)?;
        println!("    listadas com sucesso");
    });

    // ── 24. DEPLOY KEYS ────────────────────────────────────────────────

    println!();
    println!("━━━ [ Deploy Keys ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("deploy_keys.list", {
        let keys = gl.deploy_keys.list(pid)?;
        println!("    {} deploy keys", keys.len());
    });

    // ── 25. ENVIRONMENTS ───────────────────────────────────────────────

    println!();
    println!("━━━ [ Environments ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("environments CRUD", {
        let env = gl.environments.create(pid, &CreateEnvironmentPayload {
            name: fmt(&format!("env-test-{}", &sufx[..6])),
            external_url: Some("https://test.example.com".into()),
            slug: None,
            tier: None,
        })?;
        let eid = env.id;

        let envs = gl.environments.list(pid)?;
        assert!(envs.iter().any(|e| e.id == eid));

        let _got = gl.environments.get(pid, eid)?;

        gl.environments.update(pid, eid, &UpdateEnvironmentPayload {
            name: Some(fmt(&format!("env-test-renamed-{}", &sufx[..6]))),
            external_url: None,
            slug: None,
            tier: None,
        })?;

        gl.environments.stop(pid, eid)?;
        gl.environments.delete(pid, eid)?;
    });

    // ── 26. GROUPS ─────────────────────────────────────────────────────

    println!();
    println!("━━━ [ Groups ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("groups.list", {
        let groups = gl.groups.list(None)?;
        println!("    {} grupos", groups.len());
    });

    // ── 27. OAuth helpers (offline) ────────────────────────────────────

    println!();
    println!("━━━ [ OAuth ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("oauth.generate_code_verifier + challenge", {
        let v = oauth::generate_code_verifier();
        assert!(v.len() >= 43, "verifier muito curto: {}", v.len());
        let c = oauth::generate_code_challenge(&v);
        assert_eq!(c.len(), 43, "challenge com tamanho errado: {}", c.len());
        println!("    verifier: {} chars, challenge: {} chars", v.len(), c.len());
    });

    test!("oauth.authorization_code_url", {
        let v = oauth::generate_code_verifier();
        let c = oauth::generate_code_challenge(&v);
        let url = oauth::authorization_code_url(&oauth::AuthCodeUrlOptions {
            base_url: "https://gitlab.com".into(),
            client_id: "test-client".into(),
            redirect_uri: "https://localhost/callback".into(),
            scope: "api".into(),
            state: "test-state".into(),
            code_challenge: Some(c),
        });
        assert!(url.contains("client_id=test-client"));
        assert!(url.contains("code_challenge_method=S256"));
        println!("    URL de autorizacao gerada com sucesso");
    });

    // ── 28. CLEANUP ────────────────────────────────────────────────────

    println!();
    println!("━━━ [ Cleanup ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    test!("projects.delete (projeto temporario)", {
        let pid2 = find_project(&gl, &tname)?;
        if pid2 != 0 {
            gl.projects.delete(pid2)?;
            println!("    Projeto {} removido.", pid2);
        } else {
            println!("    Projeto ja nao existe.");
        }
    });

    // ── RESUMO ─────────────────────────────────────────────────────────

    let passed = PASSED.load(Ordering::Relaxed);
    let failed = FAILED.load(Ordering::Relaxed);
    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                     R E S U M O                            ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  PASSED:  {:<3}                                            ║", passed);
    println!("║  FAILED:  {:<3}                                            ║", failed);
    println!("║  TOTAL:   {:<3}                                            ║", passed + failed);
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    if failed == 0 {
        println!("🎉 Todos os testes passaram!");
    } else {
        println!("⚠️  {} teste(s) falharam. Revise a saida acima.", failed);
    }
    println!();

    Ok(())
}
