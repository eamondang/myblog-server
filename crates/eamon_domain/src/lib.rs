use err::AppError;

pub mod err;
pub mod resp;

pub type Result<T> = std::result::Result<T, AppError>;
