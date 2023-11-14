use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ButtonType {
    Submit,
    Button,
}

#[derive(Clone, PartialEq)]
pub enum ButtonStyle {
    PrimaryFill,
    PrimaryOutlined,
    PrimaryLink,
    SecondaryFill,
    SecondaryOutlined,
    SecondaryLink,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    pub label: AttrValue,
    pub button_type: Option<ButtonType>,
    pub button_style: Option<ButtonStyle>,
    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,
}

impl ButtonProps {
    fn button_type_string(&self) -> String {
        match self.button_type {
            Some(ButtonType::Submit) => "submit".to_owned(),
            _ => "button".to_owned(),
        }
    }

    fn handle_click(&self) -> Callback<MouseEvent> {
        match &self.on_click {
            Some(callback) => callback.clone(),
            None => Callback::noop(),
        }
    }

    fn button_style_string(&self) -> String {
        let default_style = "cursor-pointer p-2 rounded w-full focus-outline-none".to_owned();

        let primary_filled: Classes =
            " text-16 font-sans font-400 text-grey-shade-14 leading-20 bg-primary"
                .split_whitespace()
                .collect();
        let secondary_filled: Classes =
            "text-16 font-sans font-400 text-grey-shade-14 leading-20 bg-grey-shade-0 hover:shadow-lg hover:shadow-grey-shade-0/15 "
                .split_whitespace()
                .collect();

        match self.button_style {
            Some(ButtonStyle::PrimaryFill) => {
                format!("{} {}", default_style, primary_filled.to_string())
            }
            Some(ButtonStyle::SecondaryFill) => {
                format!("{} {}", default_style, secondary_filled.to_string())
            }
            _ => format!("{} {}", default_style, primary_filled.to_string()),
        }
    }
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let label = &props.label;
    let on_click = props.handle_click();
    let style = props.button_style_string();
    let butto_type = props.button_type_string();

    html! {
        <button
            onclick={on_click}
            type={butto_type}
            class={classes!(style)}
        >
           {label.clone()}
        </button>
    }
}
