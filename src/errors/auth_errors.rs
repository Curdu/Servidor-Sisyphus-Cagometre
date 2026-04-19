use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use super::usuari_errors::UsuariErrors;


pub(crate) enum AuthError {
    UsuariError(UsuariErrors),
    PasswdIncorrecte(String),
    ServerError(String),
    TokenInvalida(String),
    TokenFaltant(String)
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, missatge) = match self {
            AuthError::UsuariError(usuari_error) => {
                match usuari_error {
                    UsuariErrors::UsuariNotFound(_error) => {
                        (StatusCode::FORBIDDEN, "Credencials incorrectes".to_string())
                    },
                    _ => {
                        (StatusCode::INTERNAL_SERVER_ERROR, "Hi ha hagut un problema amb el servidor".to_string())
                    }
                }
            },
            AuthError::PasswdIncorrecte(_missatge) => {
                (StatusCode::FORBIDDEN, "Credencials incorrectes".to_string())
            },
            AuthError::ServerError(missatge) => {
                (StatusCode::INTERNAL_SERVER_ERROR, missatge)
            },
            AuthError::TokenInvalida(_missatge) => {
                (StatusCode::UNAUTHORIZED, _missatge)
            },
            AuthError::TokenFaltant(_missatge) => {
                (StatusCode::UNAUTHORIZED, "Falta token de validació".to_string())
            }
            
            
        };
        let body = Json(json!({"error": missatge}));
        (status, body).into_response()
    }
}