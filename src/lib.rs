mod apis;
mod components;
mod routes;
mod stores;
mod utils;

use components::organisms::navbar::Navbar;
use gloo_console::log;
use routes::{ auth_routes::{ switch_auth, AuthRoute }, main_routes::{ switch_main, MainRoute } };
use stores::auth_store::AuthStore;
use yew_router::*;
use yewdux::prelude::*;
use yew::{ prelude::*, platform::spawn_local };
use crate::apis::user::api_me;

#[derive(Clone, Properties, PartialEq, Default)]
pub struct ToastProps {
    pub add_toast: Callback<String>,
}

#[function_component]
pub fn App() -> Html {
    let is_auth = use_state(|| false);
    let (auth_store, auth_dispatch) = use_store::<AuthStore>();

    let toasts = use_state(|| Vec::<String>::default());

    let token = auth_store.token.clone();
    let api_me_fn = move || {
        let token = token.clone();
        let store_dispatch = auth_dispatch.clone();
        spawn_local(async move {
            let response = api_me(token).await;

            match response {
                Ok(response) =>
                    store_dispatch.reduce_mut(move |store| {
                        store.current_user = response;
                    }),
                Err(e) => log!("Error: {}", e.to_string()),
            }
        });
    };

    let add_toast = {
        let toasts = toasts.clone();
        Callback::from(move |value: String| {
            let mut data = (*toasts).clone();
            data.push(value);
            toasts.set(data);
        })
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
            <ContextProvider<ToastProps> context={ToastProps {add_toast: add_toast.clone()}}>
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
            </ContextProvider<ToastProps>>
            <div class="absolute z-50 bottom-6 right-6 flex flex-col gap-2">
                {
                    toasts.clone().iter().map(|toast| {
                        html! {
                            <div class="hideMe bg-white shadow-lg px-2 py-1 border-l-4 border-primary">{toast.clone()}</div>
                        }
                    }).rev().collect::<Html>()
                }
            </div>
        </BrowserRouter>
    }
}
