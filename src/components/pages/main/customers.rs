use crate::{
    components::organisms::{
        export_button::ExportButton,
        paginator::{PaginationDataProps, PaginationFucProps, Paginator},
    },
    render_svg,
    // routes::main_routes::MainRoute,
};
use std::ops::Deref;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;
// use yew_router::prelude::*;

#[function_component(Customers)]
pub fn customers() -> Html {
    // let history = use_navigator().unwrap();
    let search_text = use_state(|| String::default());
    let initial = use_state(|| true);
    let pagination = use_state(|| PaginationDataProps {
        per_page: 10,
        total_items: 0,
        total_pages: 0,
        current_page: 1,
    });

    // fn handle_row_click(_id: i32, history: Navigator) {
    //     history.push(&MainRoute::CustomerProfile {
    //         id: "12".to_owned(),
    //     })
    // }

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
        "ID",
        "Username",
        "Email ID",
        "Host IP",
        "Device",
        "Status",
        "Actions",
    ];

    #[derive(Clone, Properties, PartialEq)]
    pub struct CustomerProps {
        pub name: String,
        pub id: String,
        pub email_address: String,
        pub status: String,
        pub location: String,
        pub ip: String,
        pub device: String,
        pub version: String,
    }

    let customers = vec![
        CustomerProps {
            id: "WW20011".into(),
            name: "Dianne Russell".into(),
            email_address: "JaneCooper@gmail.com".into(),
            status: "Active".into(),
            location: "India, Tamilnadu, chennai".into(),
            ip: "196.69.80.124".into(),
            device: "Iphone X".into(),
            version: "Android 11, SM-G991U".into(),
        },
        CustomerProps {
            id: "WW20011".into(),
            name: "Dianne Russell".into(),
            email_address: "JaneCooper@gmail.com".into(),
            status: "Banned".into(),
            location: "India, Tamilnadu, chennai".into(),
            ip: "196.69.80.124".into(),
            device: "Iphone X".into(),
            version: "Android 11, SM-G991U".into(),
        },
    ];

    let export = {
        // let customers = customers.clone();
        Callback::from(move |_: MouseEvent| {
            // let mut csv_data =
            //     "Notification Message|Status|Offer available from|Offer available until|Created on"
            //         .to_string();

            // for customer in customers.iter() {
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

    html!(
        <>
            <div class="bg-grey-shade-13 py-4 px-8">
                <div class="container mx-auto md:w-auto space-y-4">
                    <div class="flex md:justify-between md:items-center flex-col gap-6 md:flex-row ">
                        <div class="flex flex-row justify-between md:justify-normal gap-2 items-center w-full md:w-auto">
                            <h1>{"Customers"}</h1>
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
                        </div>
                    </div>
                    <ContextProvider<PaginationDataProps> context={(*pagination).clone()}>
                        <Paginator update_pagination={update_pagination.clone()} />
                    </ContextProvider<PaginationDataProps>>
                </div>
            </div>
            <div class="relative px-8">
                <div class="absolute hidden xl:block -z-10 top-0 left-0 h-[45px] w-full bg-grey-shade-13"></div>
                <div class="container mx-auto z-1">
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
                                customers.clone().iter().map(|customer| {
                                    html!{
                                        <tr>
                                            <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{customer.clone().id}</td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">{customer.clone().name}</td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">{customer.clone().email_address}</td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">
                                                <p>
                                                    {customer.clone().ip}
                                                </p>
                                                <span class="text-12 text-grey-shade-5">{customer.clone().location}</span>
                                            </td>
                                            <td class="py-3 text-left text-14   font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">
                                                <p>
                                                    {customer.clone().device}
                                                </p>
                                                <span class="text-12 text-grey-shade-5">{customer.clone().version}</span>
                                            </td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-14 tracking-wider">
                                                <span class={format!("rounded-full py-1 px-2 w-[24px] {}", if customer.clone().status == "Active" {"bg-success"} else {"bg-warning"})}>
                                                    {customer.clone().status}
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
                                    }
                                }).collect::<Html>()
                            }
                        </tbody>
                    </table>
                    <div class="grid md:grid-cols-2 lg:grid-cols-3 xl:hidden gap-4 mt-4">
                        {
                            customers.clone().iter().map(|customer| {
                                html!{<div class="border p-4 flex flex-col gap-2.5 rounded-md">
                                    <div class="flex flex-row justify-between items-center">
                                        <div class="flex flex-col gap-2.5">
                                            <div class="text-14 font-medium text-grey-shade-1">{customer.clone().id}</div>
                                            <div class="flex flex-col gap-1">
                                                <div class="text-grey-shade-5 text-12">{customer.clone().name}</div>
                                                <div class="text-14">{customer.clone().email_address}</div>
                                            </div>
                                        </div>
                                        <div class="text-12 font-medium text-grey-shade-1 relative group cursor-pointer">
                                            <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                            <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                                <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">
                                                    { "Ban" }
                                                </li>
                                            </ul>
                                        </div>
                                    </div>
                                    <div class="flex flex-row justify-between">
                                        <div class="flex flex-col gap-1 p-1">
                                            <div class="text-grey-shade-5 text-12">{customer.clone().location}</div>
                                            <div class="text-14">{customer.clone().ip}</div>
                                        </div>
                                        <div class="flex flex-col gap-1 p-1">
                                            <div class="text-grey-shade-5 text-12">{customer.clone().version}</div>
                                            <div class="text-14">{customer.clone().device}</div>
                                        </div>
                                    </div>
                                    <div class="flex flex-row justify-between items-center">
                                        <div class={format!("rounded-full py-1 px-2 flex-row gap-1 text-white {} ", if customer.clone().status == "Active"{ " bg-success" } else { "bg-warning" })}>
                                            {customer.clone().status}
                                        </div>
                                    </div>
                                </div>}
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </div>
        </>
    )
}
