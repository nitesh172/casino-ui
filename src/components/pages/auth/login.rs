use crate::{
    apis::user::api_login, render_svg, routes::auth_routes::AuthRoute::ForgotPassword,
    stores::auth_store::AuthStore,
};
use gloo_console::log;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    let password_state = use_state(String::default);
    let email_state = use_state(String::default);

    let show_password = use_state(|| false);
    let history = use_navigator().unwrap();

    let forgot_route_handler = {
        let history = history.clone();
        Callback::from(move |_| history.push(&ForgotPassword))
    };

    let on_email_input = {
        let email_state = email_state.clone();
        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            email_state.set(value);
        })
    };

    let on_password_input = {
        let password_state = password_state.clone();
        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            password_state.set(value);
        })
    };

    let (_, auth_dispatch) = use_store::<AuthStore>();

    let on_submit = {
        let email = (*email_state).clone();
        let password = (*password_state).clone();
        let store_dispatch = auth_dispatch.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            if email.is_empty() {
                return;
            }

            if password.is_empty() {
                return;
            }

            let email: String = email.clone();
            let password: String = password.clone();
            let store_dispatch = store_dispatch.clone();

            spawn_local(async move {
                let response = api_login(email, password).await;

                match response {
                    Ok(response) => store_dispatch.reduce_mut(move |store| {
                        store.is_authenticated = true;
                        store.token = response.token.clone();
                    }),
                    Err(e) => log!(e.to_string()),
                }
            });
        })
    };

    let toggle_password = {
        let show_password = show_password.clone();

        Callback::from(move |_| {
            show_password.set(!*show_password);
        })
    };

    let email = (*email_state).clone();
    let password = (*password_state).clone();

    html! {
        <div class="flex min-h-screen bg-banner-woman bg-cover" >
            <div class="flex flex-col bg-white rounded-r px-4 justify-center w-screen md:px-16 md:w-auto">
                <form class="space-y-7" onsubmit={on_submit.clone()}>
                    <div class="space-y-3 max-w-xs">
                        <h1 class="text-24 leading-32 font-sans font-600 text-grey-shade-1">{"Login"}</h1>
                        <p class="text-14 leading-20 font-sans font-400 text-grey-shade-5">{"Enter your registered email ID and password"}</p>
                    </div>
                    <div class="space-y-4">
                        <div class="flex flex-col space-y-1.5">
                            <label
                                for="email"
                                class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                            >
                                    {"Email ID"}
                            </label>
                             <div class="flex items-center rounded border border-grey-shade-11 justify-start px-2" >
                                <span>{html! { render_svg!("mdi:user", color="#949494" , width="18px")}} </span>
                                <input
                                    id="email"
                                    name="email"
                                    placeholder="Email ID"
                                    oninput={on_email_input}
                                    value={email.clone()}
                                    class="px-3.5 py-3placeholder:text-grey-shade-6 text-14 leading-20
                                    bg-white
                                    h-10 
                                    w-72
                                    border-grey-shade-11
                                    font-300 font-sans outline-none
                                    pr-2 pl-2"
                                />

                            </div>
                        </div>
                        <div class="flex flex-col space-y-1.5">
                            <label
                                for="password"
                                class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                            >
                                    {"Password"}
                            </label>
                            <div class="flex items-center rounded border justify-start border-grey-shade-11 px-2">
                               <span>{html! { render_svg!("mdi:key", color="#949494", width="18px")}} </span>
                                <input
                                    id="password"
                                    name="password"
                                    placeholder="Password"
                                    type={if *show_password { "password" } else { "text" }}
                                    oninput={on_password_input}
                                    value={password.clone()}
                                    class="px-3.5 py-3
                                    w-72
                                    h-10
                                    bg-white
                                    placeholder:text-grey-shade-6 text-14 leading-20
                                    font-300 font-sans outline-none
                                    pr-2 pl-2"
                                />
                                <button type="button" class="cursor-pointer" onclick={toggle_password}>{html! { render_svg!("mdi:eye", color="#949494" )}}</button>
                            </div>
                        </div>
                        <div class="flex items-center justify-center">
                            <button
                                type="button"
                                onclick={forgot_route_handler}
                                class="cursor-pointer p-2 text-12 font-sans font-400 text-primary leading-12"
                            >
                            {"Forgot password?"}
                            </button>
                        </div>
                    </div>
                    <div>
                        <button
                            onsubmit={on_submit.clone()}
                            type="submit"
                            class="cursor-pointer p-2 text-16 font-sans     font-400 text-grey-shade-14 leading-20 bg-primary w-full rounded"
                        >
                            {"Login"}
                        </button>
                    </div>

                </form>
            </div>
        </div>
    }
}
