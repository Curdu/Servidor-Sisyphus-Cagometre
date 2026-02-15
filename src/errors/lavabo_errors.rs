use axum::{Json, http::StatusCode, response::{IntoResponse, Response}};
use serde_json::json;

pub enum LavaboErrors {
    
    LavaboNotFound(String),
    ServerError(String),
    LavaboExistent(String)
}

impl IntoResponse for LavaboErrors {
fn into_response(self) -> Response {
        let (status, missatge) = match self {
            LavaboErrors::LavaboNotFound(msg) => {
                (StatusCode::NOT_FOUND, msg)
            },
            LavaboErrors::ServerError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            },
            LavaboErrors::LavaboExistent(msg) => {
                (StatusCode::CONFLICT, msg)
            }
            
        };
        let body = Json(json!({"error": missatge}));
        (status, body).into_response()
    }
}