use std::sync::Arc;

use axum::{Router, http::Method, routing::Route};
use tower_http::cors::{Any, CorsLayer};

use crate::{controladors::{self, user_controller::UserController}, routes::user_routes::get_user_router};

pub(crate) mod user_routes;
pub(crate) mod extractors;

pub(crate) fn get_router(controladors: Controladors) -> Router {

    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST,Method::PUT,Method::DELETE])
        .allow_headers(Any);
    

    let usuari_router = get_user_router(controladors.usuari);

    Router::new()
        .nest("/api/usuaris", usuari_router)
        .layer(cors_layer)
}

pub(crate) struct  Controladors {
    pub(crate) usuari: Arc<dyn UserController>
}
