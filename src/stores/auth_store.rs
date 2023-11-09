use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
#[store(storage = "local", storage_tab_sync)]
pub struct AuthStore {
    pub token: String,
    pub is_authenticated: bool,
}

impl AuthStore {
    // Method to reset the store to default values
    pub fn reset_to_default(&mut self) {
        self.is_authenticated = false;
        self.token = String::default();
    }
}
