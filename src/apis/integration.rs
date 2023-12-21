use super::APP_HOST;
use reqwasm::{http::Request, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;
use yew::Properties;
use crate::utils::capitalize::uppercase_first_letter;

#[derive(Default, Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct Integration {
    pub api_key: String,
    pub name: String,
    pub secret_key: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct IntegrationResponse {
    pub id: String,
    pub status: String,
    pub updated_at: String,
    pub api_key: String,
    pub name: String,
    pub secret_key: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct IntegrationsResponse {
    pub result: Vec<IntegrationResponse>,
    pub page: i32,
    pub per_page: i32,
    pub total: i32,
    pub total_pages: i32
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct IntegrationsDeleteResponse {
    pub message: String
}

pub async fn create_integration(token: String, integration: Integration) -> Result<IntegrationResponse, Error> {
    let response = Request::post(&format!("{}api/providers", APP_HOST))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .body(
            json!({
                "api_key": integration.clone().api_key,
                "name": integration.clone().name,
                "secret_key": integration.clone().secret_key,
                "status": integration.clone().status,
            })
            .to_string(),
        )
        .send()
        .await?;

    response.json::<IntegrationResponse>().await
}


pub async fn fetch_integrations(token: String, per_page: i32, current_page: i32, search_text: String) -> Result<IntegrationsResponse, Error> {
    let response = Request::get(&format!("{}api/providers?limit={}&name={}&page={}", APP_HOST, per_page, search_text, current_page))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send().await?;

    response.json::<IntegrationsResponse>().await
}

pub async fn update_integration(
    token: String,
    integration: Integration,
    integration_id: String
) -> Result<IntegrationResponse, Error> {
    let response = Request::put(&format!("{}api/providers/{}", APP_HOST, integration_id))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .body(
            json!({
                "api_key": integration.clone().api_key,
                "name": integration.clone().name,
                "secret_key": integration.clone().secret_key,
                "status": uppercase_first_letter(integration.clone().status),
            }).to_string()
        )
        .send().await?;

    response.json::<IntegrationResponse>().await
}

pub async fn fetch_integration(token: String, integration_id: String) -> Result<IntegrationResponse, Error> {
    let response = Request::get(&format!("{}api/providers/{}", APP_HOST, integration_id))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send().await?;

    response.json::<IntegrationResponse>().await
}

pub async fn delete_integration(token: String, integration_id: String) -> Result<IntegrationsDeleteResponse, Error> {
    let response = Request::delete(&format!("{}api/providers/{}", APP_HOST, integration_id))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send().await?;

    response.json::<IntegrationsDeleteResponse>().await
}