use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct AuthLayoutProps {
    pub children: Children,
}

#[function_component(AuthLayout)]
pub fn auth_layout(props: &AuthLayoutProps) -> Html {
    let children = &props.children;

    html! {
        <div class="flex min-h-screen bg-banner-woman bg-cover" >
            <div class="flex flex-col bg-white rounded-r px-4 justify-center w-screen md:px-16 md:w-auto">
                {children}
            </div>
        </div>
    }
}
