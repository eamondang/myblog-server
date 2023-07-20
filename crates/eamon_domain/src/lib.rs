use err::AppError;

pub mod config;
pub mod err;
pub mod resp;

pub type AppResult<T> = std::result::Result<T, AppError>;
