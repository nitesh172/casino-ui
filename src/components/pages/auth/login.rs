use crate::components::atoms::{
    button::{Button, ButtonSize, ButtonStyle},
    text_input::TextInput,
};
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[function_component]
pub fn Login() -> Html {
    let sumbit_handler = { Callback::from(move |_| {}) };

    html! {
        <div >
            <h1>{"Login page"}</h1>
            <div class="flex flex-row space-x-2 justify-center items-center">
            <Button text={"Login"} style={ButtonStyle::PrimaryFilled} size={ButtonSize::Large} on_click={&sumbit_handler}
            icon_left={html! {<Icon icon_id={IconId::LucideHeart} height={"18px".to_owned()}  />} }
            icon_right={html!  {<Icon icon_id={IconId::HeroiconsSolidClock} height={"18px".to_owned()} /> } }
            />
            <Button text={"Login"} style={ButtonStyle::PrimaryFilled} size={ButtonSize::Large} on_click={&sumbit_handler}  disabled={true}    icon_left={html! {<Icon icon_id={IconId::LucideHeart} height={"18px".to_owned()}  />} }
            icon_right={html!  {<Icon icon_id={IconId::HeroiconsSolidClock} height={"18px".to_owned()} /> } } />
            <Button text={"Login"} style={ButtonStyle::PrimaryStroked} size={ButtonSize::Large} on_click={&sumbit_handler} />
            <Button text={"Login"} style={ButtonStyle::PrimaryLink} size={ButtonSize::Large} on_click={&sumbit_handler}  />

            <Button text={"Login"} style={ButtonStyle::SecondaryFilled} size={ButtonSize::Large} on_click={&sumbit_handler}/>
            <Button text={"Login"} style={ButtonStyle::SecondaryStroked} size={ButtonSize::Small} on_click={&sumbit_handler} />
            </div>

            <div class="flex flex-row space-x-2 justify-center items-center">
            <TextInput  />

            </div>
        </div>
    }
}
