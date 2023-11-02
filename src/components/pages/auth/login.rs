use yew::prelude::*;

#[function_component]
pub fn Login() -> Html {
    html! {
        <div class="flex h-screen bg-banner-woman bg-cover">
            <div class="w-1/4 flex p-8 bg-grey-shade-14 rounded-e items-center ">
                <div >
                        <h1 class="text-24-32-600 ">{"Login"}</h1>
                        <p class="text-14-20-400 text-grey-shade-5">{"Don't have an account "} <span class="text-primary">{"Sign Up"}</span></p>
                    </div>


                <form>
                    // <TextInput label="Username" />
                </form>
            </div>
            
        </div>
    }
}