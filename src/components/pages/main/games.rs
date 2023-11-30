use crate::{
    components::organisms::{
        export_button::ExportButton,
        paginator::{PaginationDataProps, PaginationFucProps, Paginator},
    },
    render_svg,
};
use std::ops::Deref;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;

#[function_component(Games)]
pub fn games() -> Html {
    let is_open = use_state(|| false);
    let search_text = use_state(|| String::default());
    let initial = use_state(|| true);
    let pagination = use_state(|| PaginationDataProps {
        per_page: 10,
        total_items: 0,
        total_pages: 0,
        current_page: 1,
    });

    let modal_handle = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };

    let update_pagination = {
        let pagination = pagination.clone();
        let cloned_initial = initial.clone();
        Callback::from(move |option: PaginationFucProps| {
            cloned_initial.set(true);
            let mut data = pagination.deref().clone();
            let name = option.name;
            let value = option.value;

            match name.as_str() {
                "current_page" => {
                    data.current_page = value;
                }
                "per_page" => {
                    data.per_page = value;
                }
                "total_items" => {
                    data.total_items = value;
                }
                "total_pages" => {
                    data.total_pages = value;
                }
                _ => (),
            }
            pagination.set(data);
        })
    };

    let cloned_search_text = search_text.clone();
    let cloned_initial = initial.clone();
    let search_text_changed = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        cloned_initial.set(true);
        cloned_search_text.set(value);
    });

    let t_head = vec![
        "Game name",
        "Category",
        "Provider",
        "Actions"
    ];

    #[derive(Clone, Properties, PartialEq)]
    pub struct GameProps {
        pub game_name: String,
        pub provider: String,
        pub category: String,
    }

    let games = vec![
        GameProps {
            game_name: "Roulet".into(),
            provider: "EVOLUTION".into(),
            category: "Table Games".into(),
        },
        GameProps {
            game_name: "EZUGI".into(),
            provider: "25.50".into(),
            category: "Table Games".into(),
        },
        GameProps {
            game_name: "EZUGI".into(),
            provider: "POWER GAMES".into(),
            category: "Table Games".into(),
        }
    ];

    let export = {
        // let notifications = notifications.clone();
        Callback::from(move |_: MouseEvent| {
            // let mut csv_data =
            //     "Notification Message|Status|Offer available from|Offer available until|Created on"
            //         .to_string();

            // for notification in notifications.iter() {
            //     let data: String = format!(
            //         "\n{}|{}|{}|{}|{}",
            //         notification.clone().description.as_str(),
            //         notification.clone().status,
            //         format_date_to_readable(notification.clone().starts_at, "%Y-%m-%d"),
            //         format_date_to_readable(notification.clone().ends_at, "%Y-%m-%d"),
            //         format_date_to_readable(notification.clone().created_at, "%Y-%m-%d")
            //     );
            //     csv_data = csv_data + data.as_str();
            // }

            // download_csv_file(csv_data.as_str())
        })
    };

    html! {
        <>
        <div class="bg-grey-shade-13 px-8 py-4">
            <div class="container mx-auto md:w-auto space-y-4">
                <div class="flex md:justify-between md:items-center flex-col gap-6 md:flex-row">
                    <div class="flex flex-row justify-between md:justify-normal gap-2 items-center w-full md:w-auto">
                        <h1>{"Games"}</h1>
                        <div class="flex items-center rounded border justify-start bg-white border-grey-shade-11 px-2 w-40">
                            <span>{html! { render_svg!("mynaui:search",  color="#000000", width="18px")}} </span>
                            <input
                                id="search"
                                autocomplete="off"
                                name="search"
                                placeholder={"search"}
                                onchange={search_text_changed.clone()}
                                class="px-2.5 py-2 h-7 bg-white placeholder:text-grey-shade-6 text-14  leading-20 font-300 font-sans outline-none pr-2 pl-2 w-full "
                            />
                        </div>
                    </div>
                    <div class="flex flex-row items-center w-full md:w-auto gap-2">
                        <ExportButton export={export.clone()} />
                        <button
                            onclick={modal_handle.clone()}
                            class="bg-primary flex flex-1 md:flex-none items-center rounded justify-center lg:justify-normal p-2 text-grey-shade-14 text-12 font-400"
                        >
                            <span class="pr-2">
                            {html! { render_svg!("lets-icons:add-round", color="#ffff",width="14px")}}
                            </span>
                            {"Create"}
                        </button>
                    </div>
                </div>
                <ContextProvider<PaginationDataProps> context={(*pagination).clone()}>
                    <Paginator update_pagination={update_pagination.clone()} />
                </ContextProvider<PaginationDataProps>>
            </div>
        </div>
        <div class="relative px-8">
        <div class="absolute hidden xl:block -z-10 top-0 left-0 h-[45px] w-full bg-grey-shade-13"></div>
        <div class="container mx-auto">
            <table class="w-full table-auto hidden xl:inline-table">
                <thead class="bg-grey-shade-13">
                    <tr>
                        {
                            t_head.clone().into_iter().map(|name| {
                                html!{<th key={name} class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{name}</th>}
                            }).collect::<Html>()
                        }
                    </tr>
                </thead>
                <tbody class="overflow-y-auto">
                    {
                        games.clone().iter().map(|games| {
                            html!{
                                <tr>
                                    <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{games.clone().game_name}</td>
                                    <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider">
                                        <span class="bg-grey-shade-11 rounded-full py-1 px-2">
                                            {games.clone().category}
                                        </span>
                                    </td>
                                    <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">{games.clone().provider}</td>
                                    <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider relative group cursor-pointer">
                                        <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                        <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                            <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">
                                                <a href="#" class="">
                                                { "Remove" }
                                                </a>
                                            </li>
                                        </ul>
                                    </td>
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                </tbody>
            </table>
            <div class="grid md:grid-cols-2 lg:grid-cols-3 xl:hidden gap-4 mt-4">
                {
                    games.clone().iter().map(|game| {
                        html!{
                            <div class="rounded-xl border p-4 flex flex-col justify-between gap-4">
                                <div class="flex flex-row justify-between">
                                    <div class="flex flex-row gap-3 justify-between">
                                        <div class="flex flex-col">
                                            <div class="text-12 text-grey-shade-5">{"Game name"}</div>
                                            <div class="text-14 font-400">{game.clone().game_name}</div>
                                        </div>
                                        <div class="flex flex-col">
                                            <div class="text-12 text-grey-shade-5">{"Provider"}</div>
                                            <div class="text-14 font-400">{game.clone().provider}</div>
                                        </div>
                                    </div>
                                    <div class="text-12 font-medium text-grey-shade-1 relative group cursor-pointer">
                                        <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                        <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                            <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">{"Remove"}</li>
                                        </ul>
                                    </div>
                                </div>
                                <div class="rounded-full py-1 px-2 text-12 flex-row gap-1 w-fit bg-grey-shade-11">
                                    {game.clone().category}
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
        </div>
        { if *is_open {html! {<Modal modal_handle={modal_handle.clone()}  />}  } else { html! ("") } }
        </>
    }
}

#[derive(Properties, PartialEq)]
struct ModalProps {
    modal_handle: Callback<MouseEvent>,
}

#[function_component(Modal)]
fn edit_modal(props: &ModalProps) -> Html {
    html! {
        <div class="z-10 fixed inset-0 bg-grey-shade-0/70 w-screen flex h-screen items-center justify-center p-5">
            <div class=" bg-white p-7 rounded-sm space-y-7 w-full md:w-auto">
                <div class="flex items-center justify-between">
                    <p>{"Add game"}</p>
                    <span class="cursor-pointer" onclick={props.modal_handle.clone()}>
                        {html! {render_svg!("mdi:multiply",color="#232323",width="25px")}}
                    </span>
                </div>

                <div class="flex flex-col space-y-1.5 md:w-[600px]">
                    <label
                        for="game"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"Platform name"}
                    </label>
                    <div class="relative inline-flex w-full ">
                        <select class="appearance-none border border-gray-300 rounded px-3 py-2 focus:outline-none w-full ">
                            <option>{"Evolution"}</option>
                            <option>{"Ezugi"}</option>
                            <option>{"Qtech"}</option>
                        </select>
                        <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
                            <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 11.586l3.293-3.293a1 1 0 011.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
                            </svg>
                        </div>
                    </div>
                </div>
                <div class="flex flex-col space-y-1.5 md:w-[600px]">
                    <label
                        for="category"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"Category"}
                    </label>
                    <div class="relative inline-flex w-full ">
                        <select  id="category" class="appearance-none border border-gray-300 rounded px-3 py-2 focus:outline-none w-full ">
                            <option>{"Table games"}</option>
                            <option>{"Slot machines"}</option>
                            <option>{"Video poker"}</option>
                        </select>
                        <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
                            <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 11.586l3.293-3.293a1 1 0 011.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
                            </svg>
                        </div>
                    </div>
                </div>
                <div class="md:w-[600px]">
                <ul class="flex flex-wrap w-full gap-2 items-baseline justify-start">
                    <li class="bg-grey-shade-2 w-fit text-grey-shade-14 text-12 px-3 py-2 rounded-lg">
                        {"Blackjack"}
                    </li>
                    <li class="bg-grey-shade-2 text-grey-shade-14 w-fit text-12 px-3 py-2 rounded-lg">{"Roulette"}</li>
                    <li class="bg-grey-shade-2 text-grey-shade-14 w-fit text-12 px-3 py-2 rounded-lg">{"Poker"}</li>
                    <li class="bg-grey-shade-13 text-grey-shade-1 w-fit text-12 px-3 py-2 rounded-lg">{"Baccarat"}</li>
                    <li class="bg-grey-shade-13 text-grey-shade-1 w-fit text-12 px-3 py-2 rounded-lg">{"Craps"}</li>
                    <li class="bg-grey-shade-13 text-grey-shade-1 w-fit text-12 px-3 py-2 rounded-lg">{"Pai Gow Poker"}</li>
                    <li class="bg-grey-shade-13 text-grey-shade-1 w-fit text-12 px-3 py-2 rounded-lg">{"Sci Bo"}</li>
                </ul>
                </div>
                <div class="flex space-x-4  items-center justify-start p-0 rounded-sm">
                    <button type="submit"  class="bg-primary flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
                        {"Add"}
                    </button>
                    <button class="bg-grey-shade-14 flex items-center rounded p-2 text-primary text-12 font-400" onclick={props.modal_handle.clone()}>
                        {"Cancel"}
                    </button>
                </div>
            </div>
        </div>
    }
}
