pub mod core;
pub mod http;
pub mod resources;
pub mod types;
pub mod utils;

mod client;
pub mod oauth;

pub use client::GitLabClient;
pub use core::config::{AuthMethod, GitLabConfig, ResolvedConfig};
pub use core::errors::{ErrorCategory, ErrorContext, GitLabError};
pub use http::pagination::PaginationInfo;
pub use resources::*;
pub use types::*;
