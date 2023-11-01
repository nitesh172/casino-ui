use crate::components::pages::auth::login::Login;
use yew::{html, Html};
use yew_router::{prelude::Redirect, Routable};

#[derive(Clone, Routable, PartialEq)]
pub enum AuthRoute {
    #[at("/")]
    Login,
    #[at("/reset-password")]
    ResetPassword,
    #[at("/new-password")]
    SetNewPassword,
    // #[at("/new-password/*path")]
    // SetNewPasswordDynamic { path: String },
    #[at("/*path")]
    NotFound { path: String },
}

pub fn switch_auth(routes: AuthRoute) -> Html {
    match routes {
        AuthRoute::Login => html! { <Login /> },
        AuthRoute::ResetPassword => html! {<h1>{"Reset Password"}</h1>},
        AuthRoute::SetNewPassword => html! {<h1>{"Set new Password"}</h1>},
        // AuthRoute::SetNewPasswordDynamic { path } => {
        //     html! {<h1>{"Set new Password : "} {path}</h1>}
        // }
        AuthRoute::NotFound { path: _ } => html! {<Redirect<AuthRoute> to={AuthRoute::Login} /> },
    }
}
