use super::APP_HOST;
use reqwasm::{http::Request, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;
use yew::Properties;

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

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct CreateUser {
    pub name: String,
    pub email_address: String,
    pub roles: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Eq, Properties, PartialEq, Default)]
pub struct CurrentUser {
    pub id: String,
    pub name: Option<String>,
    pub email_address: String,
    pub roles: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct UserDeleteResponse {
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

pub async fn api_update_user(token: String, user: CurrentUser) -> Result<CurrentUser, Error> {
    let response = Request::patch(&format!("{}api/users/{}", APP_HOST, user.id.clone()))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .body(
            json!({
                "name": user.name,
                "email_address": user.email_address,
                "roles": user.roles,
                "status": user.status
            })
            .to_string(),
        )
        .send()
        .await?;

    response.json::<CurrentUser>().await
}

pub async fn api_delete_user(token: String, user_id: String) -> Result<UserDeleteResponse, Error> {
    let response = Request::delete(&format!("{}api/users/{}", APP_HOST, user_id.clone()))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;

    response.json::<UserDeleteResponse>().await
}

pub async fn api_me(token: String) -> Result<CurrentUser, Error> {
    let response = Request::get(&format!("{}api/users/me", APP_HOST))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;

    // let data = response.json::<CurrentUser>().await.unwrap();
    // log!(serde_json::to_string_pretty(&data).unwrap());

    response.json::<CurrentUser>().await
}
