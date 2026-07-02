use axum::{
    extract::{State, OriginalUri},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

use shared::dto::{GameAction, GameResponse};
use shared::games::dead_mans_draw::{
    engine::handle_action,
    state::GameState,
};

type AppState = Arc<Mutex<GameState>>;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(GameState::new()));

    let app = Router::new()
        .route("/api/dead-mans-draw", get(get_game))
        .route("/api/dead-mans-draw/action", post(do_action))
        .fallback(fallback_handler)
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Backend running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn get_game(State(state): State<AppState>) -> Json<GameResponse> {
    let game = state.lock().unwrap().clone();
    Json(GameResponse { state: game })
}

async fn do_action(
    State(state): State<AppState>,
    Json(action): Json<GameAction>,
) -> Json<GameResponse> {
    let mut game = state.lock().unwrap();

    handle_action(&mut game, action);

    Json(GameResponse {
        state: game.clone(),
    })
}

async fn fallback_handler(uri: OriginalUri) -> (StatusCode, String) {
    println!("404 route called: {}", uri.0);
    (StatusCode::NOT_FOUND, format!("No route: {}", uri.0))
}

/* Initial
use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;

mod routes;
mod handlers;

#[tokio::main]
async fn main() {
    let app = routes::create_routes();
    let addr = "127.0.0.1:3000";

    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Backend is running on {}", addr);
    
    axum::serve(listener, app).await.unwrap();
}
*/