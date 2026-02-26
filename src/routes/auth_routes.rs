use std::sync::Arc;

use axum::{Json, Router, extract::{Request, State}, http::StatusCode, middleware::Next, response::{IntoResponse, Response}, routing::post};
use axum_extra::TypedHeader;
use chrono::{Utc};
use headers::{Authorization, authorization::Bearer};
use jsonwebtoken::{DecodingKey, Validation, decode};
use uuid::Uuid;

use crate::{controladors::auth_controller::AuthController, dades::models::rols::UsuariRol, errors::{auth_errors::AuthError, usuari_errors::UsuariErrors}, routes::extractors::auth_extractors::ClaimsInfo, serveis::dtos::{auth_dto::AuthToken, usuari_dto::UsuariDTO}, state::SECRET_KEY};

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

pub (crate) async fn verificar_token(auth_header: Option<TypedHeader<Authorization<Bearer>>>, mut req: Request, next: Next) -> Result<Response, AuthError>{

    let TypedHeader(auth) = auth_header.ok_or(AuthError::TokenFaltant("Falta token en la petició".to_string()))?;

    let token_decodificada = decode::<ClaimsInfo>(
        auth.token(), 
        &DecodingKey::from_secret(SECRET_KEY.as_bytes()), 
        &Validation::default()
    );

    let tokn = match token_decodificada {
        Ok(token) => {
            token.claims
        },
        Err(error) => {
            return Err(AuthError::TokenInvalida(error.to_string()))
        }
    };

    req.extensions_mut().insert(tokn);

    Ok(next.run(req).await)

}