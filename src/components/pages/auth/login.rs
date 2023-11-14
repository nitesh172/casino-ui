use crate::{
    apis::user::api_login,
    components::{
        atoms::{
            button::{Button, ButtonStyle, ButtonType},
            label::{Label, LabelStyle},
            text_input::TextInput,
        },
        organisms::{auth_layout::AuthLayout, form_layout::FormLayout},
    },
    render_svg,
    routes::auth_routes::AuthRoute::ForgotPassword,
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
            log!("here clicked");

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
                // let response = api_login(email, password).await;

                // match response {
                // Ok(response) =>
                store_dispatch.reduce_mut(move |store| {
                    store.is_authenticated = true;
                    store.token = "abc".to_owned() // response.token.clone();
                                                   // }),
                                                   // Err(e) => log!(e.to_string()),
                })
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

    let icon = "mdi:key".to_owned();

    html! {
                <AuthLayout>
                    <FormLayout
                        title="Login"
                        description="Enter your registered email ID and password"
                        submit_handler={on_submit.clone()}
                    >
                    <div class="space-y-4">
                        <div class="flex flex-col space-y-1.5">
                            <Label
                                label =  "Email"
                                label_for = "email"
                                label_style= {LabelStyle::Secondary}
                            />
                            <div class="flex items-center rounded border border-grey-shade-11 justify-start px-2" >

                                <TextInput
                                    id = "email"
                                    value = {email.clone()}
                                    input_type = "text"
                                    input_handler = {on_email_input}
                                    left_icon = "mid:user"
                                    placeholder = "Email address"
                                    helper_text = "enter a valid email address"
                                />
                            </div>
                        </div>
                        <div class="flex flex-col space-y-1.5">
                            <Label
                                label =  "Password"
                                label_for = "password"
                                label_style= {LabelStyle::Secondary}
                            />
                            <div class="flex items-center rounded border justify-start border-grey-shade-11 px-2">
                               <span>{html! { render_svg!(icon, color="#949494", width="18px")}} </span>

                               <TextInput

                                    id="password"
                                    value={password.clone()}
                                    placeholder="Password"
                                    helper_text="Enter an valid password"
                                    input_handler={on_password_input.clone()}
                                    input_type={if *show_password { "password" } else { "text" }}
                                    left_icon="mdi:key"
                                    right_icon_click_handler={toggle_password.clone()}
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
                            <Button
                                label = "Login"
                                button_type = {ButtonType::Submit}
                                button_style = {ButtonStyle::PrimaryFill}
                            />
                        </div>
                </FormLayout>
            </AuthLayout>
    }
}
