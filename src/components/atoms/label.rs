use yew::prelude::*;

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum LabelStyle {
    Primary,
    Secondary,
}

#[derive(Clone, PartialEq, Properties)]
pub struct LabelProps {
    pub label_for: AttrValue,
    pub label: AttrValue,
    pub label_style: Option<LabelStyle>,
}

impl LabelProps {
    fn label_style_string(&self) -> String {
        let primary: Classes = "text-11 leading-25 font-sans font-400 text-primary"
            .split_whitespace()
            .collect();
        let secondary: Classes = "text-11 leading-25 font-sans font-400 text-grey-shade-0 "
            .split_whitespace()
            .collect();

        match self.label_style {
            Some(LabelStyle::Primary) => primary.to_string(),
            Some(LabelStyle::Secondary) => secondary.to_string(),
            _ => primary.to_string(),
        }
    }
}

#[function_component(Label)]
pub fn label(props: &LabelProps) -> Html {
    let label = &props.label;
    let label_for = &props.label_for;
    let label_stye = props.label_style_string();

    html! {
        <label
            for={label_for}
            class={classes!(label_stye)}
        >
            {label.clone()}
        </label>
    }
}
