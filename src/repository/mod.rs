mod github;
mod local;
mod traits;

pub use self::github::GitHubRepository;
pub use self::local::LocalRepository;
pub use self::traits::{Repository, Response, Stream};
