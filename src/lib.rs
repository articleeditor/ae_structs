use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

pub mod product;
pub mod shop;
pub mod user;

#[derive(Deserialize, Serialize)]
pub struct Response<T, S> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<S>,
}

pub fn handle_error<T, S>(e: S) -> Json<Response<T, S>> {
    Json(Response {
        success: false,
        data: None,
        error: Some(e),
    })
}

pub fn handle_success<T, S>(data: Option<T>) -> Json<Response<T, S>> {
    Json(Response {
        success: true,
        data,
        error: None,
    })
}
