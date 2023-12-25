use crate::helpers::{
    executor::{execute_code, ExecutionResult},
    language,
};
use axum::{http::StatusCode, routing::post, Json, Router};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ExecuteRequestBody {
    pub lang: language::Language,
    pub code: String,
}

pub fn router() -> Router {
    Router::new().route("/", post(execute))
}

pub async fn execute(
    Json(payload): Json<ExecuteRequestBody>,
) -> (StatusCode, Json<ExecutionResult>) {
    let output = execute_code(payload.lang, payload.code).await;

    match output {
        Ok(res) => (StatusCode::OK, Json(res)),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ExecutionResult::default()),
        ),
    }
}
