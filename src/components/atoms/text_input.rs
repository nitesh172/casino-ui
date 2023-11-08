use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;
// use yew_icons::{Icon, IconId};

#[derive(PartialEq)]
pub enum TextInputSize {
    Large,
    Small,
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
    pub icon_left: Html,
    #[prop_or_default]
    pub icon_right: Html,
}

impl Default for TextInputColor {
    fn default() -> Self {
        TextInputColor::Shade0
    }
}

impl TextInputProps {
    pub fn style_string(&self) -> String {
        match self.style {
            TextInputStyle::State1 => "bg-grey-shade-14 border border-grey-shade-11 rounded outline-0 text-grey-shade-6 text-14-20-300".to_owned(),
            TextInputStyle::State2 => todo!(),
            TextInputStyle::State3 => todo!(),
            TextInputStyle::State4 => todo!(),
            TextInputStyle::Success =>  "bg-grey-shade-14 border border-success rounded  outline-0 text-grey-shade-6 text-14-20-300".to_owned(),
            TextInputStyle::Error => todo!()
        }
    }

    pub fn label_color_string(&self) -> String {
        match self.label_color {
            TextInputColor::Shade0 => "text-grey-shade-0".to_owned(),
            TextInputColor::Shade6 => "text-grey-shade-6".to_owned(),
            TextInputColor::Success => "text-success".to_owned(),
            TextInputColor::Error => "text-error".to_owned(),
        }
    }

    pub fn size_string(&self) -> String {
        match self.size {
            TextInputSize::Small => "w-44 h-7 text-12-16-400".to_owned(),
            TextInputSize::Large => "w-72 h-10 text-16-21-400".to_owned(),
        }
    }
}

#[function_component]
pub fn TextInput(props: &TextInputProps) -> Html {
    let style_class = props.style_string();
    let size_class = props.size_string();
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
        <div class="">
            <label class={format!("{}", label_color_class)}>{&props.label}</label>
            <div class={format!("pl-2 pr-2 p-2 {} {} flex items-center", style_class, size_class)}>
                { props.icon_left.clone() }
                <input class={"outline-0 px-2 flex-1"} type="text" name={props.label.clone()} onchange={onchange} placeholder={props.placeholder.clone()}/>
                {props.icon_right.clone() }
            </div>
            <div class="text-gray-500 text-xs mt-2">{props.helper_text.clone()}</div>
        </div>
    }
}
