use std::{sync::Arc};

use axum::{Extension, Json, Router, extract::{Path, State}, http::StatusCode, response::{IntoResponse, Response}, routing::{delete, get, post, put}};
use uuid::Uuid;

use crate::{controladors::icontroller::IController, errors::crud_errors::CrudErrors, serveis::dtos::{ auth_dto::AuthDataDTO, resenya_dto::ResenyaDTO}};

use super::extractors::{resenya_extractors::CreateResenyaRequest};

pub fn get_resenya_router(resenya_controller : Arc<dyn IController<ResenyaDTO>>) -> Router {
    Router::new()
        .route("/{id}", get(get_resenya_per_id))
        .route("/", post(crear_resenya))
        .route("/{id}", put(actualitzar_resenya))
        .route("/{id}", delete(eliminar_resenya))
        .with_state(resenya_controller.clone())
}

pub async fn get_resenya_per_id(State(resenya_controller) : State<Arc<dyn IController<ResenyaDTO>>>, id: Path<Uuid>)  -> Result<Json<ResenyaDTO>, CrudErrors> {
    Ok(Json(resenya_controller.get_per_id(*id).await?))
}

pub async fn crear_resenya
(
    State(resenya_controller) : State<Arc<dyn IController<ResenyaDTO>>>,
    Extension(claims): Extension<AuthDataDTO>,
    mut body : Json<CreateResenyaRequest>
) -> Result<Response, CrudErrors> {
    body.0.id_usuari = Option::Some(Uuid::parse_str(&claims.sub).unwrap());
    resenya_controller.crear(body.0.into()).await?;
    let response = (StatusCode::CREATED, "Resenya creada correctament".to_string()).into_response();
    Ok(response) 
}

pub async fn actualitzar_resenya
(
    State(resenya_controller) : State<Arc<dyn IController<ResenyaDTO>>>,
    id: Path<Uuid>,
    body : Json<ResenyaDTO>
) -> Result<Json<ResenyaDTO>, CrudErrors> {
    Ok(Json( resenya_controller.actualitzar(*id, body.0.into()).await?))
}

pub async fn eliminar_resenya
(
    State(resenya_controller) : State<Arc<dyn IController<ResenyaDTO>>>,
    id: Path<Uuid>
) -> Result<Response, CrudErrors> {
    resenya_controller.eliminar(*id).await?;
    let response = (StatusCode::OK, "Resenya eliminada correctament".to_string()).into_response();
    Ok(response)
}

