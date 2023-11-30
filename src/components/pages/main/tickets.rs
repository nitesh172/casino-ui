use gloo_console::log;
use crate::{
    components::organisms::paginator::{PaginationDataProps, PaginationFucProps, Paginator},
    apis::ticket::{ TicketResponse, fetch_tickets },
    render_svg,
};
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use std::ops::Deref;
use yew::{ platform::spawn_local, prelude::* };

#[function_component(Tickets)]
pub fn tickets() -> Html {
    let tickets = use_state(|| Vec::<TicketResponse>::default());
    let search_text = use_state(|| String::default());
    let initial = use_state(|| true);
    let pagination = use_state(|| PaginationDataProps {
        per_page: 10,
        total_items: 0,
        total_pages: 0,
        current_page: 1,
    });

    let t_head = vec!["ID", "Email ID", "Username", "Query", "Status", "Actions"];

    let cloned_tickets = tickets.clone();
    let cloned_pagination = pagination.clone();
    let fetch_handle_tickets = move |()| {
        let tickets = cloned_tickets.clone();
        let pagination = cloned_pagination.clone();
        spawn_local(async move {
            let response = fetch_tickets().await;

            match response {
                Ok(response) => {
                    tickets.set(response.result);
                    pagination.set(PaginationDataProps {
                        current_page: response.page,
                        per_page: response.per_page,
                        total_items: response.total,
                        total_pages: response.total_pages,
                    })
                }
                Err(e) => log!("Error: {}", e.to_string()),
            }
        });
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

    let onfetch = fetch_handle_tickets.clone();
    use_effect_with((), move |_| {
        onfetch(()); // Call the async function
        || {}
    });

    html! {
        <>
            <div class="bg-grey-shade-13 px-8 py-4">
                <div class="container mx-auto md:w-auto space-y-4 " >
                    <div class="flex md:justify-between md:items-center flex-col gap-6 md:flex-row">
                        <div class="flex flex-row justify-between md:justify-normal gap-2 items-center w-full md:w-auto">
                            <h1>{"Tickets"}</h1>
                            <div class="hidden lg:flex items-center rounded border justify-start border-grey-shade-11 bg-white px-2 w-40">
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
                            <button
                                class="bg-grey-shade-0 flex flex-1 md:flex-none items-center rounded justify-center lg:justify-normal p-2 text-grey-shade-14 text-12 font-400"
                            >
                                <span class="pr-2">
                                {html! { render_svg!("majesticons:file", color="#ffff",width="14px")}}
                                </span>
                                {"Export"}
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
                        <tbody>
                            {
                                tickets.clone().iter().map(|ticket| {
                                    html!{
                                        <tr>
                                            <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{ticket.clone().id}</td>
                                            // <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider pl-5">{ticket.clone().email_address}</td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider">{ticket.clone().user_id}</td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider pr-5">{ticket.clone().query}</td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-14 tracking-wider">
                                                <span class={format!("rounded-full py-1 px-2 w-[24px] {}", if ticket.clone().status == "Open" {"bg-warning"} else {"bg-success"})}>
                                                    {ticket.clone().status}
                                                </span>
                                            </td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider relative group cursor-pointer">
                                                <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                                <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                                    <li class={format!("px-4 py-2 text-grey-shade-0 {}", if ticket.clone().status == "Open" {"hover:text-grey-shade-2 hover:bg-grey-shade-12 cursor-pointer"} else {"cursor-not-allowed"})}>{ "Open" }</li>
                                                    <li class={format!("px-4 py-2 text-grey-shade-0 {}", if ticket.clone().status == "Close" {"hover:text-grey-shade-2 hover:bg-grey-shade-12 cursor-pointer"} else {"cursor-not-allowed"})}>{ "Close" }</li>
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
                            tickets.clone().iter().map(|ticket| {
                                html!{
                                    <div class="border p-4 flex flex-col gap-2.5 rounded-md">
                                        <div class="flex flex-col justify-between">
                                            <div class="flex flex-row justify-between items-center">
                                                <div class="flex flex-col gap-2.5">
                                                    <div class="text-14 font-medium text-grey-shade-1">{ticket.clone().id}</div>
                                                    <div class="flex flex-col gap-1">
                                                        <div class="text-grey-shade-5 text-12">{ticket.clone().user_id}</div>
                                                        // <div class="text-14">{ticket.clone().email_address}</div>
                                                    </div>
                                                </div>
                                                <div class="text-12 font-medium text-grey-shade-1 relative group cursor-pointer">
                                                    <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                                    <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                                        <li class={format!("px-4 py-2 text-grey-shade-0 {}", if ticket.clone().status == "Open" {"hover:text-grey-shade-2 hover:bg-grey-shade-12 cursor-pointer"} else {"cursor-not-allowed"})}>{ "Open" }</li>
                                                        <li class={format!("px-4 py-2 text-grey-shade-0 {}", if ticket.clone().status == "Close" {"hover:text-grey-shade-2 hover:bg-grey-shade-12 cursor-pointer"} else {"cursor-not-allowed"})}>{ "Close" }</li>
                                                    </ul>
                                                </div>
                                            </div>
                                            <div class="flex flex-col">
                                                <div class="text-12 text-grey-shade-5">{"Query"}</div>
                                                <div class="text-14 line-clamp-2">{ticket.clone().query}</div>
                                            </div>
                                        </div>
                                        <div class="flex flex-row justify-between items-center">
                                            <div class={format!("rounded-full py-1 px-2 flex-row gap-1 text-white {} ", if ticket.clone().status == "Close"{ " bg-success" } else { "bg-warning" })}>
                                                {ticket.clone().status}
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </div>
        </>
    }
}
