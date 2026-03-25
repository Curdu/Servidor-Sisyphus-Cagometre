use std::sync::Arc;

use auth_routes::{get_auth_router, verificar_token};
use axum::{Router, http::Method, middleware};
use etiqueta_routes::get_etiqueta_router;
use lavabo_routes::get_lavabo_router;
use resenya_routes::get_resenya_router;
use tower_http::cors::{Any, CorsLayer};

use crate::{controladors::{auth_controller::AuthController, etiqueta_controller::EtiquetaController, icontroller::IController, lavabo_controller::LavaboController, user_controller::UserController}, routes::user_routes::get_user_router, serveis::dtos::{resenya_dto::ResenyaDTO}};

pub(crate) mod user_routes;
pub(crate) mod extractors;
pub(crate) mod lavabo_routes;
pub(crate) mod auth_routes;
pub(crate) mod etiqueta_routes;
pub(crate) mod resenya_routes;

pub(crate) fn get_router(controladors: Controladors) -> Router {

    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST,Method::PUT,Method::DELETE])
        .allow_headers(Any);

    let middleware_verificacio = middleware::from_fn(verificar_token);
    
    let usuari_router = get_user_router(controladors.usuari).route_layer(middleware_verificacio.clone());
    let lavabo_router = get_lavabo_router(controladors.lavabo).route_layer(middleware_verificacio.clone());
    let auth_router = get_auth_router(controladors.auth);
    let resenya_router = get_resenya_router(controladors.resenya).route_layer(middleware_verificacio.clone());
    let etiqueta_router = get_etiqueta_router(controladors.etiqueta).route_layer(middleware_verificacio.clone());

    Router::new()
        .nest("/api/usuaris", usuari_router)
        .nest("/api/lavabos", lavabo_router)
        .nest("/api/resenyes", resenya_router)
        .nest("/api/etiquetes", etiqueta_router)
        .nest("/api", auth_router)
        .layer(cors_layer)
}

pub(crate) struct  Controladors {
    pub(crate) usuari: Arc<dyn UserController>,
    pub(crate) lavabo: Arc<dyn LavaboController>,
    pub(crate) auth: Arc<dyn AuthController>,
    pub(crate) resenya: Arc<dyn IController<ResenyaDTO>>,
    pub(crate) etiqueta: Arc<dyn EtiquetaController>
}

