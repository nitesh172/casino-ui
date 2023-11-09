use super::APP_HOST;
use gloo_console::log;
use reqwasm::{http::Request, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ForgotPasswordResponse {
    pub message: String,
    pub password_reset_token: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PasswordResetResponse {
    pub message: String,
}

pub async fn api_login(email: String, password: String) -> Result<LoginResponse, Error> {
    let response = Request::post(&format!("{}api/auth/login", APP_HOST))
        .header("Content-Type", "application/json")
        .body(
            json!({
                "email_address": email,
                "password": password
            })
            .to_string(),
        )
        .send()
        .await?;

    response.json::<LoginResponse>().await
}

pub async fn api_forgot_password(email: String) -> Result<ForgotPasswordResponse, Error> {
    let response = Request::post(&format!("{}api/auth/forgot-password", APP_HOST))
        .header("Content-Type", "application/json")
        .body(
            json!({
                "email_address": email
            })
            .to_string(),
        )
        .send()
        .await?;

    response.json::<ForgotPasswordResponse>().await
}

pub async fn api_reset_password(
    password_reset_token: String,
    password: String,
    password_confirm: String,
) -> Result<PasswordResetResponse, Error> {
    let response = Request::patch(&format!(
        "{}api/auth/rest-password/{}",
        APP_HOST, password_reset_token
    ))
    .header("Content-Type", "application/json")
    .body(
        json!({
            "password": password,
            "password_confirm": password_confirm
        })
        .to_string(),
    )
    .send()
    .await?;

    response.json::<PasswordResetResponse>().await
}
