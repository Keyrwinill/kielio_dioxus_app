use shared::dto::{GameAction, GameResponse};

const API_URL: &str = "http://127.0.0.1:3000/api/dead-mans-draw";

pub async fn fetch_game() -> Result<GameResponse, reqwest::Error> {
    reqwest::get(API_URL).await?.json::<GameResponse>().await
}

pub async fn send_action(action: GameAction) -> Result<GameResponse, reqwest::Error> {
    reqwest::Client::new()
        .post(format!("{API_URL}/action"))
        .json(&action)
        .send()
        .await?
        .json::<GameResponse>()
        .await
}
