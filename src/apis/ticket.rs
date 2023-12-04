use super::APP_HOST;
use reqwasm::{ http::Request, Error };
use serde::{ Deserialize, Serialize };
use yew::Properties;
use serde_json::json;

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct TicketResponse {
    pub created_at: String,
    pub query: String,
    pub id: String,
    pub user_id: String,
    pub status: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct TicketsResponse {
    pub data: Vec<TicketResponse>,
    // pub page: i32,
    // pub per_page: i32,
    // pub total: i32,
    // pub total_pages: i32
}

#[derive(Clone, Properties, PartialEq, Default)]
pub struct UpdateTicket {
    pub id: String,
    pub status: String
}

pub async fn fetch_tickets(token: String) -> Result<TicketsResponse, Error> {
    let response = Request::get(&format!("{}api/tickets", APP_HOST))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send().await?;

    response.json::<TicketsResponse>().await
}

pub async fn update_ticket_status(token: String, ticket: UpdateTicket) -> Result<TicketResponse, Error> {
    let response = Request::put(&format!("{}api/tickets/status", APP_HOST))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .body(
            json!({
                "id": ticket.clone().id,
                "status": ticket.clone().status
            }).to_string()
        )
        .send().await?;

    response.json::<TicketResponse>().await
}