use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[function_component]
pub fn TextInput() -> Html {
    html! {
            <div class="p-4 relative">
                <label class="block text-gray-500 text-sm font-semibold">{"Label"}</label>
                <div class="relative">
                    <span class="absolute inset-y-0 left-0 flex items-center pl-2">
    {html!  {<Icon icon_id={IconId::LucideHeart} height={"18px".to_owned()} /> } }

                    </span>
                    <input
                        type="text"
                        class="pl-10 pr-8 border p-2 rounded focus:outline-none focus:ring focus:border-blue-300"
                        placeholder="Placeholder"
                    />
                    <span class="absolute inset-y-0 right-0 flex items-center pr-2">

                    {html!  {<Icon icon_id={IconId::HeroiconsSolidClock} height={"18px".to_owned()} class="cursor-pointer"  /> } }

                    </span>
                </div>
                <div class="text-gray-500 text-xs mt-2">{"Helper text goes here"}</div>
            </div>
        }
}
