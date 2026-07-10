use std::time::Duration;

pub const LIB_NAME: &str = "gitlab-wrapper-rs";
pub const LOG_NAMESPACE: &str = "gitlab_wrapper";
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
pub const DEFAULT_MAX_RPS: u32 = 10;
pub const DEFAULT_PER_PAGE: u32 = 20;
pub const API_VERSION: &str = "v4";
pub const USER_AGENT_VALUE: &str = concat!("gitlab-wrapper-rs/", env!("CARGO_PKG_VERSION"));
