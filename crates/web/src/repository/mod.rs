type RepositoryResult<T> = Result<T, Box<dyn std::error::Error>>;

mod dlc_repository;
mod preset_repository;
mod user_repository;
mod user_token_repository;

pub use dlc_repository::*;
pub use preset_repository::*;
pub use user_repository::*;
pub use user_token_repository::*;
