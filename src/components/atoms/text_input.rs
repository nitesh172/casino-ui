use yew::prelude::*;

use crate::render_svg;

// use crate::components::atoms::icon::Icon;

#[derive(Properties, PartialEq, Clone)]
pub struct TextInputProps {
    pub id: AttrValue,
    pub value: AttrValue,
    pub input_type: AttrValue,
    pub input_handler: Callback<InputEvent>,
    #[prop_or_default]
    pub left_icon: String,
    #[prop_or_default]
    pub right_icon: String,
    #[prop_or_default]
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub helper_text: AttrValue,
    #[prop_or_default]
    pub show_password: AttrValue,
    #[prop_or_default]
    pub right_icon_click_handler: Option<Callback<MouseEvent>>,
}

impl TextInputProps {
    fn right_icon_click_handle(&self) -> Callback<MouseEvent> {
        match &self.right_icon_click_handler {
            Some(callback) => callback.clone(),
            None => Callback::noop(),
        }
    }
}

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    let id = &props.id;
    let value = &props.value;
    let placeholder = &props.placeholder;
    let input_handler = &props.input_handler;
    let input_type = &props.input_type;
    let right_icon_click_handle = props.right_icon_click_handle();

    html! {
        <>
            // <Icon
            //     label="mdi:user"
            //     color = "#ff0000"
            //     width="18px"
            //     height="18px"
            // />
            <span>{html! { render_svg!("mdi:user", color="#949494", width="18px")}} </span>

            <input
                id={id}
                autocomplete="off"
                name="password"
                placeholder={placeholder}
                value={value}
                oninput={input_handler}
                type={input_type}
                class="px-3.5 py-3 w-72 h-10 bg-white placeholder:text-grey-shade-6 text-14 leading-20 font-300 font-sans outline-none pr-2 pl-2"
            />
            // <Icon
            //     label="mdi:user"
            //     color = "#ff0000"
            //     width="18px"
            //     height="18px"
            //     on_click={right_icon_click_handle}
            // />

            { if props.right_icon.len() > 0  { html! {
            <button
                type="button"
                class="cursor-pointer"
                onclick={right_icon_click_handle}
            >
                {html! { render_svg!("mdi:eye", color="#949494" )}}
            </button>
            } } else {
                html!("")
            }}
        </>
    }
}
