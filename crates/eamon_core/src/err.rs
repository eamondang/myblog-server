use std::{error::Error, fmt::Display};

use axum::response::IntoResponse;

use crate::resp::Response;

#[derive(Debug)]
pub enum Kind {
  NotFound,
  Validator,
  Chrono,
  Jwt,
  AlreadyExists,
  Redis,
  Serde,
  UserAgent,
  Reqwest,
  Config,
  Database,
  Templates,
}

#[derive(Debug)]
pub struct AppError {
  pub message: String,
  pub cause: Option<Box<dyn std::error::Error>>,
  pub kind: Kind,
}

impl AppError {
  pub fn new(message: String, cause: Option<Box<dyn std::error::Error>>, kind: Kind) -> Self {
    Self { message, cause, kind }
  }

  pub fn with_cause(cause: Box<dyn std::error::Error>, kind: Kind) -> Self {
    Self::new(cause.to_string(), Some(cause), kind)
  }

  pub fn from_str(msg: &str, kind: Kind) -> Self {
    Self::new(msg.to_string(), None, kind)
  }

  pub fn already_exists(msg: &str) -> Self {
    Self::from_str(msg, Kind::AlreadyExists)
  }

  pub fn not_found(msg: &str) -> Self {
    Self::from_str(msg, Kind::NotFound)
  }

  pub fn code(&self) -> i32 {
    match &self.kind {
      Kind::Jwt => 9527,
      Kind::Validator => 9528,
      Kind::NotFound => 9529,
      Kind::AlreadyExists => 9530,
      _ => -1,
    }
  }
}

impl From<config::ConfigError> for AppError {
  fn from(value: config::ConfigError) -> Self {
    Self::with_cause(Box::new(value), Kind::Config)
  }
}

impl From<sqlx::Error> for AppError {
  fn from(value: sqlx::Error) -> Self {
    Self::with_cause(Box::new(value), Kind::Database)
  }
}

impl From<askama::Error> for AppError {
  fn from(value: askama::Error) -> Self {
    Self::with_cause(Box::new(value), Kind::Templates)
  }
}

impl Display for AppError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for AppError {}

impl From<redis::RedisError> for AppError {
  fn from(value: redis::RedisError) -> Self {
    Self::with_cause(Box::new(value), Kind::Redis)
  }
}

impl From<serde_json::Error> for AppError {
  fn from(value: serde_json::Error) -> Self {
    Self::with_cause(Box::new(value), Kind::Serde)
  }
}

impl From<chrono::ParseError> for AppError {
  fn from(value: chrono::ParseError) -> Self {
    Self::with_cause(Box::new(value), Kind::Chrono)
  }
}

impl IntoResponse for AppError {
  fn into_response(self) -> axum::response::Response {
    Response::<()>::err_with_core(self.code(), &self).to_json().into_response()
  }
}
