//! Módulo de recursos da API do GitLab.
//!
//! Este módulo contém todos os recursos (endpoints) da API do GitLab,
//! organizados em arquivos separados por domínio. Cada recurso expõe
//! métodos para listar, obter, criar, atualizar e deletar entidades
//! no GitLab.
//!
//! Para usar um recurso, acesse-o através do `GitLabClient`:
//!
//! ```rust,ignore
//! let client = GitLabClient::new(config)?;
//! let projects = client.projects().list(None)?;
//! ```

mod branches;
mod commits;
mod deploy_keys;
mod discussions;
mod environments;
mod events;
mod groups;
mod issues;
mod jobs;
mod labels;
mod members;
mod merge_requests;
mod milestones;
mod notes;
mod pipeline_schedules;
mod pipelines;
mod projects;
mod releases;
mod repository_files;
mod runners;
mod search;
mod tags;
mod todos;
mod users;
mod wikis;

/// Recurso de API para operações com branches no GitLab.
pub use branches::BranchesResource;
/// Recurso de API para operações com commits no GitLab.
pub use commits::CommitsResource;
/// Recurso de API para operações com chaves de deploy no GitLab.
pub use deploy_keys::DeployKeysResource;
/// Recurso de API para operações com discussões no GitLab.
pub use discussions::DiscussionsResource;
/// Recurso de API para operações com ambientes no GitLab.
pub use environments::EnvironmentsResource;
/// Recurso de API para operações com eventos no GitLab.
pub use events::EventsResource;
/// Recurso de API para operações com grupos no GitLab.
pub use groups::GroupsResource;
/// Recurso de API para operações com issues no GitLab.
pub use issues::IssuesResource;
/// Recurso de API para operações com jobs no GitLab.
pub use jobs::JobsResource;
/// Recurso de API para operações com labels no GitLab.
pub use labels::LabelsResource;
/// Recurso de API para operações com membros no GitLab.
pub use members::MembersResource;
/// Recurso de API para operações com merge requests no GitLab.
pub use merge_requests::MergeRequestsResource;
/// Recurso de API para operações com milestones no GitLab.
pub use milestones::MilestonesResource;
/// Recurso de API para operações com notas no GitLab.
pub use notes::NotesResource;
/// Recurso de API para operações com agendamentos de pipeline no GitLab.
pub use pipeline_schedules::PipelineSchedulesResource;
/// Recurso de API para operações com pipelines no GitLab.
pub use pipelines::PipelinesResource;
/// Recurso de API para operações com projetos no GitLab.
pub use projects::ProjectsResource;
/// Recurso de API para operações com releases no GitLab.
pub use releases::ReleasesResource;
/// Recurso de API para operações com arquivos de repositório no GitLab.
pub use repository_files::RepositoryFilesResource;
/// Recurso de API para operações com runners no GitLab.
pub use runners::RunnersResource;
/// Recurso de API para operações com busca no GitLab.
pub use search::SearchResource;
/// Recurso de API para operações com tags no GitLab.
pub use tags::TagsResource;
/// Recurso de API para operações com todos no GitLab.
pub use todos::TodosResource;
/// Recurso de API para operações com usuários no GitLab.
pub use users::UsersResource;
/// Recurso de API para operações com wikis no GitLab.
pub use wikis::WikisResource;
