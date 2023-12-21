use super::APP_HOST;
use crate::components::organisms::paginator::PaginationDataProps;
use reqwasm::{ http::Request, Error };
use serde::{ Deserialize, Serialize };
use serde_json::json;
use yew::Properties;

#[derive(Default, Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct Game {
    pub category_id: String,
    pub name: Vec<String>,
    pub provider_id: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct Category {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct Provider {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct GameResponse {
    pub created_at: String,
    pub id: String,
    pub updated_at: String,
    pub category: Category,
    pub provider: Provider,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct GamesResponse {
    pub result: Vec<GameResponse>,
    pub page: i32,
    pub per_page: i32,
    pub total: i32,
    pub total_pages: i32,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct GameDeleteResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct ProviderResponse {
    pub id: String,
    pub status: String,
    pub updated_at: String,
    pub api_key: String,
    pub name: String,
    pub secret_key: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct ProvidersResponse {
    pub result: Vec<ProviderResponse>,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct CategoryResponse {
    pub id: String,
    pub updated_at: String,
    pub name: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct CategoriesResponse {
    pub result: Vec<CategoryResponse>,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct CategoriesWithPagiantionResponse {
    pub result: Vec<CategoryResponse>,
}

pub async fn create_game(token: String, game: Game) -> Result<Vec<GameResponse>, Error> {
    let response = Request::post(&format!("{}api/games", APP_HOST))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .body(
            json!({
                "category_id": game.clone().category_id,
                "provider_id": game.clone().provider_id,
                "name": game.clone().name
            }).to_string()
        )
        .send().await?;

    response.json::<Vec<GameResponse>>().await
}

pub async fn fetch_games(
    token: String,
    pagination: PaginationDataProps,
    search_text: String
) -> Result<GamesResponse, Error> {
    let response = Request::get(
        &format!(
            "{}api/games?limit={}&page={}&name={}",
            APP_HOST,
            pagination.per_page,
            pagination.current_page,
            search_text
        )
    )
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send().await?;

    response.json::<GamesResponse>().await
}

pub async fn delete_game(token: String, game_id: String) -> Result<GameDeleteResponse, Error> {
    let response = Request::delete(&format!("{}api/games/{}", APP_HOST, game_id))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send().await?;

    response.json::<GameDeleteResponse>().await
}

pub async fn fetch_providers(token: String) -> Result<ProvidersResponse, Error> {
    let response = Request::get(&format!("{}api/providers?limit=1000", APP_HOST))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send().await?;

    response.json::<ProvidersResponse>().await
}

pub async fn fetch_categories(token: String) -> Result<CategoriesResponse, Error> {
    let response = Request::get(&format!("{}api/categories?limit=1000", APP_HOST))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send().await?;

    response.json::<CategoriesResponse>().await
}

pub async fn create_category(token: String, category: String) -> Result<CategoryResponse, Error> {
    let response = Request::post(&format!("{}api/categories", APP_HOST))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .body(json!({
                "name": category
            }).to_string())
        .send().await?;

    response.json::<CategoryResponse>().await
}

pub async fn delete_category(token: String, category_id: String) -> Result<GameDeleteResponse, Error> {
    let response = Request::delete(&format!("{}api/categories/{}", APP_HOST, category_id))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send().await?;

    response.json::<GameDeleteResponse>().await
}