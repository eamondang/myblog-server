use err::AppError;

pub mod config;
pub mod context;
pub mod err;
pub mod resp;
pub mod user;

pub type AppResult<T> = std::result::Result<T, AppError>;
