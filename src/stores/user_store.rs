use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
#[store(storage = "local", storage_tab_sync)]
pub struct UserStore {
    pub id: String,
    pub name: Option<String>,
    pub email_address: String,
    pub roles: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}
