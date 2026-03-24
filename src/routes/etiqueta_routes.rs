use std::{sync::Arc};

use axum::{Json, Router, extract::{Path, State}, http::StatusCode, response::{IntoResponse, Response}, routing::{delete, get, post, put}};
use uuid::Uuid;

use crate::{controladors::icontroller::IController, errors::crud_errors::CrudErrors, serveis::dtos::etiqueta_dto::EtiquetaDTO};

use super::extractors::etiqueta_extractors::CreateEtiquetaRequest;

pub fn get_etiqueta_router(etiqueta_controller: Arc<dyn IController<EtiquetaDTO>>) -> Router {
    Router::new()
    .route("/{id}", get(get_etiqueta_per_id))
    .route("/", post(crear_etiqueta))
    .route("/{id}", put(actualitzar_etiqueta))
    .route("/{id}", delete(eliminar_etiqueta))
    .with_state(etiqueta_controller.clone())
}

pub async fn crear_etiqueta
(
    State(etiqueta_controller): State<Arc<dyn IController<EtiquetaDTO>>>,
    body: Json<CreateEtiquetaRequest>
) -> Result<Response, CrudErrors> {
    etiqueta_controller.crear(body.0.into()).await?;
    let response = (StatusCode::CREATED, "Etiqueta creada correctament".to_string()).into_response();
    Ok(response)
}

pub async  fn get_etiqueta_per_id
(
    State(etiqueta_controller): State<Arc<dyn IController<EtiquetaDTO>>>,
    id: Path<Uuid>
) -> Result<Json<EtiquetaDTO>,CrudErrors> {
    Ok(Json(etiqueta_controller.get_per_id(*id).await?)) 
}

pub async fn actualitzar_etiqueta 
(
    State(etiqueta_controller): State<Arc<dyn IController<EtiquetaDTO>>>,
    id: Path<Uuid>,
    body: Json<EtiquetaDTO>
) -> Result<Json<EtiquetaDTO>, CrudErrors> {
    Ok(Json(etiqueta_controller.actualitzar(*id, body.0).await?))
}

pub async fn eliminar_etiqueta
(
    State(etiqueta_controller): State<Arc<dyn IController<EtiquetaDTO>>>,
    id: Path<Uuid>
) -> Result<Response, CrudErrors> {
    etiqueta_controller.eliminar(*id).await?;
    let response = (StatusCode::OK, "Etiqueta eliminada correctament").into_response();
    Ok(response)
}