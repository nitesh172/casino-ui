use crate::{
    apis::ticket::{ fetch_tickets, update_ticket_status, TicketResponse, UpdateTicket },
    components::organisms::{
        export_button::{ download_csv_file, ExportButton },
        paginator::{ PaginationDataProps, PaginationFucProps, Paginator },
    },
    render_svg,
    stores::auth_store::AuthStore,
    ToastProps,
};
use gloo_console::log;
use std::ops::Deref;
use web_sys::{ wasm_bindgen::JsCast, HtmlElement, HtmlInputElement };
use yew::{ platform::spawn_local, prelude::* };
use yewdux::prelude::*;

#[function_component(Tickets)]
pub fn tickets() -> Html {
    let is_open = use_state(|| false);
    let tickets = use_state(|| Vec::<TicketResponse>::default());
    let ticket = use_state(|| UpdateTicket::default());
    let search_text = use_state(|| String::default());
    let initial = use_state(|| true);
    let pagination = use_state(|| PaginationDataProps {
        per_page: 10,
        total_items: 0,
        total_pages: 0,
        current_page: 1,
    });

    let t_head = vec!["ID", "Email ID", "Username", "Query", "Status", "Actions"];

    let (auth_store, _) = use_store::<AuthStore>();

    let token = auth_store.token.clone();

    let modal_handle = {
        let is_open = is_open.clone();
        let ticket = ticket.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
            ticket.set(UpdateTicket::default());
        })
    };

    let on_ok_response = {
        let is_open = is_open.clone();
        let ticket = ticket.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
            ticket.set(UpdateTicket::default());
        })
    };

    let cloned_tickets = tickets.clone();
    let cloned_pagination = pagination.clone();
    let cloned_search_text = search_text.clone();
    let token = token.clone();
    let fetch_handle_tickets = move |()| {
        let tickets = cloned_tickets.clone();
        let pagination = cloned_pagination.clone();
        let search_text = cloned_search_text.clone();
        let token = token.clone();
        spawn_local(async move {
            let response = fetch_tickets(
                token,
                pagination.deref().clone(),
                search_text.to_string()
            ).await;

            match response {
                Ok(response) => {
                    tickets.set(response.result);
                    pagination.set(PaginationDataProps {
                        current_page: response.page,
                        per_page: response.per_page,
                        total_items: response.total,
                        total_pages: response.total_pages,
                    });
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
        let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        cloned_initial.set(true);
        cloned_search_text.set(value);
    });

    let status_callback = {
        let ticket = ticket.clone();
        let is_open = is_open.clone();
        Callback::from(move |event: MouseEvent| {
            if let Some(target) = event.target() {
                if let Some(li) = target.dyn_ref::<HtmlElement>() {
                    if let Some(id) = li.get_attribute("data-id") {
                        if !id.is_empty() {
                            if let Some(status) = li.get_attribute("id") {
                                if !id.is_empty() {
                                    is_open.set(!*is_open);
                                    ticket.set(UpdateTicket {
                                        id: id.clone(),
                                        status: status.clone(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        })
    };

    let toasts_data = use_context::<ToastProps>().expect("no ctx found");

    let export = {
        let tickets = tickets.clone();
        let add_toast = toasts_data.add_toast.clone();
        Callback::from(move |_: MouseEvent| {
            let mut csv_data = "ID|Username|Email ID|Query|Status".to_string();
            let add_toast = add_toast.clone();
            for ticket in tickets.iter() {
                let data: String = format!(
                    "\n{}|{}|{}|{}|{}",
                    ticket.clone().id,
                    ticket.clone().user.name,
                    ticket.clone().user.email_address,
                    ticket.clone().query,
                    ticket.clone().status
                );
                csv_data = csv_data + data.as_str();
            }

            download_csv_file(csv_data.as_str(), add_toast)
        })
    };

    {
        let onfetch = fetch_handle_tickets.clone();
        let pagination = pagination.clone();
        let search_text = search_text.clone();
        use_effect_with((pagination.clone(), search_text.clone()), move |_| {
            if *initial {
                onfetch(());
            }

            || {}
        });
    }

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
                        <tbody>
                            {
                                tickets.clone().iter().map(|ticket| {
                                    html!{
                                        <tr>
                                            <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{ticket.clone().id}</td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider">{ticket.clone().user.email_address}</td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider">{ticket.clone().user.name}</td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider pr-5">{ticket.clone().query}</td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-14 tracking-wider">
                                                <span class={format!("rounded-full py-1 px-2 w-[24px] {}", if ticket.clone().status == "Open" {"bg-warning"} else {"bg-success"})}>
                                                    {ticket.clone().status}
                                                </span>
                                            </td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider relative group cursor-pointer">
                                                <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                                <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                                    <li id="Open" data-id={ticket.clone().id} onclick={status_callback.clone()} class={format!("px-4 py-2 text-grey-shade-0 {}", if ticket.clone().status == "Closed" {"hover:text-grey-shade-2 hover:bg-grey-shade-12 cursor-pointer"} else {"cursor-not-allowed"})}>{ "Open" }</li>
                                                    <li id="Closed" data-id={ticket.clone().id} onclick={status_callback.clone()} class={format!("px-4 py-2 text-grey-shade-0 {}", if ticket.clone().status == "Open" {"hover:text-grey-shade-2 hover:bg-grey-shade-12 cursor-pointer"} else {"cursor-not-allowed"})}>{ "Closed" }</li>
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
                                                        <div class="text-grey-shade-5 text-12">{ticket.clone().user.name}</div>
                                                        <div class="text-14">{ticket.clone().user.email_address}</div>
                                                    </div>
                                                </div>
                                                <div class="text-12 font-medium text-grey-shade-1 relative group cursor-pointer">
                                                    <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                                    <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                                        <li class={format!("px-4 py-2 text-grey-shade-0 {}", if ticket.clone().status == "Open" {"hover:text-grey-shade-2 hover:bg-grey-shade-12 cursor-pointer"} else {"cursor-not-allowed"})}>{ "Open" }</li>
                                                        <li class={format!("px-4 py-2 text-grey-shade-0 {}", if ticket.clone().status == "Closed" {"hover:text-grey-shade-2 hover:bg-grey-shade-12 cursor-pointer"} else {"cursor-not-allowed"})}>{ "Closed" }</li>
                                                    </ul>
                                                </div>
                                            </div>
                                            <div class="flex flex-col">
                                                <div class="text-12 text-grey-shade-5">{"Query"}</div>
                                                <div class="text-14 line-clamp-2">{ticket.clone().query}</div>
                                            </div>
                                        </div>
                                        <div class="flex flex-row justify-between items-center">
                                            <div class={format!("rounded-full py-1 px-2 flex-row gap-1 text-white {} ", if ticket.clone().status == "Closed"{ " bg-success" } else { "bg-warning" })}>
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
            { if *is_open {
                html! {
                        <StatusConfirmModal
                            modal_handle={modal_handle.clone()}
                            on_ok_response={on_ok_response.clone()}
                            fetch_handle_tickets={fetch_handle_tickets.clone()}
                            ticket={ticket.clone()}
                        />
                    }
                } else {
                    html! {}
                }
            }
        </>
    }
}

#[derive(Properties, PartialEq)]
struct StatusModalProps {
    #[prop_or_default]
    pub on_ok_response: Callback<()>,
    pub fetch_handle_tickets: Callback<()>,
    pub ticket: UseStateHandle<UpdateTicket>,
    modal_handle: Callback<MouseEvent>,
}

#[function_component(StatusConfirmModal)]
fn status_confirm_modal(props: &StatusModalProps) -> Html {
    log!(props.ticket.id.clone(), props.ticket.status.clone());

    let (auth_store, _) = use_store::<AuthStore>();

    let token = auth_store.token.clone();

    let toasts_data = use_context::<ToastProps>().expect("no ctx found");

    let update_status_handler = {
        let on_ok = props.on_ok_response.clone();
        let on_handle_tickets = props.fetch_handle_tickets.clone();
        let ticket = props.ticket.clone();
        let token = token.clone();
        let add_toast = toasts_data.add_toast.clone();
        Callback::from(move |_event: MouseEvent| {
            let on_ok = on_ok.clone();
            let on_handle_tickets = on_handle_tickets.clone();
            let ticket = ticket.clone();
            let token = token.clone();
            let add_toast = add_toast.clone();
            spawn_local(async move {
                if !ticket.id.is_empty() && !ticket.status.is_empty() {
                    let response = update_ticket_status(token, UpdateTicket {
                        id: ticket.id.clone(),
                        status: ticket.status.clone(),
                    }).await;

                    match response {
                        Ok(_response) => {
                            on_ok.emit(());
                            on_handle_tickets.emit(());
                            add_toast.emit(format!("Ticket status {}.", ticket.status.to_lowercase()));
                        }
                        Err(e) => log!("Error: ", e.to_string()),
                    }
                }
            });
        })
    };

    html! {
        <div class="z-10 fixed inset-0 bg-grey-shade-0/70 w-screen flex h-screen items-center justify-center p-5">
            <div class=" bg-white p-7 rounded-sm space-y-7">
                <div class="flex items-center justify-between">
                    <p>{"Confirmation required"}</p>
                    <span class="cursor-pointer" onclick={props.modal_handle.clone()}>
                        {html! {render_svg!("mdi:multiply",color="#232323",width="25px")}}
                    </span>
                </div>
                <div>{"Are you sure you want to proceed with this action?"}</div>
                <div class="flex space-x-4  items-center justify-start p-0 rounded-sm">
                    <button type="button" onclick={update_status_handler} class="bg-primary flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
                        {"Confirm"}
                    </button>
                    <button class="bg-grey-shade-14 flex items-center rounded p-2 text-primary text-12 font-400" onclick={props.modal_handle.clone()}>
                        {"Close"}
                    </button>
                </div>
            </div>
        </div>
    }
}
