
use std::{ sync::Arc};

use axum::{ Json, Router, extract::State, http::StatusCode, response::{IntoResponse, Response}, routing::{delete, get, post, put}};
use chrono::{Utc};
use uuid::{Uuid};

use crate::{controladors::user_controller::UserController, dades::models::rols::UsuariRol, errors::usuari_errors::UsuariErrors, routes::extractors::usuari_extractors::{ActualitzarUsuariRequest, CrearUsuariRequest, EliminarUsuariRequest, ObtenirUsuariPerIdRequest}, serveis::dtos::usuari_dto::UsuariDTO};


pub fn get_user_router(user_controller: Arc<dyn UserController>) -> Router{
    Router::new()
    .route("/", get(get_user_id))
    .route("/", post(create_user))
    .route("/", put(update_user))
    .route("/", delete(delete_user))
    .with_state(user_controller.clone())
}

async  fn get_user_id(State(user_controller) : State<Arc<dyn UserController>> , body: Json<ObtenirUsuariPerIdRequest>) -> Result<Json<UsuariDTO>, UsuariErrors>{
    let result = user_controller.obte_usuari_per_id(body.id).await;

    match result {
        Ok(usuari) => {
            Ok(Json(usuari))
        },
        Err(error) => {
            Err(error)
        }
        
    }
}

async fn create_user(State(user_controller) : State<Arc<dyn UserController>>, body: Json<CrearUsuariRequest>) -> Result<Response, UsuariErrors> {
    let usuari_dto = UsuariDTO::new(Uuid::nil(), body.correu.clone(), body.nom.clone(), body.cognoms.clone(), body.contrasenya.clone(), Utc::now(), UsuariRol::USUARI);
    let result = user_controller.crear_usuari(usuari_dto).await;
    match result {
        Ok(()) => {
            Ok((StatusCode::CREATED, "Usuari creat correctament").into_response())
        },
        Err(error) => {
            Err(error)
        }
        
    }
}

async fn update_user(State(user_controller) : State<Arc<dyn UserController>>, body : Json<ActualitzarUsuariRequest>) -> Result<Json<UsuariDTO>, UsuariErrors> {
    let result = user_controller.actualitzar_usuari(body.id, body.usuari.clone()).await;

    match result {
        Ok(usuari_dto) => Ok(Json(usuari_dto)),
        Err( error) => Err(error)
    }
}

async fn delete_user(State(user_controller) : State<Arc<dyn UserController>>, body : Json<EliminarUsuariRequest>) -> Result<Response, UsuariErrors> {
    let result = user_controller.eliminar_usuari(body.id.clone()).await;

    match result {
        Ok(_) => Ok((StatusCode::NO_CONTENT, "Usuari eliminat correctament").into_response()),
        Err(error) => Err(error)
    }

}

