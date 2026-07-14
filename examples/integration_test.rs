//! Teste de integração completo contra um GitLab real.
//!
//! Uso:
//!   export GITLAB_TOKEN="glpat-xxxx"
//!   export GITLAB_URL="https://gitlab.com"
//!   cargo run --example integration_test
//!
//! Modo seguro (read-only): não cria, altera ou remove dados.

use gitlab_wrapper::*;
use std::sync::atomic::{AtomicU32, Ordering};

static PASSED: AtomicU32 = AtomicU32::new(0);
static FAILED: AtomicU32 = AtomicU32::new(0);

macro_rules! check {
    ($name:expr, $body:expr) => {{
        print!("  {:<45}", $name);
        match $body.await {
            Ok(_) => {
                PASSED.fetch_add(1, Ordering::Relaxed);
                println!(" ✅");
            }
            Err(e) => {
                FAILED.fetch_add(1, Ordering::Relaxed);
                println!(" ❌ {}", e);
            }
        }
    }};
}

macro_rules! check_count {
    ($name:expr, $body:expr) => {{
        print!("  {:<45}", $name);
        match $body.await {
            Ok(items) => {
                PASSED.fetch_add(1, Ordering::Relaxed);
                println!(" ✅ {} itens", items.len());
            }
            Err(e) => {
                FAILED.fetch_add(1, Ordering::Relaxed);
                println!(" ❌ {}", e);
            }
        }
    }};
}

fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║       gitlab-wrapper-rs — Teste de Integração Real             ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    let url = std::env::var("GITLAB_URL").unwrap_or_else(|_| "https://gitlab.com".into());
    let token = std::env::var("GITLAB_TOKEN").expect("GITLAB_TOKEN é obrigatório");

    let gl = GitLabClient::new(GitLabConfig {
        base_url: url.clone(),
        token: Some(token),
        auth_method: Some(AuthMethod::Bearer),
        timeout: Some(std::time::Duration::from_secs(15)),
        ..Default::default()
    })?;

    println!("  Conectado a: {}", url);
    println!();

    // ── 1. USER ──────────────────────────────────────────────────────────
    println!("{}", bold("━━━ [ 1/8  Users ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"));
    let me = gl.users.get_current().await?;
    println!("         👤 {} (@{})", me.name, me.username);
    println!("         🆔 ID: {}", me.id);
    println!("         📧 Email: {:?}", me.email.as_deref().unwrap_or("não informado"));
    println!("         🔗 Web: {:?}", me.web_url.as_deref().unwrap_or("n/a"));
    PASSED.fetch_add(1, Ordering::Relaxed);

    let me_id = me.id;

    check!("users.list (filtro por username)", async {
        gl.users
            .list(Some(&UserFilter { username: Some("root".into()), ..Default::default() }))
            .await
    });
    check!("users.status", async { gl.users.status(me_id).await });
    check!("users.preferences", async { gl.users.preferences().await });

    // ── 2. PROJECTS ──────────────────────────────────────────────────────
    println!();
    println!("{}", bold("━━━ [ 2/8  Projects ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"));

    let projects = gl
        .projects
        .list(Some(&ProjectFilter {
            membership: Some(true),
            per_page: Some(5),
            ..Default::default()
        }))
        .await?;
    println!("         ✅ {} projetos encontrados", projects.len());
    PASSED.fetch_add(1, Ordering::Relaxed);

    let project_id = projects.first().map(|p| p.id).unwrap_or(0);

    if project_id > 0 {
        let full_path = match gl.projects.get(project_id).await {
            Ok(p) => {
                println!("         📦 {} (ID: {})", p.name, p.id);
                PASSED.fetch_add(1, Ordering::Relaxed);
                format!("{}/{}", p.namespace.as_ref().map(|n| &n.path).unwrap_or(&p.path), p.path)
            }
            Err(e) => {
                println!("         ❌ projects.get: {}", e);
                FAILED.fetch_add(1, Ordering::Relaxed);
                String::new()
            }
        };

        if !full_path.is_empty() {
            check!("projects.get_by_path", async { gl.projects.get_by_path(&full_path).await });
        }
    }

    // ── 3. GROUPS ────────────────────────────────────────────────────────
    println!();
    println!("{}", bold("━━━ [ 3/8  Groups ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"));
    check_count!("groups.list", async {
        gl.groups.list(Some(&GroupFilter { per_page: Some(5), ..Default::default() })).await
    });

    // ── 4. PROJECT RESOURCES ─────────────────────────────────────────────
    println!();
    if project_id > 0 {
        println!("{}", bold("━━━ [ 4/8  Project Resources ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━"));
        println!("         Projeto ID: {}", project_id);

        check_count!("branches.list", async { gl.branches.list(project_id).await });
        check!("branches.get (main)", async { gl.branches.get(project_id, "main").await });
        check_count!("tags.list", async { gl.tags.list(project_id).await });
        check_count!("commits.list", async {
            gl.commits
                .list(project_id, Some(&CommitFilter { per_page: Some(3), ..Default::default() }))
                .await
        });
        check_count!("issues.list (by project)", async {
            gl.issues
                .list_for_project(
                    project_id,
                    Some(&IssueFilter { per_page: Some(3), ..Default::default() }),
                )
                .await
        });
        check_count!("merge_requests.list (by project)", async {
            gl.merge_requests
                .list_for_project(
                    project_id,
                    Some(&MergeRequestFilter { per_page: Some(3), ..Default::default() }),
                )
                .await
        });
        check_count!("labels.list", async { gl.labels.list_project_labels(project_id).await });
        check_count!("milestones.list", async {
            gl.milestones.list_project_milestones(project_id, None).await
        });
        check_count!("members.list", async { gl.members.list_project_members(project_id).await });
        check_count!("events.list (project)", async {
            gl.events.list(Some(&EventFilter { per_page: Some(3), ..Default::default() })).await
        });
        check_count!("pipelines.list", async {
            gl.pipelines
                .list(project_id, Some(&PipelineFilter { per_page: Some(3), ..Default::default() }))
                .await
        });
        check_count!("jobs.list", async {
            gl.jobs
                .list(project_id, Some(&JobFilter { per_page: Some(3), ..Default::default() }))
                .await
        });
        check_count!("pipeline_schedules.list", async {
            gl.pipeline_schedules.list(project_id).await
        });
        check_count!("environments.list", async { gl.environments.list(project_id).await });
        check_count!("deploy_keys.list", async { gl.deploy_keys.list(project_id).await });
        check_count!("releases.list", async { gl.releases.list(project_id).await });

        // Repository Files
        check!("repository_files.get (README.md)", async {
            gl.repository_files.get(project_id, "README.md", "main").await
        });
        check!("repository_files.raw (README.md)", async {
            gl.repository_files.raw(project_id, "README.md", "main").await
        });
        check!("repository_files.blame (README.md)", async {
            gl.repository_files.blame(project_id, "README.md", "main").await
        });

        // Wikis
        check_count!("wikis.list", async { gl.wikis.list(project_id).await });
    } else {
        println!("         ⚠️  Nenhum projeto com permissão — pulando recursos de projeto");
    }

    // ── 5. GLOBAL RESOURCES ──────────────────────────────────────────────
    println!();
    println!("{}", bold("━━━ [ 5/8  Global Resources ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"));
    check_count!("issues.list (global)", async {
        gl.issues.list(Some(&IssueFilter { per_page: Some(3), ..Default::default() })).await
    });
    check_count!("merge_requests.list (global)", async {
        gl.merge_requests
            .list(Some(&MergeRequestFilter { per_page: Some(3), ..Default::default() }))
            .await
    });
    check_count!("todos.list", async {
        gl.todos.list(Some(&TodoFilter { per_page: Some(3), ..Default::default() })).await
    });
    check_count!("events.list (global)", async {
        gl.events.list(Some(&EventFilter { per_page: Some(3), ..Default::default() })).await
    });

    // ── 6. SEARCH ─────────────────────────────────────────────────────────
    println!();
    println!("{}", bold("━━━ [ 6/8  Search ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"));
    check_count!("search.global (projects)", async { gl.search.global("projects", "test").await });

    // ── 7. NOTES / DISCUSSIONS ───────────────────────────────────────────
    println!();
    if project_id > 0 {
        println!("{}", bold("━━━ [ 7/8  Notes & Discussions ] ━━━━━━━━━━━━━━━━━━━━━━━━━━"));
        check_count!("notes.list (on first issue)", async {
            let issues = gl
                .issues
                .list_for_project(
                    project_id,
                    Some(&IssueFilter { per_page: Some(1), ..Default::default() }),
                )
                .await?;
            if let Some(issue) = issues.first() {
                gl.notes.list_issue_notes(project_id, issue.iid).await
            } else {
                Ok(Vec::new())
            }
        });
    }

    // ── 8. RUNNERS ───────────────────────────────────────────────────────
    println!();
    println!("{}", bold("━━━ [ 8/8  Runners ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"));
    check_count!("runners.list", async { gl.runners.list().await });

    // ── SUMMARY ──────────────────────────────────────────────────────────
    let passed = PASSED.load(Ordering::Relaxed);
    let failed = FAILED.load(Ordering::Relaxed);
    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           Teste de Integração Concluído                         ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║  ✅ PASSED:  {:<3}                                              ║", passed);
    println!("║  ❌ FAILED:  {:<3}                                              ║", failed);
    println!(
        "║  📦 TOTAL:   {:<3}                                              ║",
        passed + failed
    );
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    if failed > 0 { Err(format!("{} teste(s) falharam", failed).into()) } else { Ok(()) }
}
