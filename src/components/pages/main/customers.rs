use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::organisms::paginator::Paginator, render_svg, routes::main_routes::MainRoute,
};

#[function_component(Customers)]
pub fn customers() -> Html {
    let history = use_navigator().unwrap();

    fn handle_row_click(_id: i32, history: Navigator) {
        history.push(&MainRoute::CustomerProfile {
            id: "12".to_owned(),
        })
    }

    html!(
        <>
            <div class="bg-grey-shade-13 py-4">
                <div class="container mx-auto md:w-auto space-y-4">
                    <div class="flex md:justify-between items-start md:items-center flex-col md:flex-row">
                        <div class="flex flex-col md:flex-row space-x-4  items-start md:items-center space-y-2 md:space-y-0">
                            <h1>{"Customers"}</h1>
                            <div class="flex items-center rounded border justify-start border-grey-shade-11 px-2 w-40">
                                <span>{html! { render_svg!("mynaui:search",  color="#000000", width="18px")}} </span>
                                <input
                                    id="search"
                                    autocomplete="off"
                                    name="search"
                                    placeholder={"search"}
                                    class="px-2.5 py-2 h-7 bg-white placeholder:text-grey-shade-6 text-14  leading-20 font-300 font-sans outline-none pr-2 pl-2 w-full "
                                />
                            </div>
                        </div>
                        <div class="flex space-x-4 items-center">
                            <button
                                class="bg-grey-shade-0 flex items-center rounded p-2 text-grey-shade-14 text-12 font-400 "
                            >
                                <span class="pr-2">
                                {html! { render_svg!("majesticons:file", color="#ffff",width="14px")}}
                                </span>
                                {"Export"}
                            </button>
                        </div>
                    </div>
                    <Paginator per_page={10} total_pages={1} total_items={5} current_page={1} />
                </div>
            </div>
            <div class="relative">
                <div class="absolute -z-10 top-0 left-0 h-[45px] w-full bg-grey-shade-13"></div>
                <div class="container mx-auto z-1">
                    <table class="w-full table-auto">
                        <thead class="bg-grey-shade-13">
                            <tr class="">
                                <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"ID"}</th>
                                <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Username"}</th>
                                <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Email ID"}</th>
                                <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Host IP"}</th>
                                <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Device"}</th>
                                <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Status"}</th>
                                <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Action"}</th>
                            </tr>
                        </thead>
                        <tbody class="overflow-y-auto">
                            <tr onclick={Callback::from(move |_| handle_row_click(12, history.clone()))}>
                                <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{"WWC001"}</td>
                                <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">{"Jack Cooper"}</td>
                                <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">{"jackcooper001@gmail.com"}</td>
                                <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">
                                    <p>
                                        {"196.69.80.124"}
                                    </p>
                                    <span class="text-12 text-grey-shade-5">{"India, Tamilnadu, chennai"}</span>
                                </td>
                                <td class="py-3 text-left text-14   font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">
                                    <p>
                                        {"IPhone X"}
                                    </p>
                                    <span class="text-12 text-grey-shade-5">{"Android 11, SM-G991U"}</span>
                                </td>
                                <td class="py-3 text-left text-14 font-medium text-grey-shade-14 tracking-wider">
                                    <span class="bg-warning rounded-full py-1 px-2 w-[24px]">
                                        {"Banned"}
                                    </span>
                                </td>
                                <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider relative group cursor-pointer">
                                    <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                    <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                        <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">
                                            <a href="#" class="">
                                            { "Ban" }
                                            </a>
                                        </li>
                                        <li  class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2 hover:bg-grey-shade-12">
                                            <a href="#" class="text-grey-shade-0 ">
                                                { "Unban" }
                                            </a>
                                        </li>
                                    </ul>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </>
    )
}
