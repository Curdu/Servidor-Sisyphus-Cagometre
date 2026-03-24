use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use sqlx::{Error, error::DatabaseError};

pub enum CrudErrors {
    NotFound(String),
    ServerError(String),
    NotEnoughtPermission(String),
    RegistreExistent(String),
    ReferenciaFornaeaNoValida(String)
}

impl IntoResponse for CrudErrors {
    fn into_response(self) -> axum::response::Response {
        let (status, missatge) = match self {
            CrudErrors::NotFound(msg) => {
                (StatusCode::NOT_FOUND, msg)
            },
            CrudErrors::ServerError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            },
            CrudErrors::NotEnoughtPermission(msg) => {
                (StatusCode::FORBIDDEN, msg)
            },
            CrudErrors::RegistreExistent(msg) => {
                (StatusCode::CONFLICT, msg)
            },
            CrudErrors::ReferenciaFornaeaNoValida(msg) => {
                (StatusCode::CONFLICT, msg)
            }
            
        };
        let body = Json(json!({"error": missatge}));
        (status, body).into_response()
    }
}

impl From<Error> for CrudErrors {
    fn from(value: Error) -> Self {
        Self::ServerError(value.to_string())
    }
}

impl From<&dyn DatabaseError> for CrudErrors {
    fn from(value: &dyn DatabaseError) -> Self {
        
        if value.is_foreign_key_violation(){
            Self::ReferenciaFornaeaNoValida(value.message().to_string())
        }else if value.is_unique_violation() {
            Self::RegistreExistent("Ja existeix un registre amb aquest id".to_string())
        } else {
            Self::ServerError(value.message().to_string())
        }

    }
}