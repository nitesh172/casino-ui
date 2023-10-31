mod components;
mod routes;

use routes::{
    app_routes::{switch_main, MainRoute},
    auth_routes::{switch_auth, AuthRoute},
};
use yew::prelude::*;
use yew_router::*;

enum AuthState {
    Authenticated,
    Unauthenticated,
}

#[function_component]
pub fn App() -> Html {
    let auth_state: AuthState = if is_user_authenticated() {
        AuthState::Authenticated
    } else {
        AuthState::Unauthenticated
    };

    html! {
        <BrowserRouter>
            { match auth_state {
                AuthState::Authenticated => {
                    html! { <Switch<MainRoute> render={switch_main} /> }
                }
                AuthState::Unauthenticated => {
                    html! { <Switch<AuthRoute> render={switch_auth} /> }
                }
            }}
        </BrowserRouter>
    }
}

fn is_user_authenticated() -> bool {
    false
}
