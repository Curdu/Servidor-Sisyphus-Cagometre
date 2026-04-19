use axum::{Json, http::StatusCode, response::{IntoResponse, Response}};
use serde_json::json;

use crate::errors::{crud_errors::CrudErrors, storage_errors::StorageError};

pub enum LavaboErrors {
    
    LavaboNotFound(String),
    ServerError(String),
    LavaboExistent(String),
    ImatgeNoPujada(String)
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
            },
            LavaboErrors::ImatgeNoPujada(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            
        };
        let body = Json(json!({"error": missatge}));
        (status, body).into_response()
    }
}


impl From<StorageError> for LavaboErrors {
    fn from(value: StorageError) -> Self {
        match value {
            StorageError::ServerError(err) => LavaboErrors::ImatgeNoPujada(err),
        }
    }
}

impl From<CrudErrors> for LavaboErrors {
    fn from(value: CrudErrors) -> Self {
        match value {
            CrudErrors::NotFound(err) => LavaboErrors::LavaboNotFound(err),
            CrudErrors::ServerError(err) => LavaboErrors::ServerError(err),
            CrudErrors::NotEnoughtPermission(err) => LavaboErrors::ServerError(err),
            CrudErrors::RegistreExistent(err) => LavaboErrors::LavaboExistent(err),
            CrudErrors::ReferenciaFornaeaNoValida(err) => LavaboErrors::ServerError(err),
        }
    }
}