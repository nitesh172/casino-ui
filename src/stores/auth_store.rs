use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct AuthStore {
    pub token: String,
    pub is_authenticated: bool,
}
