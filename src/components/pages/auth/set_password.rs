use crate::{
    apis::user::api_reset_password,
    components::{
        atoms::{
            button::{Button, ButtonStyle, ButtonType},
            label::{Label, LabelStyle},
            text_input::TextInput,
        },
        organisms::{auth_layout::AuthLayout, form_layout::FormLayout},
    },
    render_svg,
    routes::auth_routes::AuthRoute,
    utils::string_validation::{
        validate_contains_number, validate_contains_special, validate_contains_uppercase,
        validate_length,
    },
};
use gloo_console::log;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::use_navigator;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub token: String,
}

#[function_component(SetPassword)]
pub fn set_password(props: &Props) -> Html {
    let password_handle: UseStateHandle<String> = use_state(String::default);
    let confirm_password_handle = use_state(String::default);

    let password = (*password_handle).clone();
    let confirm_password = (*confirm_password_handle).clone();

    let contains_uppercase = use_state(|| false);
    let contains_special = use_state(|| false);
    let contains_number = use_state(|| false);
    let contains_length = use_state(|| false);
    let passwords_match = use_state(|| true);

    let show_password = use_state(|| false);

    let on_password_input = {
        let contains_number_clone = contains_number.clone();
        let contains_uppercase_clone = contains_uppercase.clone();
        let contains_special_clone = contains_special.clone();
        let contains_length_clone = contains_length.clone();
        let passwords_match = passwords_match.clone();
        let confirm_password_handle = confirm_password_handle.clone();

        let password_handle_clone = password_handle.clone();

        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            password_handle_clone.set(value.clone());

            let is_uppercase = validate_contains_uppercase(&value);
            let is_special = validate_contains_special(&value);
            let is_number = validate_contains_number(&value);
            let is_length = validate_length(&value, 8);
            let password_match = value == (*confirm_password_handle).clone();

            contains_uppercase_clone.set(is_uppercase.clone());
            contains_number_clone.set(is_number.clone());
            contains_special_clone.set(is_special.clone());
            contains_length_clone.set(is_length.clone());
            passwords_match.set(password_match);
        })
    };

    let on_confirm_password_input = {
        let password = (password).clone();
        let confirm_password_handle: UseStateHandle<String> = (confirm_password_handle).clone();
        let passwords_match = (passwords_match).clone();

        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            confirm_password_handle.set(value.clone());

            passwords_match.set(password == value);
        })
    };

    let toggle_password = {
        let show_password = show_password.clone();

        Callback::from(move |_| {
            show_password.set(!*show_password);
        })
    };

    let onsubmit_handle = {
        let contains_number = (*contains_number).clone();
        let contains_uppercase = (*contains_uppercase).clone();
        let contains_special = (*contains_special).clone();
        let contains_length = (*contains_length).clone();
        let passwords_match = (*passwords_match).clone();
        let password: String = password.clone();
        let confirm_password = (*confirm_password_handle).clone();
        let password_reset_token = props.token.clone();
        let history = use_navigator().unwrap();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            if contains_number
                && contains_special
                && contains_uppercase
                && contains_length
                && passwords_match
            {
                let password = password.clone();
                let password_confirm = confirm_password.clone();
                let password_reset_token = password_reset_token.clone();
                let history = history.clone();

                spawn_local(async move {
                    let response =
                        api_reset_password(password_reset_token, password, password_confirm).await;

                    match response {
                        Ok(response) => {
                            log!(response.message.to_string());

                            history.replace(&AuthRoute::Login);
                        }
                        Err(e) => log!("Error: ", e.to_string()),
                    }
                });
            }
        })
    };

    let contains_number_clone = (*contains_number).clone();
    let contains_uppercase_clone = (*contains_uppercase).clone();
    let contains_special_clone = (*contains_special).clone();
    let contains_length_clone = (*contains_length).clone();
    let password_match = (*passwords_match).clone();

    html! {
        <AuthLayout>
                    <FormLayout
                        title="Set password"
                        description="Set new password"
                        submit_handler={onsubmit_handle.clone()}
                    >
                    <div class="space-y-4">
                        <div class="flex flex-col space-y-1.5">
                            <Label
                                label =  "New password"
                                label_for = "newpassword"
                                label_style= {LabelStyle::Secondary}
                            />
                            <div class="flex items-center rounded border justify-start border-grey-shade-11 px-2">
                               <span>{html! { render_svg!("mdi:key", color="#949494", width="18px")}} </span>
                                <TextInput
                                    id="newpassword"
                                    value={password.clone()}
                                    placeholder="New password"
                                    helper_text="Enter an valid password"
                                    input_handler={on_password_input.clone()}
                                    input_type={if *show_password { "password" } else { "text" }}
                                    left_icon="mdi:key"
                                    right_icon_click_handler={toggle_password.clone()}
                                />
                                <button type="button" class="cursor-pointer" onclick={toggle_password}>{html! { render_svg!("mdi:eye", color="#949494" )}}</button>
                            </div>
                            </div>
                            <div class="flex flex-col space-y-1.5">
                            <Label
                            label =  "Confirm password"
                            label_for = "confirmpassword"
                            label_style= {LabelStyle::Secondary}
                        />
                            <div class="flex items-center rounded border justify-start border-grey-shade-11 px-2">
                               <span>{html! { render_svg!("mdi:key", color="#949494", width="18px")}} </span>

                                <TextInput
                                    id="confirmpassword"
                                    value={confirm_password.clone()}
                                    placeholder="Confirm password"
                                    helper_text="Enter an valid password"
                                    input_handler={on_confirm_password_input.clone()}
                                    input_type="password"
                                />

                            </div>
                            { if !password_match {
                                html! { <p class="text-primary text-11 font-400">{ "Passwords do not match." }</p> }
                            }else {
                                    html! {}
                            }}
                        </div>
                        </div>
                        <div class="flex flex-col justify-center items-start text-11 font-400 font-sans leading-15 text-grey-shade-2 space-y-2">
                            <p>{"Must have: "}</p>
                            <ul class="space-y-2">
                                <li class="flex items-center space-x-2"><span>
                                {
                                    if contains_length_clone {
                                    html! { render_svg!("ph:check-circle-fill", color="#83BF94", width="18px", height="18px" )}
                                } else {
                                    html! { render_svg!("ph:check-circle-fill", color="#949494", width="18px", height="18px" )}
                                } }
                                </span><span>{"8 characters"}</span></li>

                                <li class="flex items-center space-x-2"> <span>
                                {
                                    if contains_uppercase_clone {
                                    html! { render_svg!("ph:check-circle-fill", color="#83BF94", width="18px", height="18px" )}
                                } else {
                                    html! { render_svg!("ph:check-circle-fill", color="#949494", width="18px", height="18px" )}
                                } }
                                </span><span>{"One capital letter"}</span></li>

                                <li class="flex items-center space-x-2">
                                <span>
                                {
                                    if contains_special_clone {
                                    html! { render_svg!("ph:check-circle-fill", color="#83BF94", width="18px", height="18px" )}
                                } else {
                                    html! { render_svg!("ph:check-circle-fill", color="#949494", width="18px", height="18px" )}
                                } }
                                </span><span>{"One special character"}</span></li>

                                <li class="flex items-center space-x-2">
                                <span>
                                {
                                    if contains_number_clone {
                                    html! { render_svg!("ph:check-circle-fill", color="#83BF94", width="18px", height="18px" )}
                                } else {
                                    html! { render_svg!("ph:check-circle-fill", color="#949494", width="18px", height="18px" )}
                                } }
                                </span><span>{"One number"}</span></li>

                            </ul>
                        </div>

                    <div>
                        <Button
                            label = "Reset password"
                            button_type = {ButtonType::Submit}
                            button_style = {ButtonStyle::PrimaryFill}
                        />
                    </div>
               </FormLayout>
            </AuthLayout>
    }
}
