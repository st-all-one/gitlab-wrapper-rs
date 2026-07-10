use std::sync::Arc;

use crate::http::client::HttpClient;
use crate::resources::*;

#[derive(Debug)]
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
