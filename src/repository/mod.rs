mod github;
mod local;
mod repository;

pub use self::github::GitHubRepository;
pub use self::local::LocalRepository;
pub use self::repository::{Repository, Response, ResponseInfo, Stream};
