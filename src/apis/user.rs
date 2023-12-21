use super::APP_HOST;
use reqwasm::{ http::Request, Error };
use serde::{ Deserialize, Serialize };
use serde_json::json;
use web_sys::File;
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
pub struct ImageURL {
    pub s3_key: String,
    pub signed_url: String,
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
    pub last_login_at: String,
    pub image_url: ImageURL,
}

#[derive(Serialize, Deserialize, Clone, Eq, Properties, PartialEq, Default)]
pub struct UpdateProfile {
    pub id: String,
    pub name: Option<String>,
    pub image_url: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct UserDeleteResponse {
    pub message: String,
}

#[derive(Default, Clone, Properties, PartialEq)]
pub struct UserNotification {
    pub is_email_enabled: bool,
    pub is_push_enabled: bool,
    pub is_sms_enabled: bool,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct UserNotificationResponse {
    pub is_email_enabled: bool,
    pub is_push_enabled: bool,
    pub is_sms_enabled: bool,
    pub created_at: String,
    pub id: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct ErrorMessage {
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct S3Object {
    pub s3_key: String,
    pub presigned_url: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct Media {
    pub mime_type: String,
    pub relative_path: String,
}

pub async fn api_login(email: String, password: String) -> Result<LoginResponse, String> {
    let response = match
        Request::post(&format!("{}api/auth/login", APP_HOST))
            .header("Content-Type", "application/json")
            .body(
                json!({
                "email_address": email,
                "password": password
            }).to_string()
            )
            .send().await
    {
        Ok(res) => res,
        Err(_) => {
            return Err("Failed to make request".to_string());
        }
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorMessage>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<LoginResponse>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_forgot_password(email: String) -> Result<ForgotPasswordResponse, Error> {
    let response = Request::post(&format!("{}api/auth/forgot-password", APP_HOST))
        .header("Content-Type", "application/json")
        .body(json!({
                "email_address": email
            }).to_string())
        .send().await?;

    response.json::<ForgotPasswordResponse>().await
}

pub async fn api_reset_password(
    password_reset_token: String,
    password: String,
    password_confirm: String
) -> Result<PasswordResetResponse, Error> {
    let response = Request::patch(
        &format!("{}api/auth/reset-password/{}", APP_HOST, password_reset_token)
    )
        .header("Content-Type", "application/json")
        .body(
            json!({
            "password": password,
            "password_confirm": password_confirm
        }).to_string()
        )
        .send().await?;

    response.json::<PasswordResetResponse>().await
}

pub async fn api_update_user(token: String, user: UpdateProfile) -> Result<CurrentUser, Error> {
    let response = Request::put(&format!("{}api/users/{}", APP_HOST, user.id.clone()))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .body(
            if !user.image_url.is_empty() {
                json!({
                    "name": user.name,
                    "image_url": user.image_url.to_string()
                }).to_string()
            } else {
                json!({
                    "name": user.name,
                }).to_string()
            }
        )
        .send().await?;

    response.json::<CurrentUser>().await
}

pub async fn api_delete_user(token: String, user_id: String) -> Result<UserDeleteResponse, Error> {
    let response = Request::delete(&format!("{}api/users/{}", APP_HOST, user_id.clone()))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send().await?;

    response.json::<UserDeleteResponse>().await
}

pub async fn api_me(token: String) -> Result<CurrentUser, Error> {
    let response = Request::get(&format!("{}api/users/me", APP_HOST))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send().await?;

    // let data = response.json::<CurrentUser>().await.unwrap();
    // log!(serde_json::to_string_pretty(&data).unwrap());

    response.json::<CurrentUser>().await
}

pub async fn update_user_notification(
    token: String,
    notification: UserNotification
) -> Result<UserNotificationResponse, Error> {
    let response = Request::put(&format!("{}api/user-notifications", APP_HOST))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .body(
            json!({
                "is_push_enabled": notification.is_push_enabled,
                "is_email_enabled": notification.is_email_enabled,
                "is_sms_enabled": notification.is_sms_enabled
            }).to_string()
        )
        .send().await?;

    response.json::<UserNotificationResponse>().await
}

pub async fn fetch_user_notification(token: String) -> Result<UserNotificationResponse, Error> {
    let response = Request::get(&format!("{}api/user-notifications", APP_HOST))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send().await?;

    response.json::<UserNotificationResponse>().await
}

pub async fn update_email(token: String, email_address: String) -> Result<CurrentUser, Error> {
    let response = Request::put(&format!("{}api/users/email-updation", APP_HOST))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .body(json!({
                "email_address": email_address
            }).to_string())
        .send().await?;

    response.json::<CurrentUser>().await
}

pub async fn upload_file(media: Media) -> Result<S3Object, Error> {
    let response = Request::get(
        &format!(
            "{}api/media/upload-signature?mime_type={}&relative_path={}",
            APP_HOST,
            media.mime_type,
            media.relative_path
        )
    )
        .header("Content-Type", "application/json")
        .send().await?;

    response.json::<S3Object>().await
}

pub async fn save_file_url(presigned_url: String, file: File) -> Result<bool, Error> {
    let response = Request::put(&format!("{}", presigned_url))
        .header("Content-Type", &file.type_())
        .body(file)
        .send().await?;

    response.json::<bool>().await
}
