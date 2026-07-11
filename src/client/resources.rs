//! Agrupamento de todos os recursos da API do GitLab.
//!
//! [`ResourceGroup`] contém uma instância de cada recurso (branches, commits, projects,
//! issues, etc.) e é acessado transparentemente via `Deref` em [`crate::client::GitLabClient`].

use std::sync::Arc;

use crate::http::client::HttpClient;
use crate::resources::*;

/// Agrupa todos os recursos da API do GitLab em um único ponto de acesso.
///
/// Cada campo público expõe um recurso específico (ex.: `projects`, `issues`,
/// `merge_requests`) com seus respectivos métodos de CRUD e consulta.
#[derive(Debug)]
pub struct ResourceGroup {
    /// Recurso de branches.
    pub branches: BranchesResource,
    /// Recurso de commits.
    pub commits: CommitsResource,
    /// Recurso de chaves de deploy.
    pub deploy_keys: DeployKeysResource,
    /// Recurso de discussões.
    pub discussions: DiscussionsResource,
    /// Recurso de ambientes (environments).
    pub environments: EnvironmentsResource,
    /// Recurso de eventos.
    pub events: EventsResource,
    /// Recurso de grupos.
    pub groups: GroupsResource,
    /// Recurso de issues.
    pub issues: IssuesResource,
    /// Recurso de jobs.
    pub jobs: JobsResource,
    /// Recurso de labels.
    pub labels: LabelsResource,
    /// Recurso de membros.
    pub members: MembersResource,
    /// Recurso de merge requests.
    pub merge_requests: MergeRequestsResource,
    /// Recurso de marcos (milestones).
    pub milestones: MilestonesResource,
    /// Recurso de notas (comentários).
    pub notes: NotesResource,
    /// Recurso de agendamentos de pipeline.
    pub pipeline_schedules: PipelineSchedulesResource,
    /// Recurso de pipelines.
    pub pipelines: PipelinesResource,
    /// Recurso de projetos.
    pub projects: ProjectsResource,
    /// Recurso de releases.
    pub releases: ReleasesResource,
    /// Recurso de arquivos de repositório.
    pub repository_files: RepositoryFilesResource,
    /// Recurso de runners.
    pub runners: RunnersResource,
    /// Recurso de busca.
    pub search: SearchResource,
    /// Recurso de tags.
    pub tags: TagsResource,
    /// Recurso de todos (tarefas pendentes).
    pub todos: TodosResource,
    /// Recurso de usuários.
    pub users: UsersResource,
    /// Recurso de wikis.
    pub wikis: WikisResource,
}

impl ResourceGroup {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self {
            branches: BranchesResource::new(Arc::clone(&http)),
            commits: CommitsResource::new(Arc::clone(&http)),
            deploy_keys: DeployKeysResource::new(Arc::clone(&http)),
            discussions: DiscussionsResource::new(Arc::clone(&http)),
            environments: EnvironmentsResource::new(Arc::clone(&http)),
            events: EventsResource::new(Arc::clone(&http)),
            groups: GroupsResource::new(Arc::clone(&http)),
            issues: IssuesResource::new(Arc::clone(&http)),
            jobs: JobsResource::new(Arc::clone(&http)),
            labels: LabelsResource::new(Arc::clone(&http)),
            members: MembersResource::new(Arc::clone(&http)),
            merge_requests: MergeRequestsResource::new(Arc::clone(&http)),
            milestones: MilestonesResource::new(Arc::clone(&http)),
            notes: NotesResource::new(Arc::clone(&http)),
            pipeline_schedules: PipelineSchedulesResource::new(Arc::clone(&http)),
            pipelines: PipelinesResource::new(Arc::clone(&http)),
            projects: ProjectsResource::new(Arc::clone(&http)),
            releases: ReleasesResource::new(Arc::clone(&http)),
            repository_files: RepositoryFilesResource::new(Arc::clone(&http)),
            runners: RunnersResource::new(Arc::clone(&http)),
            search: SearchResource::new(Arc::clone(&http)),
            tags: TagsResource::new(Arc::clone(&http)),
            todos: TodosResource::new(Arc::clone(&http)),
            users: UsersResource::new(Arc::clone(&http)),
            wikis: WikisResource::new(Arc::clone(&http)),
        }
    }
}
