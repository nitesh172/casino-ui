use yew::prelude::*;

#[derive(PartialEq)]
pub enum ButtonSize {
    Large,
    Small,
}

#[derive(PartialEq)]
pub enum ButtonStyle {
    PrimaryFilled,
    PrimaryStroked,
    PrimaryLink,
    SecondaryFilled,
    SecondaryStroked,
    SecondaryLink,
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub text: String,
    pub on_click: Callback<()>,
    pub style: ButtonStyle,
    pub size: ButtonSize,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub icon_left: Html,
    #[prop_or_default]
    pub icon_right: Html,
}

impl ButtonProps {
    pub fn style_string(&self) -> String {
        match self.style {
            ButtonStyle::PrimaryFilled => "bg-primary text-grey-shade-14".to_owned(),
            ButtonStyle::PrimaryStroked => {
                "bg-secondary text-primary border  border-primary".to_owned()
            }
            ButtonStyle::PrimaryLink => "bg-secondary text-primary ".to_owned(),
            ButtonStyle::SecondaryFilled => "bg-grey-shade-0 text-grey-shade-14".to_owned(),
            ButtonStyle::SecondaryStroked => {
                "text-grey-shade-0 border border-grey-shade-0".to_owned()
            }
            ButtonStyle::SecondaryLink => "text-grey-shade-0".to_owned(),
        }
    }

    pub fn size_string(&self) -> String {
        match self.size {
            ButtonSize::Small => "w-32 h-7 text-12-16-400".to_owned(),
            ButtonSize::Large => "w-44 h-10 text-16-21-400".to_owned(),
        }
    }
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let style_class = props.style_string();
    let size_class = props.size_string();

    let disabled_style = if props.disabled {
        match &props.style {
            ButtonStyle::PrimaryFilled => "opacity-50".to_owned(),
            _ => "".to_owned(),
        }
    } else {
        "".to_owned()
    };

    let onclick: Callback<()> = props.on_click.clone();
    let button_onclick: Callback<MouseEvent> = Callback::from(move |_| {
        onclick.emit(());
    });

    html! {
        <button
            class={format!("button rounded {} {} {} space-x-2", size_class, style_class, disabled_style)}
            onclick={button_onclick}
            disabled={props.disabled}>
            <div class="flex items-center justify-center space-x-2 ">
           { props.icon_left.clone() } <span>
            { &props.text }
        </span>
        { props.icon_right.clone() }
        </div>
        </button>
    }
}
