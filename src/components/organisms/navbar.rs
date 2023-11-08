use crate::routes::main_routes::MainRoute::{
    self, Customers, Games, Integrations, Overview, Participants, Settings, Teams, Tickets,
};
use gloo_console::log;
// use crate::routes::main_routes::MainRoute::Overview;
use yew::prelude::*;
use yew_router::{prelude::*, router::RouterProps};

macro_rules! render_svg {
    ($($rest:expr),*) => {
        Html::from_html_unchecked(iconify::svg!($($rest),*).into())
    };
}

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let p = use_location();
    let route = p.unwrap().path().to_string();
    // log!(route.path().to_string());

    let overview_path = "/";
    let integrations_path = "/integrations";
    let participants_path = "/participants";

    html! {
        <nav class="bg-black">
        <div class="container mx-auto flex justify-between items-center px-4 py-6 text-grey-shade-14">
           <div>
           <img src="img/logo.svg" class="h-42" />
           </div>
           <ul class="flex space-x-6">
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
                <ul class="hidden absolute left-0 mt-1 space-y-2 bg-grey-shade-14 group-hover:block py-2 rounded-lg shadow-md shadow-grey-shade-0/10">
                    <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2 hover:bg-grey-shade-12 ">
                        <a href="#" class="">
                            <Link<MainRoute> to={Teams}>{ "Teams" }</Link<MainRoute>>
                        </a>
                    </li>
                    <li  class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2 hover:bg-grey-shade-12">
                        <a href="#" class="text-grey-shade-0 ">
                            <Link<MainRoute> to={Customers}>{ "Customers" }</Link<MainRoute>>
                        </a>
                    </li>
                    <li  class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2 hover:bg-grey-shade-12">
                        <a href="#" class="text-grey-shade-0 ">
                            <Link<MainRoute> to={Games}>{"Games"}</Link<MainRoute>>
                        </a>
                    </li>
                    <li  class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2 hover:bg-grey-shade-12">
                        <a href="#" class="text-grey-shade-0 ">
                            <Link<MainRoute> to={Tickets}>{"Tickets"}</Link<MainRoute>>
                        </a>
                    </li>
                    <li  class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2 hover:bg-grey-shade-12">
                        <a href="#" class="text-grey-shade-0 ">
                            <Link<MainRoute> to={Settings}>{"Settings"}</Link<MainRoute>>
                        </a>
                    </li>
                </ul>
            </li>

           </ul>
           <div>
           <img src="img/circle_profile.png" class="h-10" />
           </div>
        </div>
    </nav>
        }
}
