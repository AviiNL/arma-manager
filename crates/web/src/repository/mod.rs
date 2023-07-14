mod user_repository;
mod user_token_repository;

type RepositoryResult<T> = Result<T, Box<dyn std::error::Error>>;

pub use user_repository::*;
pub use user_token_repository::*;
