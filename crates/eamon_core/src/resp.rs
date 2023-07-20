use axum::Json;
use serde::Serialize;

use crate::err::AppError;

#[derive(Serialize, Debug)]
pub struct Response<T: Serialize> {
  pub code: i32,
  pub msg: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<T>,
}

impl<T: Serialize> Response<T> {
  pub fn new(code: i32, msg: &str, data: Option<T>) -> Self {
    let msg = msg.to_string();
    Self { code, msg, data }
  }

  pub fn ok(data: T) -> Self {
    Self::new(0, "OK", Some(data))
  }

  pub fn ok_empty() -> Self {
    Self::new(0, "OK", None)
  }

  pub fn failed(code: i32, msg: &str) -> Self {
    Self::new(code, msg, None)
  }

  pub fn failed_default(msg: &str) -> Self {
    Self::failed(-1, msg)
  }

  pub fn err_with_core(code: i32, e: &AppError) -> Self {
    Self::failed(code, &e.message)
  }

  pub fn err(e: &AppError) -> Self {
    Self::err_with_core(e.code(), e)
  }

  pub fn err_with_data(e: &AppError, data: T) -> Self {
    Self::new(e.code(), &e.message, Some(data))
  }

  pub fn to_json(self) -> JsonResponse<T> {
    Json(self)
  }
}

pub type JsonResponse<T> = Json<Response<T>>;

#[derive(Serialize)]
pub struct IDResponse {
  pub id: u32,
}

#[derive(Serialize)]
pub struct ID64Response {
  pub id: u64,
}
