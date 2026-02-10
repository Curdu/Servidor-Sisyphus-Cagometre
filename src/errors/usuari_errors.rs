use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub enum UsuariErrors {
    UsuariNotFound(String),
    ServerError(String),
    NotEnoughtPermission(String),
    UsuariExistent(String)
}

impl IntoResponse for UsuariErrors {
    fn into_response(self) -> axum::response::Response {
        let (status, missatge) = match self {
            UsuariErrors::UsuariNotFound(msg) => {
                (StatusCode::NOT_FOUND, msg)
            },
            UsuariErrors::ServerError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            },
            UsuariErrors::NotEnoughtPermission(msg) => {
                (StatusCode::FORBIDDEN, msg)
            },
            UsuariErrors::UsuariExistent(msg) => {
                (StatusCode::CONFLICT, msg)
            }
            
        };
        let body = Json(json!({"error": missatge}));
        (status, body).into_response()
    }
}