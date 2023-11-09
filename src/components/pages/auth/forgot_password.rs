// use gloo::console::log;
// use web_sys::HtmlInputElement;
use crate::{apis::user::api_forgot_password, render_svg};
use gloo_console::log;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*};

#[function_component(ForgotPassword)]
pub fn forgot_password() -> Html {
    let email_handle = use_state(String::default);

    let on_email_input = {
        let email_handle = email_handle.clone();
        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            email_handle.set(value);
        })
    };

    let on_submit = {
        let email = (*email_handle).clone();
        log!("Hii");
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            log!("Hii");
            if email.is_empty() {
                return;
            }

            let email: String = email.clone();
            spawn_local(async move {
                let response = api_forgot_password(email).await;

                match response {
                    Ok(response) => {
                        log!(response.message.to_string());
                    }
                    Err(e) => log!("Error: ", e.to_string()),
                }
            });
        })
    };

    let email = (*email_handle).clone();

    html! {
        <div class="flex min-h-screen bg-banner-woman bg-cover" >
            <div class="flex flex-col bg-white rounded-r px-4 justify-center w-screen md:px-16 md:w-auto">
                <form class="space-y-7" onsubmit={on_submit.clone()}>
                    <div class="space-y-3 max-w-xs">
                        <h1 class="text-24 leading-32 font-sans font-600 text-grey-shade-1">{"Forgot password?"}</h1>
                        <p class="text-14 leading-20 font-sans font-400 text-grey-shade-5">{"Enter the registered email ID"}</p>
                    </div>
                    <div class="space-y-4">
                        <div class="flex flex-col space-y-1.5">
                            <label
                                for="email"
                                class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                            >
                                    {"Email ID"}
                            </label>
                            <div class="flex items-center rounded border border-grey-shade-11 px-1">
                                <span>{html! { render_svg!("mdi:user", color="#949494" )}} </span>
                                <input
                                    id="email"
                                    name="email"
                                    placeholder="Email ID"
                                    oninput={on_email_input}
                                    value={email.clone()}
                                    class="px-3.5 py-3 w-80 placeholder:text-grey-shade-6 text-14 leading20
                                    bg-white
                                    h-10 
                                    border-grey-shade-11
                                    font-300 font-sans outline-none
                                    pr-2 pl-2"
                                />
                            </div>
                        </div>
                    </div>
                    <div>
                        <button
                            type="submit"
                            onsubmit={on_submit.clone()}
                            class="cursor-pointer p-2 text-16 font-sans     font-400 text-grey-shade-14 leading-20 bg-primary w-full rounded"
                        >
                            {"Submit"}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
