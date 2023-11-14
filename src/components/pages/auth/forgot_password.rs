// use gloo::console::log;
// use web_sys::HtmlInputElement;
use crate::{
    apis::user::api_forgot_password,
    components::{
        atoms::{
            button::{Button, ButtonStyle, ButtonType},
            label::{Label, LabelStyle},
            text_input::TextInput,
        },
        organisms::{auth_layout::AuthLayout, form_layout::FormLayout},
    },
    render_svg,
};
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
        <AuthLayout>
            <FormLayout
                        title="Forgot password?"
                        description="Enter the registered email ID"
                        submit_handler={on_submit.clone()}
            >
                <div class="space-y-4">
                        <div class="flex flex-col space-y-1.5">
                            <Label
                                label =  "Email"
                                label_for = "email"
                                label_style= {LabelStyle::Secondary}
                            />
                            <div class="flex items-center rounded border border-grey-shade-11 px-1">
                                <span>{html! { render_svg!("mdi:user", color="#949494" )}} </span>
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
                    </div>
                    <div>
                        <Button
                            label = "Submit"
                            button_type = {ButtonType::Submit}
                            button_style = {ButtonStyle::PrimaryFill}
                        />
                    </div>
            </FormLayout>
        </AuthLayout>
    }
}
