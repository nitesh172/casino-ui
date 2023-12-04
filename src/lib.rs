mod apis;
mod components;
mod routes;
mod stores;
mod utils;

use components::organisms::navbar::Navbar;
use gloo_console::log;
use routes::{
    auth_routes::{switch_auth, AuthRoute},
    main_routes::{switch_main, MainRoute},
};
use stores::auth_store::AuthStore;
use yew_router::*;
use yewdux::prelude::*;
use yew::{prelude::*, platform::spawn_local};
use crate::apis::user::api_me;

#[function_component]
pub fn App() -> Html {
    let is_auth = use_state(|| false);
    let (auth_store, auth_dispatch) = use_store::<AuthStore>();

    let token = auth_store.token.clone();
    let api_me_fn = move || {
        let token = token.clone();
        let store_dispatch = auth_dispatch.clone();
        spawn_local(async move {
            let response = api_me(token).await;

            match response {
                Ok(response) => store_dispatch.reduce_mut(move |store| {
                    store.current_user = response;
                }),
                Err(e) => log!("Error: {}", e.to_string()),
            }
        });
    };

    {
        let is_auth = is_auth.clone();
        let is_authenticated = auth_store.is_authenticated.clone();

        use_effect_with(is_authenticated, move |_| {
            log!("Ui rendered");
            if is_authenticated {
                is_auth.set(true);
                api_me_fn();
            } else {
                is_auth.set(false);
            }

            || {}
        });
    }

    let is_auth_state = (*is_auth).clone();
    html! {
        <BrowserRouter>
            { match is_auth_state {
                true => {
                    html! {
                        <>
                        <Navbar />
                        <Switch<MainRoute> render={switch_main} />
                        </>
                    }
                }
                false => {
                    html! { <Switch<AuthRoute> render={switch_auth} /> }
                }
            }}
        </BrowserRouter>
    }
}
