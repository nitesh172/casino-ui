use crate::render_svg;
use gloo_console::log;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;

#[function_component(SetPassword)]
pub fn set_password() -> Html {
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

        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            password_handle.set(value.clone());

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

        Callback::from(move |event: MouseEvent| {
            if contains_number
                && contains_special
                && contains_uppercase
                && contains_length
                && passwords_match
            {
                log!("Good to go");
            }
        })
    };

    let contains_number_clone = (*contains_number).clone();
    let contains_uppercase_clone = (*contains_uppercase).clone();
    let contains_special_clone = (*contains_special).clone();
    let contains_length_clone = (*contains_length).clone();
    let password_match = (*passwords_match).clone();

    html! {
        <div class="flex min-h-screen bg-banner-woman bg-cover" >
            <div class="flex flex-col bg-white rounded-r px-4 justify-center w-screen md:px-16 md:w-auto">
                <form class="space-y-7">
                    <div class="space-y-3 max-w-xs">
                        <h1 class="text-24 leading-32 font-sans font-600 text-grey-shade-1">{"Set password "}</h1>
                        <p class="text-14 leading-20 font-sans font-400 text-grey-shade-5">{"Set new password"}</p>
                    </div>
                    <div class="space-y-4">
                        <div class="flex flex-col space-y-1.5">
                            <label
                                for="newpassword"
                                class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                            >
                                    {"New password"}
                            </label>
                            <div class="flex items-center rounded border justify-start border-grey-shade-11 px-2">
                               <span>{html! { render_svg!("mdi:key", color="#949494", width="18px")}} </span>
                                <input
                                    id="newpassword"
                                    name="newpassword"
                                    placeholder="New password"
                                    oninput={on_password_input}
                                    value={password.clone()}
                                    type={if *show_password { "password" } else { "text" }}
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
                            <div class="flex flex-col space-y-1.5">
                            <label
                                for="confirmpassword"
                                class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                            >
                                    {"Confirm password"}
                            </label>
                            <div class="flex items-center rounded border justify-start border-grey-shade-11 px-2">
                               <span>{html! { render_svg!("mdi:key", color="#949494", width="18px")}} </span>
                                <input
                                    id="confirmpassword"
                                    name="confirmpassword"
                                    placeholder="Confirm password"
                                    type="password"
                                    oninput={on_confirm_password_input}
                                    value={confirm_password.clone()}
                                    class="px-3.5 py-3
                                    w-72
                                    h-10
                                    bg-white
                                    placeholder:text-grey-shade-6 text-14 leading-20
                                    font-300 font-sans outline-none
                                    pr-2 pl-2"
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
                        <button
                            type="button"
                            onclick={onsubmit_handle}
                            class="cursor-pointer p-2 text-16 font-sans     font-400 text-grey-shade-14 leading-20 bg-primary w-full rounded"
                        >
                            {"Reset password"}
                        </button>
                    </div>

                </form>
            </div>
        </div>
    }
}

fn validate_contains_number(password: &str) -> bool {
    password.chars().any(|c| c.is_ascii_digit())
}

fn validate_contains_uppercase(password: &str) -> bool {
    password.chars().any(|c| c.is_ascii_uppercase())
}

fn validate_contains_special(password: &str) -> bool {
    password.chars().any(|c| !c.is_alphanumeric())
}

fn validate_length(password: &str, min_length: usize) -> bool {
    password.len() >= min_length
}
