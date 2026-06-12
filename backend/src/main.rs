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
