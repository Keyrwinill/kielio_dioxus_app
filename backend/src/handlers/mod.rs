use axum::Json;
use shared::NameResponse;

pub async fn get_name() -> Json<NameResponse> {
    Json(NameResponse {
        name: "Erwin".to_string(),
    })
}