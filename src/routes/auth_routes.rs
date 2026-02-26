use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::{IntoResponse, Response}, routing::post};
use chrono::{Utc};
use uuid::Uuid;

use crate::{controladors::auth_controller::AuthController, dades::models::rols::UsuariRol, errors::{auth_errors::AuthError, usuari_errors::UsuariErrors}, serveis::dtos::{auth_dto::AuthToken, usuari_dto::UsuariDTO}};

use super::extractors::{auth_extractors::LoginRequest, usuari_extractors::CrearUsuariRequest};

pub fn get_auth_router(auth_controller: Arc<dyn AuthController>) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/registre", post(registre))
        .with_state(auth_controller)
        
}


async fn login(State(auth_controller) : State<Arc<dyn AuthController>>, body : Json<LoginRequest>) -> Result<Json<AuthToken>, AuthError> {
    let result = auth_controller.login(body.correu.clone(), body.passwd.clone()).await;
    match result {
        Ok(token) => Ok(Json(token)),
        Err(error) => Err(error)
    }

}

async fn registre(State(auth_controller) : State<Arc<dyn AuthController>>, body : Json<CrearUsuariRequest>) -> Result<Response, UsuariErrors> {
    let usuari_dto = UsuariDTO::new(Uuid::new_v4(), body.correu.clone(), body.nom.clone(), body.cognoms.clone(), body.contrasenya.clone(), Utc::now(), UsuariRol::USUARI);
    let result = auth_controller.registre(usuari_dto).await;
    match result {
        Ok(_) => {
            Ok((StatusCode::CREATED, "Registrat correctament").into_response())
        },
        Err(error) => Err(error)
    }
}