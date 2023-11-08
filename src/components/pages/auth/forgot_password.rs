// use gloo::console::log;
// use web_sys::HtmlInputElement;
use crate::render_svg;
use yew::prelude::*;

#[function_component(ForgotPassword)]
pub fn forgot_password() -> Html {
    // let password_handle: UseStateHandle<bool> = use_state(bool::default);
    // let password: bool = (*is_password_hidden_state).clone();

    // let onclick_handle ={
    //     let is_password_hidden_state = is_password_hidden_state.clone();

    //     Callback::from(|event: MouseEvent| {
    //       let input = e.target_dyn_into::<HtmlInputElement>();

    //       if let Some(input) = input {
    //         is_password_hidden.set(input.value());
    //       }
    //     })
    // };

    html! {
        <div class="flex min-h-screen bg-banner-woman bg-cover" >
            <div class="flex flex-col bg-white rounded-r px-4 justify-center w-screen md:px-16 md:w-auto">
                <form class="space-y-7">
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
                            type="button"
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
