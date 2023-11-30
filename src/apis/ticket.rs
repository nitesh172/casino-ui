use super::APP_HOST;
use reqwasm::{ http::Request, Error };
use serde::{ Deserialize, Serialize };
use yew::Properties;

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct TicketResponse {
    pub created_at: String,
    pub query: bool,
    pub id: String,
    pub user_id: String,
    pub status: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct TicketsResponse {
    pub result: Vec<TicketResponse>,
    pub page: i32,
    pub per_page: i32,
    pub total: i32,
    pub total_pages: i32
}

pub async fn fetch_tickets() -> Result<TicketsResponse, Error> {
    let response = Request::get(&format!("{}api/tickets", APP_HOST))
        .header("Content-Type", "application/json")
        .send().await?;

    response.json::<TicketsResponse>().await
}