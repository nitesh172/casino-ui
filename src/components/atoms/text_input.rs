use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
// use yew_icons::{Icon, IconId};

#[derive(PartialEq)]
pub enum TextInputSize {
    Large,
    Small
}

#[derive(PartialEq)]
pub enum TextInputStyle {
    State1,
    State2,
    State3,
    State4,
    Success,
    Error,
}

#[derive(PartialEq)]
pub enum TextInputColor {
    Shade0,
    Shade6,
    Success,
    Error,
}

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub label: String,
    pub label_color: TextInputColor,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub helper_text: String,
    #[prop_or_default]
    pub helper_text_colder: TextInputColor,
    pub on_change: Callback<String>,
    pub style: TextInputStyle,
    pub size: TextInputSize,
    #[prop_or_default]
    pub icon_left:Html,
    #[prop_or_default] 
    pub icon_right: Html
}

impl Default for TextInputColor {
    fn default() -> Self {
        TextInputColor::Shade0
    }
}

impl TextInputProps {
    pub fn style_string(&self) -> String {
        match self.style {
            TextInputStyle::State1 => "bg-grey-shade-14 border border-grey-shade-11 rounded px-4 py-3 outline-0 text-grey-shade-6 text-14-20-300".to_owned(),
            TextInputStyle::State2 => todo!(),
            TextInputStyle::State3 => todo!(),
            TextInputStyle::State4 => todo!(),
            TextInputStyle::Success => todo!(),
            TextInputStyle::Error => todo!()
        }
    }

    pub fn label_color_string(&self )-> String {
        match self.label_color {
            TextInputColor::Shade0 => "text-grey-shade-0".to_owned(),
            TextInputColor::Shade6 => "text-grey-shade-6".to_owned(),
            TextInputColor::Success => "text-success".to_owned(),
            TextInputColor::Error => "text-error".to_owned()
        }
    }

    pub fn size_string(&self) -> String {
        match self.size {
            TextInputSize::Small => "w-44 h-7 text-12-16-400".to_owned(),
            TextInputSize::Large => "w-44 h-10 text-16-21-400".to_owned(),
        }
    }
}

#[function_component]
pub fn TextInput(props: &TextInputProps) -> Html {
    let style_class = props.style_string();
    let label_color_class = props.label_color_string();

    let handle_onchange = props.on_change.clone();

    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        handle_onchange.emit(value);
    });


    html! {
            <div class="p-4">
                <label class={format!("{}", label_color_class)}>{&props.label}</label>
                <div class="relative">
                    <span class="absolute inset-y-0 left-0 flex items-center pl-2">
                    { props.icon_left.clone() }
                    </span>
                    <input class={format!("pl-10 pr-8 p-2 {}", style_class)} type="text" name={props.label.clone()} onchange={onchange} placeholder={props.placeholder.clone()}/>
                    <span class="absolute inset-y-0 right-0 flex items-center pr-2 cursor-pointer">
                    { props.icon_left.clone() }
                    </span>
                </div>
                <div class="text-gray-500 text-xs mt-2">{props.helper_text.clone()}</div>
            </div>
        }
}
