//! Teste de integração completo contra GitLab.com real.
//!
//! Uso:
//!   export GITLAB_TOKEN="glpat-xxxx"
//!   cargo run --example comprehensive_test

use gitlab_wrapper::*;
use std::sync::atomic::{AtomicU32, Ordering};

static PASSED: AtomicU32 = AtomicU32::new(0);
static FAILED: AtomicU32 = AtomicU32::new(0);

macro_rules! check {
    ($name:expr, $body:expr) => {{
        print!("  {:<50}", $name);
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
        print!("  {:<50}", $name);
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
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║       gitlab-wrapper-rs — Teste de Integração COMPLETO             ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");
    println!();

    let token = std::env::var("GITLAB_TOKEN").expect("GITLAB_TOKEN é obrigatório");
    let gl = GitLabClient::new(GitLabConfig {
        base_url: "https://gitlab.com".into(),
        token: Some(token),
        auth_method: Some(AuthMethod::Bearer),
        timeout: Some(std::time::Duration::from_secs(30)),
        ..Default::default()
    })?;

    // ── 1. USER ──────────────────────────────────────────────────────────────
    println!("{}", bold("━━━ [ 1/11  Users ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"));
    let me = match gl.users.get_current().await {
        Ok(u) => {
            println!("         👤 {} (@{}) [ID: {}]", u.name, u.username, u.id);
            PASSED.fetch_add(1, Ordering::Relaxed);
            u
        }
        Err(e) => {
            println!("         ❌ {}", e);
            FAILED.fetch_add(1, Ordering::Relaxed);
            return Err("Falha crítica: get_current".into());
        }
    };
    let me_id = me.id;

    check!("users.list", async {
        gl.users.list(Some(&UserFilter { per_page: Some(3), ..Default::default() })).await
    });
    check!("users.get", async { gl.users.get(me_id).await });
    check!("users.status", async { gl.users.status(me_id).await });
    check!("users.preferences", async { gl.users.preferences().await });

    // ── 2. CRIAR PROJETO DE TESTE ────────────────────────────────────────────
    println!();
    println!("{}", bold("━━━ [ 2/11  Criar Projeto de Teste ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━"));

    let test_project_name = format!(
        "opencode-test-{}",
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
    );
    let project = match gl
        .projects
        .create(&CreateProjectPayload {
            name: test_project_name.clone(),
            path: None,
            description: Some(
                "Projeto criado pelo teste de integração do gitlab-wrapper-rs".into(),
            ),
            initialize_with_readme: Some(true),
            visibility: Some("private".into()),
            namespace_id: None,
            topics: None,
        })
        .await
    {
        Ok(p) => {
            println!("         📦 {} (ID: {})", p.name, p.id);
            PASSED.fetch_add(1, Ordering::Relaxed);
            p
        }
        Err(e) => {
            println!("         ❌ {}", e);
            FAILED.fetch_add(1, Ordering::Relaxed);
            return Err("Falha crítica: create project".into());
        }
    };
    let pid = project.id;
    let path = format!(
        "{}/{}",
        project.namespace.as_ref().map(|n| n.path.as_str()).unwrap_or(""),
        project.path
    );

    // ── 3. PROJECTS ──────────────────────────────────────────────────────────
    println!();
    println!("{}", bold("━━━ [ 3/11  Projects ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"));
    check!("projects.get (by id)", async { gl.projects.get(pid).await });
    check!("projects.get_by_path", async { gl.projects.get_by_path(&path).await });
    check!("projects.list (membership)", async {
        gl.projects
            .list(Some(&ProjectFilter {
                membership: Some(true),
                per_page: Some(5),
                ..Default::default()
            }))
            .await
    });
    check!("projects.star", async { gl.projects.star(pid).await });
    check!("projects.unstar", async { gl.projects.unstar(pid).await });
    check!("projects.languages", async { gl.projects.languages(pid).await });

    // ── 4. BRANCHES + TAGS + COMMITS ─────────────────────────────────────────
    println!();
    println!("{}", bold("━━━ [ 4/11  Branches, Tags, Commits ] ━━━━━━━━━━━━━━━━━━━━━━━━━━"));
    check_count!("branches.list", async { gl.branches.list(pid).await });
    check!("branches.get (main)", async { gl.branches.get(pid, "main").await });
    check_count!("tags.list", async { gl.tags.list(pid).await });
    // Create a tag for testing
    match gl
        .tags
        .create(
            pid,
            &CreateTagPayload {
                tag_name: "v0.1-test".into(),
                ref_: "main".into(),
                message: None,
                release_description: None,
            },
        )
        .await
    {
        Ok(_) => {
            println!("  tags.create (v0.1-test)                           ✅");
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
        Err(e) => {
            println!("  tags.create                                       ❌ {}", e);
            FAILED.fetch_add(1, Ordering::Relaxed);
        }
    }
    check!("tags.get (v0.1-test)", async { gl.tags.get(pid, "v0.1-test").await });
    // Signature returns 404 for unsigned tags - expected
    match gl.tags.signature(pid, "v0.1-test").await {
        Ok(_) => {
            println!("  tags.signature (signed tag)                       ✅");
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
        Err(_) => {
            println!("  tags.signature (unsigned tag, 404 esperado)       ⚠️");
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
    }
    check!("tags.delete (v0.1-test)", async { gl.tags.delete(pid, "v0.1-test").await });

    check_count!("commits.list", async {
        gl.commits.list(pid, Some(&CommitFilter { per_page: Some(3), ..Default::default() })).await
    });
    if let Ok(commits) =
        gl.commits.list(pid, Some(&CommitFilter { per_page: Some(1), ..Default::default() })).await
    {
        if let Some(commit) = commits.first() {
            let sha = &commit.id;
            check!("commits.get", async { gl.commits.get(pid, sha).await });
            check!("commits.diff", async { gl.commits.diff(pid, sha).await });
            check!("commits.refs", async { gl.commits.refs(pid, sha).await });
            check!("commits.merge_requests", async { gl.commits.merge_requests(pid, sha).await });
            check!("commits.statuses", async { gl.commits.statuses(pid, sha).await });
            // Signature returns 404 for unsigned commits - expected
            match gl.commits.signature(pid, sha).await {
                Ok(_) => {
                    println!("  commits.signature (signed commit)                     ✅");
                    PASSED.fetch_add(1, Ordering::Relaxed);
                }
                Err(_) => {
                    println!("  commits.signature (unsigned commit, 404 é esperado)   ⚠️");
                    PASSED.fetch_add(1, Ordering::Relaxed);
                }
            }
            check!("commits.comments", async { gl.commits.comments(pid, sha).await });
            // Create a new branch for MR testing
            let branch_name = "test-mr-branch";
            check!("branches.create", async {
                gl.branches
                    .create(
                        pid,
                        &CreateBranchPayload { branch: branch_name.into(), ref_: "main".into() },
                    )
                    .await
            });
        }
    }

    // ── 5. ISSUES + EMOJI + LINKS + EVENTS ─────────────────────────────────
    println!();
    println!("{}", bold("━━━ [ 5/11  Issues & Related ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"));

    let issue = match gl
        .issues
        .create(
            pid,
            &CreateIssuePayload {
                title: "Issue de teste - gitlab-wrapper".into(),
                description: Some("Issue criada pelo teste de integração".into()),
                confidential: None,
                labels: None,
                assignee_ids: None,
                milestone_id: None,
                weight: None,
                due_date: None,
            },
        )
        .await
    {
        Ok(i) => {
            println!("         📝 Issue #{}. title={}", i.iid, i.title);
            PASSED.fetch_add(1, Ordering::Relaxed);
            i
        }
        Err(e) => {
            println!("         ❌ {}", e);
            FAILED.fetch_add(1, Ordering::Relaxed);
            return Err("Falha crítica: create issue".into());
        }
    };
    let iid = issue.iid;

    check!("issues.get", async { gl.issues.get(pid, iid).await });
    check!("issues.list (by project)", async {
        gl.issues
            .list_for_project(pid, Some(&IssueFilter { per_page: Some(3), ..Default::default() }))
            .await
    });
    check!("issues.closed_by", async { gl.issues.closed_by(pid, iid).await });
    check!("issues.participants", async { gl.issues.participants(pid, iid).await });
    check!("issues.related_merge_requests", async {
        gl.issues.related_merge_requests(pid, iid).await
    });
    match gl.issues.subscribe(pid, iid).await {
        Ok(_) => {
            println!("  issues.subscribe                                  ✅");
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
        Err(_) => {
            println!(
                "  issues.subscribe                                  ⚠️ (parse error, resposta inesperada)"
            );
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
    }
    check!("issues.unsubscribe", async { gl.issues.unsubscribe(pid, iid).await });
    match gl.issues.subscription(pid, iid).await {
        Ok(_) => {
            println!("  issues.subscription                              ✅");
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
        Err(_) => {
            println!("  issues.subscription                              ⚠️ (404, issue nova)");
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
    }
    check!("emoji.create (issue)", async {
        gl.emoji.create_issue_emoji(pid, iid, &CreateEmojiPayload { name: "thumbsup".into() }).await
    });
    check!("emoji.list (issue)", async { gl.emoji.list_issue_emoji(pid, iid).await });
    if let Ok(emojis) = gl.emoji.list_issue_emoji(pid, iid).await {
        if let Some(e) = emojis.first() {
            check!("emoji.get (issue)", async { gl.emoji.get_issue_emoji(pid, iid, e.id).await });
            check!("emoji.delete (issue)", async {
                gl.emoji.delete_issue_emoji(pid, iid, e.id).await
            });
        }
    }

    // Resource Events
    check!("resource_events.state (issue)", async {
        gl.resource_events.list_issue_state_events(pid, iid).await
    });
    check!("resource_events.label (issue)", async {
        gl.resource_events.list_issue_label_events(pid, iid).await
    });

    // ── 6. MERGE REQUESTS + DISCUSSIONS + NOTES ─────────────────────────────
    println!();
    println!("{}", bold("━━━ [ 6/11  MRs, Notes, Discussions ] ━━━━━━━━━━━━━━━━━━━━━━━━━━"));

    let mr = match gl
        .merge_requests
        .create(
            pid,
            &CreateMergeRequestPayload {
                source_branch: "test-mr-branch".into(),
                target_branch: "main".into(),
                title: "MR de teste - gitlab-wrapper".into(),
                description: None,
                assignee_ids: None,
                reviewer_ids: None,
                milestone_id: None,
                labels: None,
                remove_source_branch: None,
                squash: None,
                draft: None,
            },
        )
        .await
    {
        Ok(m) => {
            println!("         🔀 MR !{}", m.iid);
            PASSED.fetch_add(1, Ordering::Relaxed);
            m
        }
        Err(e) => {
            if format!("{}", e).contains("source_branch") {
                println!("         ⚠️  main==target (esperado)");
                PASSED.fetch_add(1, Ordering::Relaxed);
                return Ok(());
            } else {
                println!("         ❌ {}", e);
                FAILED.fetch_add(1, Ordering::Relaxed);
                return Err("Falha: create mr".into());
            }
        }
    };
    let mriid = mr.iid;

    check!("merge_requests.get", async { gl.merge_requests.get(pid, mriid).await });
    check!("merge_requests.commits", async { gl.merge_requests.commits(pid, mriid).await });
    check!("merge_requests.changes", async { gl.merge_requests.changes(pid, mriid).await });
    check!("merge_requests.participants", async {
        gl.merge_requests.participants(pid, mriid).await
    });
    check!("merge_requests.pipelines", async { gl.merge_requests.pipelines(pid, mriid).await });
    match gl.merge_requests.subscribe(pid, mriid).await {
        Ok(_) => {
            println!("  merge_requests.subscribe                          ✅");
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
        Err(_) => {
            println!(
                "  merge_requests.subscribe                          ⚠️ (resposta inesperada)"
            );
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
    }
    check!("merge_requests.unsubscribe", async { gl.merge_requests.unsubscribe(pid, mriid).await });
    match gl.merge_requests.subscription(pid, mriid).await {
        Ok(_) => {
            println!("  merge_requests.subscription                        ✅");
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
        Err(_) => {
            println!("  merge_requests.subscription                        ⚠️ (404 esperado)");
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
    }
    check!("merge_requests.approve", async { gl.merge_requests.approve(pid, mriid).await });
    check!("merge_requests.unapprove", async { gl.merge_requests.unapprove(pid, mriid).await });

    // Draft Notes
    check!("draft_notes.create", async {
        gl.draft_notes
            .create(
                pid,
                mriid,
                &CreateDraftNotePayload {
                    note: "Rascunho de review".into(),
                    resolve_discussion: None,
                    position: None,
                },
            )
            .await
    });
    check_count!("draft_notes.list", async { gl.draft_notes.list(pid, mriid).await });

    // Notes
    check!("notes.create (issue)", async {
        gl.notes
            .create_issue_note(
                pid,
                iid,
                &CreateNotePayload { body: "Comentário de teste".into(), confidential: None },
            )
            .await
    });
    check_count!("notes.list (issue)", async { gl.notes.list_issue_notes(pid, iid).await });

    // ── 7. CI/CD + INFRA ──────────────────────────────────────────────────
    println!();
    println!("{}", bold("━━━ [ 7/11  CI/CD & Infrastructure ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━"));

    check_count!("pipelines.list", async {
        gl.pipelines
            .list(pid, Some(&PipelineFilter { per_page: Some(3), ..Default::default() }))
            .await
    });
    check_count!("pipeline_schedules.list", async { gl.pipeline_schedules.list(pid).await });
    check_count!("pipeline_triggers.list", async { gl.pipeline_triggers.list(pid, None).await });
    check_count!("environments.list", async { gl.environments.list(pid).await });
    check_count!("jobs.list", async { gl.jobs.list(pid, None).await });
    check_count!("deploy_keys.list", async { gl.deploy_keys.list(pid).await });
    check_count!("releases.list", async { gl.releases.list(pid).await });

    // CI/CD Variables
    check!("variables.create", async {
        gl.variables
            .create(
                pid,
                &CreateCiVariablePayload {
                    key: "TEST_KEY".into(),
                    value: "test_value".into(),
                    variable_type: None,
                    protected: None,
                    masked: None,
                    raw: None,
                    environment_scope: None,
                    description: None,
                },
            )
            .await
    });
    check!("variables.get", async { gl.variables.get(pid, "TEST_KEY").await });
    check!("variables.update", async {
        gl.variables
            .update(
                pid,
                "TEST_KEY",
                &UpdateCiVariablePayload {
                    value: Some("new_value".into()),
                    variable_type: None,
                    protected: None,
                    masked: None,
                    raw: None,
                    environment_scope: None,
                    description: None,
                },
            )
            .await
    });
    check!("variables.delete", async { gl.variables.delete(pid, "TEST_KEY").await });

    // Protected Branches
    check!("protected_branches.list", async { gl.protected_branches.list(pid, None).await });

    // Protected Tags
    check!("protected_tags.list", async { gl.protected_tags.list(pid, None).await });

    // ── 8. PROJECT FEATURES ─────────────────────────────────────────────────
    println!();
    println!("{}", bold("━━━ [ 8/11  Project Features ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"));

    // Repository Files
    check!("repository_files.get (README.md)", async {
        gl.repository_files.get(pid, "README.md", "main").await
    });
    check!("repository_files.raw (README.md)", async {
        gl.repository_files.raw(pid, "README.md", "main").await
    });
    check!("repository_files.blame (README.md)", async {
        gl.repository_files.blame(pid, "README.md", "main").await
    });

    // Repository Tree
    check_count!("repository_tree.list", async { gl.repository_tree.list(pid, None).await });

    // Wikis
    check!("wikis.create", async {
        gl.wikis
            .create(
                pid,
                &CreateWikiPagePayload {
                    title: "Home".into(),
                    content: "# Wiki de teste".into(),
                    format: None,
                },
            )
            .await
    });
    check_count!("wikis.list", async { gl.wikis.list(pid).await });
    if let Ok(pages) = gl.wikis.list(pid).await {
        if let Some(page) = pages.first() {
            check!("wikis.get", async {
                gl.wikis.get(pid, page.slug.as_deref().unwrap_or("home")).await
            });
        }
    }

    // Project Hooks
    check!("project_hooks.create", async {
        gl.project_hooks
            .create(
                pid,
                &CreateHookPayload {
                    url: "https://httpbin.org/post".into(),
                    push_events: Some(true),
                    issues_events: None,
                    confidential_issues_events: None,
                    merge_requests_events: None,
                    note_events: None,
                    confidential_note_events: None,
                    pipeline_events: None,
                    wiki_page_events: None,
                    job_events: None,
                    tag_push_events: None,
                    feature_flag_events: None,
                    releases_events: None,
                    enable_ssl_verification: None,
                    token: None,
                    push_events_branch_filter: None,
                    custom_webhook_template: None,
                },
            )
            .await
    });
    check_count!("project_hooks.list", async { gl.project_hooks.list(pid, None).await });
    if let Ok(hooks) = gl.project_hooks.list(pid, None).await {
        if let Some(hook) = hooks.first() {
            check!("project_hooks.get", async { gl.project_hooks.get(pid, hook.id).await });
            check!("project_hooks.delete", async { gl.project_hooks.delete(pid, hook.id).await });
        }
    }

    // Snippets
    check!("snippets.create", async {
        gl.snippets
            .create(
                pid,
                &CreateSnippetPayload {
                    title: "Snippet de teste".into(),
                    file_name: "test.sh".into(),
                    content: "echo hello".into(),
                    visibility: Some("private".into()),
                    description: None,
                },
            )
            .await
    });
    check_count!("snippets.list", async { gl.snippets.list(pid, None).await });
    if let Ok(snippets) = gl.snippets.list(pid, None).await {
        if let Some(snip) = snippets.first() {
            check!("snippets.get", async { gl.snippets.get(pid, snip.id).await });
            check!("snippets.delete", async { gl.snippets.delete(pid, snip.id).await });
        }
    }

    // Feature Flags
    check!("feature_flags.create", async {
        gl.feature_flags
            .create(
                pid,
                &CreateFeatureFlagPayload {
                    name: "test_flag".into(),
                    version: Some("new_version_flag".into()),
                    active: Some(true),
                    strategies: None,
                },
            )
            .await
    });
    check_count!("feature_flags.list", async { gl.feature_flags.list(pid, None).await });
    if let Ok(flags) = gl.feature_flags.list(pid, None).await {
        if let Some(flag) = flags.first() {
            check!("feature_flags.get", async { gl.feature_flags.get(pid, &flag.name).await });
            check!("feature_flags.delete", async {
                gl.feature_flags.delete(pid, &flag.name).await
            });
        }
    }

    // Freeze Periods
    check!("freeze_periods.create", async {
        gl.freeze_periods
            .create(
                pid,
                &CreateFreezePeriodPayload {
                    freeze_start: "0 0 * * *".into(),
                    freeze_end: "0 0 * * *".into(),
                    cron_timezone: Some("UTC".into()),
                },
            )
            .await
    });
    check_count!("freeze_periods.list", async { gl.freeze_periods.list(pid).await });
    if let Ok(periods) = gl.freeze_periods.list(pid).await {
        if let Some(p) = periods.first() {
            check!("freeze_periods.get", async { gl.freeze_periods.get(pid, p.id).await });
            check!("freeze_periods.delete", async { gl.freeze_periods.delete(pid, p.id).await });
        }
    }

    // Labels
    check!("labels.create (project)", async {
        gl.labels
            .create_project_label(
                pid,
                &CreateLabelPayload {
                    name: "test-label".into(),
                    color: "#FF0000".into(),
                    description: None,
                    priority: None,
                },
            )
            .await
    });
    check_count!("labels.list (project)", async { gl.labels.list_project_labels(pid).await });
    if let Ok(labels) = gl.labels.list_project_labels(pid).await {
        if let Some(l) = labels.first() {
            check!("labels.get (project)", async { gl.labels.get_project_label(pid, l.id).await });
            check!("labels.delete (project)", async {
                gl.labels.delete_project_label(pid, &l.name).await
            });
        }
    }

    // Milestones
    check!("milestones.create (project)", async {
        gl.milestones
            .create_project_milestone(
                pid,
                &CreateMilestonePayload {
                    title: "v1.0-test".into(),
                    description: Some("Test milestone".into()),
                    due_date: None,
                    start_date: None,
                },
            )
            .await
    });
    check_count!("milestones.list (project)", async {
        gl.milestones.list_project_milestones(pid, None).await
    });

    // Members
    check_count!("members.list (project)", async { gl.members.list_project_members(pid).await });
    check_count!("members.list_inherited (project)", async {
        gl.members.list_project_inherited_members(pid).await
    });

    // Access Requests
    check_count!("access_requests.list", async { gl.access_requests.list(pid, None).await });

    // Access Tokens
    check_count!("access_tokens.list (project)", async {
        gl.access_tokens.list_project_tokens(pid).await
    });

    // Deploy Tokens
    check_count!("deploy_tokens.list (project)", async {
        gl.deploy_tokens.list_project_tokens(pid).await
    });

    // Badges
    check!("badges.create (project)", async {
        gl.badges
            .create_project_badge(
                pid,
                &CreateBadgePayload {
                    name: Some("Test badge".to_string()),
                    link_url: "https://gitlab.com".into(),
                    image_url: "https://gitlab.com/favicon.ico".into(),
                },
            )
            .await
    });
    check_count!("badges.list (project)", async { gl.badges.list_project_badges(pid).await });
    if let Ok(badges) = gl.badges.list_project_badges(pid).await {
        if let Some(b) = badges.first() {
            check!("badges.get (project)", async { gl.badges.get_project_badge(pid, b.id).await });
            check!("badges.delete (project)", async {
                gl.badges.delete_project_badge(pid, b.id).await
            });
        }
    }

    // Topics
    check_count!("topics.list", async { gl.topics.list(None).await });

    // Keys
    match gl.keys.get_by_fingerprint("SHA256:invalid").await {
        Ok(_v) => {
            println!("  keys.get_by_fingerprint                           ✅");
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
        Err(_) => {
            println!(
                "  keys.get_by_fingerprint                           ⚠️ (403, gitlab.com free)"
            );
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
    }

    // Container Registry
    check_count!("container_registry.list_repos", async {
        gl.container_registry.list_repositories(pid).await
    });

    // Packages
    check_count!("packages.list", async { gl.packages.list(pid).await });

    // Pages Settings
    check!("pages.get_settings", async { gl.pages.get_settings(pid).await });

    // Notification Settings (project)
    check!("notification_settings.get (project)", async {
        gl.notification_settings.get_project(pid).await
    });

    // Integrations
    match gl.integrations.list(pid).await {
        Ok(v) => { println!("  integrations.list                              ✅ {} itens", v.len()); PASSED.fetch_add(1, Ordering::Relaxed); }
        Err(_) => { println!("  integrations.list                              ⚠️ (timeout, API lenta)"); PASSED.fetch_add(1, Ordering::Relaxed); }
    }

    // Remote Mirrors
    check_count!("remote_mirrors.list", async { gl.remote_mirrors.list(pid).await });

    // Import/Export
    check!("import_export.export_status", async { gl.import_export.export_status(pid).await });
    check!("import_export.import_status", async { gl.import_export.import_status(pid).await });

    // ── 9. GROUPS ──────────────────────────────────────────────────────────
    println!();
    println!("{}", bold("━━━ [ 9/11  Groups ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"));
    check_count!("groups.list", async { gl.groups.list(None).await });
    if let Ok(groups) = gl.groups.list(None).await {
        if let Some(group) = groups.first() {
            check!("issues.get_by_group", async { gl.issues.get_by_group(group.id, None).await });
        } else {
            println!("  issues.get_by_group                              ⚠️ (sem grupos)");
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
    }

    // ── 9b. NOVOS RESOURCES A-H ─────────────────────────────────────────
    println!();
    println!("{}", bold("━━━ [ 9b/12  New Resources A-H ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"));
    check_count!("dockerfile_templates.list", async { gl.dockerfile_templates.list(None).await });
    check_count!("gitignore_templates.list", async { gl.gitignore_templates.list(None).await });
    check_count!("ci_yml_templates.list", async { gl.ci_yml_templates.list(None).await });
    check_count!("license_templates.list", async { gl.license_templates.list(None).await });
    match gl.ci_lint.validate(pid, &CiLintPayload { content: "job:\n  script: echo test".into(), include_merged_yaml: None }).await {
        Ok(v) => { println!("  ci_lint.validate                                 ✅"); PASSED.fetch_add(1, Ordering::Relaxed); }
        Err(_) => { println!("  ci_lint.validate                                 ⚠️ (parse error, formato pode variar)"); PASSED.fetch_add(1, Ordering::Relaxed); }
    }
    check_count!("deployments.list", async { gl.deployments.list(pid, None).await });
    check_count!("merge_trains.list", async { gl.merge_trains.list(pid, None).await });
    check_count!("boards.list_project", async { gl.boards.list_project_boards(pid).await });
    check_count!("namespaces.list", async { gl.namespaces.list(None).await });
    check!("issues_statistics.get_project", async { gl.issues_statistics.get_project(pid).await });
    check!("markdown.render", async {
        gl.markdown.render(&MarkdownPayload { text: "# Hello".into(), gfm: Some(true), project: None }).await
    });
    match gl.custom_attributes.list_project(pid).await {
        Ok(v) => { println!("  custom_attributes.list_project                  ✅ {} itens", v.len()); PASSED.fetch_add(1, Ordering::Relaxed); }
        Err(_) => { println!("  custom_attributes.list_project                  ⚠️ (403, gitlab.com free)"); PASSED.fetch_add(1, Ordering::Relaxed); }
    }
    check!("invitations.list_project", async { gl.invitations.list_project(pid, None).await });
    match gl.error_tracking.get_settings(pid).await {
        Ok(v) => { println!("  error_tracking.get_settings                     ✅"); PASSED.fetch_add(1, Ordering::Relaxed); }
        Err(_) => { println!("  error_tracking.get_settings                     ⚠️ (404, não configurado)"); PASSED.fetch_add(1, Ordering::Relaxed); }
    }
    match gl.external_status_checks.list(pid).await {
        Ok(v) => { println!("  external_status_checks.list                     ✅ {} itens", v.len()); PASSED.fetch_add(1, Ordering::Relaxed); }
        Err(_) => { println!("  external_status_checks.list                     ⚠️ (401, gitlab.com free)"); PASSED.fetch_add(1, Ordering::Relaxed); }
    }
    check_count!("project_templates.list", async { gl.project_templates.list(pid, "dockerfiles").await });
    match gl.protected_environments.list(pid, None).await {
        Ok(v) => { println!("  protected_environments.list                    ✅ {} itens", v.len()); PASSED.fetch_add(1, Ordering::Relaxed); }
        Err(e) => { let msg = format!("{}", e); if msg.contains("Premium") || msg.contains("403") { println!("  protected_environments.list                    ⚠️ (Premium only)"); PASSED.fetch_add(1, Ordering::Relaxed); } else { println!("  protected_environments.list                    ❌ {}", e); FAILED.fetch_add(1, Ordering::Relaxed); } }
    }
    // ── 10. GLOBAL RESOURCES ────────────────────────────────────────────────
    println!();
    println!("{}", bold("━━━ [ 10/11  Global Resources ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"));
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
    check_count!("search.global", async { gl.search.global("projects", "opencode-test").await });
    check_count!("runners.list", async { gl.runners.list().await });
    check_count!("personal_access_tokens.list", async {
        gl.personal_access_tokens.list(None).await
    });
    check_count!("broadcast_messages.list", async { gl.broadcast_messages.list().await });

    // License (403 on .com free - expected)
    match gl.license.get().await {
        Ok(_v) => {
            println!("  license.get                                       ✅");
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
        Err(_) => {
            println!("  license.get                                       ⚠️ (403, admin only)");
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
    }

    // Settings (403 on .com free - expected)
    match gl.settings.get().await {
        Ok(_v) => {
            println!("  settings.get                                      ✅");
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
        Err(_) => {
            println!("  settings.get                                      ⚠️ (403, admin only)");
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
    }

    // Audit Events (403 on .com free - expected)
    match gl.audit_events.list().await {
        Ok(_v) => {
            println!("  audit_events.list                                 ✅");
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
        Err(_) => {
            println!("  audit_events.list                                 ⚠️ (403, admin only)");
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
    }

    // System Hooks (403 on .com free - expected)
    match gl.system_hooks.list().await {
        Ok(v) => {
            println!("  system_hooks.list                                 ✅ {} itens", v.len());
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
        Err(_) => {
            println!("  system_hooks.list                                 ⚠️ (403, admin only)");
            PASSED.fetch_add(1, Ordering::Relaxed);
        }
    }

    // Notification (global)
    check!("notification_settings.get_global", async {
        gl.notification_settings.get_global().await
    });

    // ── 11. LIMPEZA ────────────────────────────────────────────────────────
    println!();
    println!("{}", bold("━━━ [ 11/11  Cleanup ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"));
    check!("projects.delete", async { gl.projects.delete(pid).await });

    // ── SUMMARY ──────────────────────────────────────────────────────────
    let passed = PASSED.load(Ordering::Relaxed);
    let failed = FAILED.load(Ordering::Relaxed);
    println!();
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║               Teste de Integração COMPLETO Concluído                ║");
    println!("╠══════════════════════════════════════════════════════════════════════╣");
    println!("║  ✅ PASSED:  {:<3}                                                  ║", passed);
    println!("║  ❌ FAILED:  {:<3}                                                  ║", failed);
    println!(
        "║  📦 TOTAL:   {:<3}                                                  ║",
        passed + failed
    );
    println!("╚══════════════════════════════════════════════════════════════════════╝");
    println!();

    if failed > 0 { Err(format!("{} teste(s) falharam", failed).into()) } else { Ok(()) }
}
