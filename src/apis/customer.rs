use super::APP_HOST;
use crate::components::organisms::paginator::PaginationDataProps;
use reqwasm::{http::Request, Error};
use serde::{Deserialize, Serialize};
use yew::Properties;
use serde_json::json;

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq, Default)]
pub struct DeviceProps {
    pub device: Option<String>,
    pub os: String,
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
    pub device: DeviceProps,
    pub ip_address: String
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct UsersResponse {
    pub result: Vec<UserResponse>,
    pub page: i32,
    pub per_page: i32,
    pub total: i32,
    pub total_pages: i32,
}

#[derive(Clone, Properties, PartialEq, Default)]
pub struct UpdateCustomer {
    pub id: String,
    pub status: String,
}


pub async fn fetch_users(
    token: String,
    pagination: PaginationDataProps,
    search_text: String,
) -> Result<UsersResponse, Error> {
    let response = Request::get(&format!(
        "{}api/users?limit={}&page={}&name={}&role=Customer",
        APP_HOST, pagination.per_page, pagination.current_page, search_text
    ))
    .header("Content-Type", "application/json")
    .header("Authorization", &format!("Bearer {}", token))
    .send()
    .await?;

    response.json::<UsersResponse>().await
}

pub async fn update_user(token: String, user: UpdateCustomer) -> Result<UserResponse, Error> {
    let response = Request::put(&format!("{}api/users/status", APP_HOST))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .body(
            json!({
                "id": user.clone().id,
                "status": user.clone().status,
            }).to_string()
        )
        .send().await?;

    response.json::<UserResponse>().await
}