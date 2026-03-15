use std::sync::Arc;

use auth_routes::{get_auth_router, verificar_token};
use axum::{Router, http::Method, middleware};
use lavabo_routes::get_lavabo_router;
use tower_http::cors::{Any, CorsLayer};

use crate::{controladors::{auth_controller::AuthController, lavabo_controller::LavaboController, user_controller::UserController}, routes::user_routes::get_user_router};

pub(crate) mod user_routes;
pub(crate) mod extractors;
pub(crate) mod lavabo_routes;
pub(crate) mod auth_routes;

pub(crate) fn get_router(controladors: Controladors) -> Router {

    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST,Method::PUT,Method::DELETE])
        .allow_headers(Any);

    let usuari_router = get_user_router(controladors.usuari).route_layer(middleware::from_fn(verificar_token));
    let lavabo_router = get_lavabo_router(controladors.lavabo);
    let auth_router = get_auth_router(controladors.auth);

    Router::new()
        .nest("/api/usuaris", usuari_router)
        .nest("/api/lavabos", lavabo_router)
        .nest("/api", auth_router)
        .layer(cors_layer)
}

pub(crate) struct  Controladors {
    pub(crate) usuari: Arc<dyn UserController>,
    pub(crate) lavabo: Arc<dyn LavaboController>,
    pub(crate) auth: Arc<dyn AuthController>
}
