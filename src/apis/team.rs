use super::APP_HOST;
use crate::components::organisms::paginator::PaginationDataProps;
use reqwasm::{http::Request, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;
use yew::Properties;

#[derive(Default, Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct User {
    pub email_address: String,
    pub name: String,
    pub password: String,
    pub roles: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct UserResponse {
    pub created_at: String,
    pub id: String,
    pub updated_at: String,
    pub email_address: String,
    pub name: String,
    pub roles: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct UsersResponse {
    pub result: Vec<UserResponse>,
    pub page: i32,
    pub per_page: i32,
    pub total: i32,
    pub total_pages: i32,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct UserDeleteResponse {
    pub message: String,
}

pub async fn create_user(token: String, user: User) -> Result<UserResponse, Error> {
    let response = Request::post(&format!("{}api/users/team_member", APP_HOST))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .body(
            json!({
                "email_address": user.clone().email_address,
                "name": user.clone().name,
                "password": user.clone().password,
                "roles": user.clone().roles,
                "status": user.clone().status
            })
            .to_string(),
        )
        .send()
        .await?;

    response.json::<UserResponse>().await
}

pub async fn update_user(
    token: String,
    user: User,
    user_id: String,
) -> Result<UserResponse, Error> {
    let response = Request::put(&format!("{}api/users/team_member/{}", APP_HOST, user_id))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .body(
            json!({
                "email_address": user.clone().email_address,
                "name": user.clone().name,
                // "password": user.clone().password,
                "roles": user.clone().roles,
                "status": user.clone().status
            })
            .to_string(),
        )
        .send()
        .await?;

    response.json::<UserResponse>().await
}

pub async fn fetch_users(
    token: String,
    pagination: PaginationDataProps,
    search_text: String,
) -> Result<UsersResponse, Error> {
    let response = Request::get(&format!(
        "{}api/users?limit={}&page={}&name={}",
        APP_HOST, pagination.per_page, pagination.current_page, search_text
    ))
    .header("Content-Type", "application/json")
    .header("Authorization", &format!("Bearer {}", token))
    .send()
    .await?;

    response.json::<UsersResponse>().await
}

pub async fn fetch_user(token: String, user_id: String) -> Result<UserResponse, Error> {
    let response = Request::get(&format!("{}api/users/{}", APP_HOST, user_id))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;

    response.json::<UserResponse>().await
}

pub async fn delete_user(token: String, user_id: String) -> Result<UserDeleteResponse, Error> {
    let response = Request::delete(&format!("{}api/users/{}", APP_HOST, user_id))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;

    response.json::<UserDeleteResponse>().await
}
