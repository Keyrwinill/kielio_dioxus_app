use axum::{
    Router,
    routing::{get, post}
};
use tower_http::cors::{CorsLayer, Any};

use crate::handlers;

pub fn create_routes() -> Router {
    Router::new()
        .route("/api/name", get(handlers::get_name)).layer(CorsLayer::permissive())
        //.route("/api/echo", post(handlers::echo_handler))
}