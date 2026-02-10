use axum::{Router, routing::get};


pub fn get_proba_controlador() -> Router {
    Router::new()
    .route("/", get(async || {"Cafe"}))
}