use yew::prelude::*;

use crate::render_svg;

#[derive(Properties, PartialEq, Clone)]
pub struct IconProps {
    pub label: String,
    pub color: String,
    pub width: String,
    pub height: String,
    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,
}

impl IconProps {
    fn handle_click(&self) -> Callback<MouseEvent> {
        match &self.on_click {
            Some(callback) => callback.clone(),
            None => Callback::noop(),
        }
    }
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    let label = &props.label;

    let color = props.color.to_string();
    let width = props.width.to_string();
    let height = props.height.to_string();
    let onclick = props.on_click.unwrap();

    html! {
        <span onclick={onclick}>
            {
                render_svg!(
                    label={label},
                    color={color},
                    width={width},
                    height={height}
                )
            }
        </span>
    }
}
