use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::post};

use crate::{controladors::auth_controller::AuthController, errors::auth_errors::AuthError, serveis::dtos::auth_dto::AuthToken};

use super::extractors::auth_extractors::LoginRequest;

pub fn get_auth_router(auth_controller: Arc<dyn AuthController>) -> Router {
    Router::new()
        .route("/login", post(login))
        .with_state(auth_controller)
        
}


async fn login(State(auth_controller) : State<Arc<dyn AuthController>>, body : Json<LoginRequest>) -> Result<Json<AuthToken>, AuthError> {
    let result = auth_controller.login(body.correu.clone(), body.passwd.clone()).await;
    match result {
        Ok(token) => Ok(Json(token)),
        Err(error) => Err(error)
    }

}