use serde::{Deserialize, Serialize};

pub mod requests;
pub mod responses;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct UserDto {
  #[serde(skip_serializing, skip_deserializing)]
  pub id: i64,
  pub username: String,
  pub email: Option<String>,
  pub bio: Option<String>,
  pub image: Option<String>,
  pub publickey: String,
}
