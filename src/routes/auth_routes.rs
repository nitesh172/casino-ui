use crate::components::pages::auth::login::Login;
use crate::components::pages::auth::{forgot_password::ForgotPassword, set_password::SetPassword};
use yew::{html, Html};
use yew_router::{prelude::Redirect, Routable};

#[derive(Clone, Routable, PartialEq)]
pub enum AuthRoute {
    #[at("/")]
    Login,
    #[at("/forgot-password")]
    ForgotPassword,
    #[at("/set-password")]
    SetNewPassword,
    // #[at("/new-password/*path")]
    // SetNewPasswordDynamic { path: String },
    #[at("/*path")]
    NotFound { path: String },
}

pub fn switch_auth(routes: AuthRoute) -> Html {
    match routes {
        AuthRoute::Login => html! { <Login /> },
        AuthRoute::ForgotPassword => html! {<ForgotPassword />},
        AuthRoute::SetNewPassword => html! {< SetPassword />},
        AuthRoute::NotFound { path: _ } => html! {<Redirect<AuthRoute> to={AuthRoute::Login} /> },
    }
}
