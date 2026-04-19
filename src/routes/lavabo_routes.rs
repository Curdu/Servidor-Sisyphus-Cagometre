use std::{sync::Arc};

use axum::{Extension, Json, Router, extract::{Path, State}, http::StatusCode, response::{IntoResponse, Response}, routing::{delete, get, post, put}};
use axum_typed_multipart::{FieldData, TypedMultipart};
use tempfile::NamedTempFile;
use uuid::{Uuid};

use crate::{controladors::lavabo_controller::LavaboController, errors::lavabo_errors::LavaboErrors, serveis::dtos::{auth_dto::{AuthDataDTO, AuthToken}, lavabo_dto::{LavaboAmbEtiquetesDTO, LavaboDTO}}, state::SECRET_KEY};

use super::extractors::{ lavabo_extractors::CreateLavaboRequest};


pub fn get_lavabo_router(lavabo_controller : Arc<dyn LavaboController>) -> Router {
    Router::new()
    .route("/{id}", get(get_lavabo_per_id))
    .route("/", post(post_crear_lavabo))
    .route("/{id}", put(put_actualitzar_lavabo))
    .route("/{id}", delete(delete_eliminar_lavabo))
    .route("/", get(get_tots_lavabos))
    .route("/previews", get(get_tots_lavabos_amb_etiquetes))
    .with_state(lavabo_controller.clone())
}


pub async fn get_lavabo_per_id(State(lavabo_controlador) : State<Arc<dyn LavaboController>>, id: Path<Uuid>) -> Result<Json<LavaboDTO>, LavaboErrors> {
    let result = lavabo_controlador.get_lavabo_per_id(*id).await;
    match result {
        Ok(lavabo_dto) => {
            Ok(Json(lavabo_dto))
        },
        Err(error) => {
            Err(error)
        }
        
    }
}
pub async fn post_crear_lavabo(
    State(lavabo_controlador) : State<Arc<dyn LavaboController>>,
    Extension(claims): Extension<AuthDataDTO>,
    TypedMultipart(body): TypedMultipart<CreateLavaboRequest>
) -> Result<Response, LavaboErrors> {
    let (mut lavabo_dto, imatges) : (LavaboDTO, Vec<FieldData<NamedTempFile>>) = body.into();
    lavabo_dto.creador_id = Uuid::parse_str(&claims.sub).unwrap();
    let result = lavabo_controlador.crear_lavabo(lavabo_dto,imatges,claims).await;
    match result {
        Ok(())=>{
            Ok((StatusCode::CREATED, "Lavabo Creat corectament").into_response())
        },
        Err(error) => Err(error)
        
    }
}

pub async fn put_actualitzar_lavabo(
    State(lavabo_controlador) : State<Arc<dyn LavaboController>>, 
    id : Path<Uuid>,
    Extension(claims): Extension<AuthDataDTO>, 
    body : Json<LavaboDTO>
) -> Result<Json<LavaboDTO>, LavaboErrors> {
    let result = lavabo_controlador.actualitzar_lavabo(*id, body.0).await;
    match result {
        Ok(lavabo) => Ok(Json(lavabo)),
        Err(error) => Err(error)
    }
}

pub async fn delete_eliminar_lavabo(State(lavabo_controlador) : State<Arc<dyn LavaboController>>, id : Path<Uuid>) -> Result<Response, LavaboErrors> {
    let result = lavabo_controlador.eliminar_lavabo(*id).await;
    match result {
        Ok(()) => Ok((StatusCode::NO_CONTENT, "El lavabo s'ha eliminat correctament").into_response()),
        Err(error) => Err(error)
        
    }
}

pub async fn get_tots_lavabos(State(lavabo_controlador) : State<Arc<dyn LavaboController>>) -> Result<Json<Vec<LavaboDTO>>, LavaboErrors> {
    Ok(Json(lavabo_controlador.get_tots_lavabos().await?)) 
}

pub async fn get_tots_lavabos_amb_etiquetes(State(lavabo_controlador) : State<Arc<dyn LavaboController>>) -> Result<Json<Vec<LavaboAmbEtiquetesDTO>>, LavaboErrors> {
    Ok(Json(lavabo_controlador.get_tots_lavabos_amb_etiqueta().await?))
}
