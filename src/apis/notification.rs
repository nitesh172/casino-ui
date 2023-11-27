use super::APP_HOST;
use reqwasm::{ http::Request, Error };
use serde::{ Deserialize, Serialize };
use serde_json::{ json };
use yew::Properties;

#[derive(Default, Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct Notification {
    pub description: String,
    pub ends_at: String,
    pub has_expiry_date: bool,
    pub is_email_enabled: bool,
    pub is_push_enabled: bool,
    pub is_sms_enabled: bool,
    pub starts_at: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct NotificationResponse {
    pub created_at: String,
    pub description: String,
    pub ends_at: String,
    pub has_expiry_date: bool,
    pub id: String,
    pub is_email_enabled: bool,
    pub is_push_enabled: bool,
    pub is_sms_enabled: bool,
    pub starts_at: String,
    pub status: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct NotificationsResponse {
    pub result: Vec<NotificationResponse>,
    pub page: i32,
    pub per_page: i32,
    pub total: i32,
    pub total_pages: i32
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct NotificationsDeleteResponse {
    pub message: String
}

pub fn format_date_for_backend(input_date: String) -> String {
    let formatted_date = format!("{}:00Z", input_date);
    formatted_date
}

pub async fn create_notification(
    notification: Notification
) -> Result<NotificationResponse, Error> {
    let response = Request::post(&format!("{}api/notifications", APP_HOST))
        .header("Content-Type", "application/json")
        .body(
            json!({
                "description": notification.clone().description,
                "ends_at": format_date_for_backend(notification.clone().ends_at),
                "has_expiry_date": notification.clone().has_expiry_date,
                "is_email_enabled": notification.clone().is_email_enabled,
                "is_push_enabled": notification.clone().is_push_enabled,
                "is_sms_enabled": notification.clone().is_sms_enabled,
                "starts_at": format_date_for_backend(notification.clone().starts_at)
            }).to_string()
        )
        .send().await?;

    response.json::<NotificationResponse>().await
}

pub async fn update_notification(
    notification: Notification,
    notification_id: String
) -> Result<NotificationResponse, Error> {
    let response = Request::put(&format!("{}api/notifications/{}", APP_HOST, notification_id))
        .header("Content-Type", "application/json")
        .body(
            json!({
                "description": notification.clone().description,
                "ends_at": format_date_for_backend(notification.clone().ends_at),
                "has_expiry_date": notification.clone().has_expiry_date,
                "is_email_enabled": notification.clone().is_email_enabled,
                "is_push_enabled": notification.clone().is_push_enabled,
                "is_sms_enabled": notification.clone().is_sms_enabled,
                "starts_at": format_date_for_backend(notification.clone().starts_at)
            }).to_string()
        )
        .send().await?;

    response.json::<NotificationResponse>().await
}

pub async fn fetch_notifications() -> Result<NotificationsResponse, Error> {
    let response = Request::get(&format!("{}api/notifications", APP_HOST))
        .header("Content-Type", "application/json")
        .send().await?;

    response.json::<NotificationsResponse>().await
}

pub async fn fetch_notification(notification_id: String) -> Result<NotificationResponse, Error> {
    let response = Request::get(&format!("{}api/notifications/{}", APP_HOST, notification_id))
        .header("Content-Type", "application/json")
        .send().await?;

    response.json::<NotificationResponse>().await
}

pub async fn delete_notification(notification_id: String) -> Result<NotificationsDeleteResponse, Error> {
    let response = Request::delete(&format!("{}api/notifications/{}", APP_HOST, notification_id))
        .header("Content-Type", "application/json")
        .send().await?;

    response.json::<NotificationsDeleteResponse>().await
}
