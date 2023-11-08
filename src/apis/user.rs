use super::APP_HOST;
use gloo_console::log;
use reqwasm::{http::Request, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;
use yew::{platform::spawn_local, prelude::*};

// pub struct User {
//     pub id: i32,
//     pub username: String,
//     pub created_at: String,
// }

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginResponse {
    pub token: String,
    pub status: String,
}

// #[derive(Deserialize)]
// pub struct MeResponse {
//     pub id: i32,
//     pub username: String,
//     pub created_at: String,
// }

pub async fn api_login(email: String, password: String) -> Result<LoginResponse, Error> {
    let response = Request::post(&format!("{}api/auth/login", APP_HOST))
        .header("Content-Type", "application/json")
        .body(
            json!({
                "email": email,
                "password": password
            })
            .to_string(),
        )
        .send()
        .await
        .unwrap()
        .json::<LoginResponse>()
        .await
        .unwrap();

    Ok(response)
}

// pub async fn api_me(token: &String) -> Result<MeResponse, Error> {
//     let response = Request::post(&format!("{}/me", APP_HOST))
//         .header("Authorization", &format!("Bearer {}", token))
//         .send()
//         .await?;

//     response.json::<MeResponse>().await
// }
