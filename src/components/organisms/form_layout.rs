use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct FormLayoutProps {
    pub children: Children,
    pub title: String,
    pub description: String,
    pub submit_handler: Callback<SubmitEvent>,
}

#[function_component(FormLayout)]
pub fn form_layout(props: &FormLayoutProps) -> Html {
    let children = &props.children;
    let title = &props.title;
    let description = &props.description;
    let submit_handler = &props.submit_handler;

    html! {
        <form class="space-y-7" onsubmit={submit_handler}>
            <div class="space-y-3 max-w-xs">
                <h1 class="text-24 leading-32 font-sans font-600 text-grey-shade-1">{title}</h1>
                <p class="text-14 leading-20 font-sans font-400 text-grey-shade-5">{description}</p>
            </div>

            {children}

        </form>
    }
}
