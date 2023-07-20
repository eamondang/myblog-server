use serde::Deserialize;

use crate::{err::AppError, AppResult};

#[derive(Deserialize)]
pub struct Postgres {
  pub dsn: String,
  pub max_conns: i32,
}

#[derive(Deserialize)]
pub struct WebConfig {
  pub addr: String,
}

#[derive(Deserialize)]
pub struct Config {
  pub web: WebConfig,
  pub postgres: Postgres,
}

impl Config {
  pub fn from_env() -> AppResult<Self> {
    config::Config::builder()
      .add_source(config::Environment::default())
      .build()
      .map_err(AppError::from)?
      .try_deserialize()
      .map_err(AppError::from)
  }
}
