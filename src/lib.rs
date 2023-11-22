mod apis;
mod binding;
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
use yew::prelude::*;
use yew_router::*;
use yewdux::prelude::*;

#[function_component]
pub fn App() -> Html {
    let is_auth = use_state(|| false);
    let (store, _) = use_store::<AuthStore>();

    {
        let is_auth = is_auth.clone();
        let is_authenticated = store.is_authenticated.clone();

        use_effect_with(is_authenticated, move |_| {
            log!("Ui rendered");
            if is_authenticated {
                is_auth.set(true);
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
