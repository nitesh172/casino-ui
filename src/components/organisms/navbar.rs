use crate::{
    render_svg,
    routes::main_routes::MainRoute::{
        self, Customers, Games, Integrations, Notifications, Overview, Participants, Settings,
        Teams, Tickets,
    },
};
// use crate::routes::main_routes::MainRoute::Overview;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let p = use_location();
    let route = p.unwrap().path().to_string();

    let is_open = use_state(|| false);

    let modal_handle = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };

    let overview_path = "/";
    let integrations_path = "/integrations";
    let participants_path = "/participants";

    html! {
        <div class="bg-black">
            <nav class="flex items-center justify-between py-6 text-grey-shade-14 px-2">
                <div>
                    <img src="img/logo.svg" class="h-34 md:h-42" />
                </div>
                <ul class="hidden md:flex space-x-6">
                    <li class={format!("rounded-3xl px-6 py-2.5 {} ", if route == overview_path{ " bg-grey-shade-2" } else { "" })}>
                        <Link<MainRoute> to={Overview}>{ "Overview" }</Link<MainRoute>>
                    </li>
                    <li class={format!("rounded-3xl px-6 py-2.5 {} ", if route == integrations_path{ " bg-grey-shade-2" } else { "" })}>
                        <Link<MainRoute> to={Integrations}>{ "Integrations" }</Link<MainRoute>>
                    </li>
                    <li class={format!("rounded-3xl px-6 py-2.5 {} ", if route == participants_path{ " bg-grey-shade-2" } else { "" })}>
                        <Link<MainRoute> to={Participants}>{ "Participants" }</Link<MainRoute>>
                    </li>
                    <li class="relative  group rounded-3xl px-6 py-2.5">
                        <div class="flex items-center space-x-2">
                            <span>{"More"}</span> <span>
                                {html! { render_svg!("bxs:down-arrow", color="#ffffff",width="10px")}}
                            </span>
                        </div>
                        <ul class="hidden absolute left-0 mt-1 space-y-2 bg-grey-shade-14 group-hover:block py-2 rounded-lg shadow-md shadow-grey-shade-0/10 z-40">
                            <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2 hover:bg-grey-shade-12 z-40">
                                <Link<MainRoute> to={Teams}>{ "Teams" }</Link<MainRoute>>
                            </li>
                            <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2 hover:bg-grey-shade-12 z-40">
                                <Link<MainRoute> to={Customers}>{ "Customers" }</Link<MainRoute>>
                            </li>
                            <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2 hover:bg-grey-shade-12 z-40">
                                <Link<MainRoute> to={Games}>{"Games"}</Link<MainRoute>>
                            </li>
                            <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2 hover:bg-grey-shade-12 z-40">
                                <Link<MainRoute> to={Tickets}>{"Tickets"}</Link<MainRoute>>
                            </li>
                            <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2 hover:bg-grey-shade-12 z-40">
                                <Link<MainRoute> to={Notifications}>{"Notifications"}</Link<MainRoute>>
                            </li>
                            <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2 hover:bg-grey-shade-12 z-40">
                                <Link<MainRoute> to={Settings}>{"Settings"}</Link<MainRoute>>
                            </li>
                        </ul>
                    </li>
                </ul>
                <div class="hidden md:block">
                    <img src="img/circle_profile.png" class="h-10" />
                </div>
                <div class="md:hidden">
                    <button
                        type="button"
                        onclick={modal_handle.clone()}
                        class={format!("z-40 block hamburger md:hidden focus:outline-none {}", if (*is_open).clone() {"open"} else {""})}
                    >
                        <span class="hamburger-top"></span>
                        <span class="hamburger-middle"></span>
                        <span class="hamburger-bottom"></span>
                    </button>
                </div>
                <ul
                    id="menu"
                    class={format!("fixed top-0 bottom-0 left-0 flex-col self-end w-full min-h-screen py-1 pt-32 pl-12 space-y-3 z-30 text-lg text-white uppercase bg-black {}", if (*is_open).clone() { "flex"} else {"hidden"})}
                >
                    <li class="hover:grey-shade-13">
                        <Link<MainRoute> to={Overview}>{ "Overview" }</Link<MainRoute>>
                    </li>
                    <li class="hover:grey-shade-13">
                        <Link<MainRoute> to={Integrations}>{ "Integrations" }</Link<MainRoute>>
                    </li>
                    <li class="hover:grey-shade-13">
                        <Link<MainRoute> to={Participants}>{ "Participants" }</Link<MainRoute>>
                    </li>
                    <li class="hover:grey-shade-13">
                        <Link<MainRoute> to={Teams}>{ "Teams" }</Link<MainRoute>>
                    </li>
                    <li class="hover:grey-shade-13">
                        <Link<MainRoute> to={Customers}>{ "Customers" }</Link<MainRoute>>
                    </li>
                    <li class="hover:grey-shade-13">
                        <Link<MainRoute> to={Games}>{"Games"}</Link<MainRoute>>
                    </li>
                    <li class="hover:grey-shade-13">
                        <Link<MainRoute> to={Tickets}>{"Tickets"}</Link<MainRoute>>
                    </li>
                    <li class="hover:grey-shade-13">
                        <Link<MainRoute> to={Notifications}>{"Notifications"}</Link<MainRoute>>
                    </li>
                    <li class="hover:grey-shade-13">
                        <Link<MainRoute> to={Settings}>{"Settings"}</Link<MainRoute>>
                    </li>
                </ul>
            </nav>
        </div>
    }
}
