use super::APP_HOST;
use gloo_console::log;
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

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct CurrentUser {
    pub id: String,
    pub name: Option<String>,
    pub email_address: String,
    pub roles: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for CurrentUser {
    fn default() -> Self {
        CurrentUser {
            id: String::from("abc"),
            name: None,
            email_address: String::new(),
            roles: String::new(),
            status: String::new(),     // And for status
            created_at: String::new(), // And for created_at
            updated_at: String::new(), // And for updated_at
        }
    }
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

// pub async fn api_update_user(user: CreateUser) -> Result<CurrentUser, Error> {
//     let response = Request::patch(&format!("{}api/users/{}", APP_HOST, user.id.clone()))
//         .header("Content-Type", "application/json")
//         .body(
//             json!({
//                 "name": user.name,
//                 "email_address": user.email_address,
//                 "roles": user.roles,
//                 "status": user.status
//             })
//             .to_string(),
//         )
//         .send()
//         .await?;

//     response.json::<CurrentUser>().await
// }

pub async fn api_me() -> Result<CurrentUser, Error> {
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIyYTZhNmY1Mi00ODQxLTQ3NTctYTRkMi1mMjhiNmFiMTkwYTQiLCJpYXQiOjE2OTk1MDkwMzUsImV4cCI6MTcwNDY5MzAzNX0.3lFpaMk9qJ2DxW1PYNK_IEMQ_y98mLcrNiPrp2dCSBM";

    let response = Request::get(&format!("{}api/users/me", APP_HOST))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;

    // let data = response.json::<CurrentUser>().await.unwrap();
    // log!(serde_json::to_string_pretty(&data).unwrap());

    response.json::<CurrentUser>().await
}
