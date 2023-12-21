use crate::{
    apis::customer::{ fetch_users, UserResponse, update_user, UpdateCustomer },
    components::organisms::{
        export_button::{ download_csv_file, ExportButton },
        paginator::{ PaginationDataProps, PaginationFucProps, Paginator },
    },
    render_svg,
    routes::main_routes::MainRoute,
    stores::auth_store::AuthStore,
    ToastProps,
};
use gloo_console::log;
use std::ops::Deref;
use web_sys::{ wasm_bindgen::JsCast, HtmlInputElement, HtmlElement };
use yew::{ platform::spawn_local, prelude::* };
use yewdux::prelude::*;
use yew_router::prelude::*;

#[function_component(Customers)]
pub fn customers() -> Html {
    let history = use_navigator().unwrap();
    let search_text = use_state(|| String::default());
    let initial = use_state(|| true);
    let is_open = use_state(|| false);
    let customers = use_state(|| Vec::<UserResponse>::default());
    let customer = use_state(|| UpdateCustomer::default());
    let pagination = use_state(|| PaginationDataProps {
        per_page: 10,
        total_items: 0,
        total_pages: 0,
        current_page: 1,
    });

    let (auth_store, _) = use_store::<AuthStore>();

    let token = auth_store.token.clone();

    // fn handle_row_click(_id: i32, history: Navigator) {
    //     history.push(&MainRoute::CustomerProfile {
    //         id: "12".to_owned(),
    //     })
    // }

    let handle_row_click = {
        let history = history.clone();
        Callback::from(move |event: MouseEvent| {
            log!("clicked");
            if let Some(target) = event.target() {
                if let Some(td) = target.dyn_ref::<HtmlElement>() {
                    if let Some(id) = td.get_attribute("data-id") {
                        if !id.is_empty() {
                            log!("clicked", id.clone());
                            history.push(
                                &(MainRoute::CustomerProfile {
                                    id: id.clone(),
                                })
                            )
                        }
                    }
                }
            }
        })
    };

    let modal_handle = {
        let is_open = is_open.clone();
        let customer = customer.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
            customer.set(UpdateCustomer::default());
        })
    };

    let on_ok_response = {
        let is_open = is_open.clone();
        let customer = customer.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
            customer.set(UpdateCustomer::default());
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
        let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        cloned_initial.set(true);
        cloned_search_text.set(value);
    });

    let t_head = vec!["ID", "Username", "Email ID", "Host IP", "Device", "Status", "Actions"];

    let cloned_customers = customers.clone();
    let cloned_pagination = pagination.clone();
    let cloned_initial = initial.clone();
    let cloned_search_text = search_text.clone();
    let token = token.clone();
    let fetch_handle_customers = move |()| {
        let customers = cloned_customers.clone();
        let pagination = cloned_pagination.clone();
        let cloned_initial = cloned_initial.clone();
        let search_text = cloned_search_text.clone();
        let token = token.clone();
        spawn_local(async move {
            let response = fetch_users(
                token,
                pagination.deref().clone(),
                search_text.to_string()
            ).await;

            match response {
                Ok(response) => {
                    customers.set(response.result);
                    cloned_initial.set(false);
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

    {
        let onfetch = fetch_handle_customers.clone();
        let pagination = pagination.clone();
        let search_text = search_text.clone();
        use_effect_with((pagination.clone(), search_text.clone()), move |_| {
            if *initial {
                onfetch(());
            }

            || {}
        });
    }

    let toasts_data = use_context::<ToastProps>().expect("no ctx found");

    let export = {
        let customers = customers.clone();
        let add_toast = toasts_data.add_toast.clone();
        Callback::from(move |_: MouseEvent| {
            let mut csv_data = "ID|Username|Email ID|Status".to_string();
            let add_toast = add_toast.clone();

            for customer in customers.iter() {
                let data: String = format!(
                    "\n{}|{}|{}|{}",
                    customer.clone().id,
                    customer.clone().name,
                    customer.clone().email_address,
                    customer.clone().status
                );
                csv_data = csv_data + data.as_str();
            }

            download_csv_file(csv_data.as_str(), add_toast)
        })
    };

    // #[derive(Clone, Properties, PartialEq)]
    // pub struct CustomerProps {
    //     pub name: String,
    //     pub id: String,
    //     pub email_address: String,
    //     pub status: String,
    //     pub location: String,
    //     pub ip: String,
    //     pub device: String,
    //     pub version: String,
    // }

    // let customers = vec![
    //     CustomerProps {
    //         id: "WW20011".into(),
    //         name: "Dianne Russell".into(),
    //         email_address: "JaneCooper@gmail.com".into(),
    //         status: "Active".into(),
    //         location: "India, Tamilnadu, chennai".into(),
    //         ip: "196.69.80.124".into(),
    //         device: "Iphone X".into(),
    //         version: "Android 11, SM-G991U".into(),
    //     },
    //     CustomerProps {
    //         id: "WW20011".into(),
    //         name: "Dianne Russell".into(),
    //         email_address: "JaneCooper@gmail.com".into(),
    //         status: "Banned".into(),
    //         location: "India, Tamilnadu, chennai".into(),
    //         ip: "196.69.80.124".into(),
    //         device: "Iphone X".into(),
    //         version: "Android 11, SM-G991U".into(),
    //     },
    // ];

    let status_callback = {
        let customer = customer.clone();
        let is_open = is_open.clone();
        Callback::from(move |event: MouseEvent| {
            if let Some(target) = event.target() {
                if let Some(li) = target.dyn_ref::<HtmlElement>() {
                    if let Some(id) = li.get_attribute("data-id") {
                        if !id.is_empty() {
                            if let Some(status) = li.get_attribute("id") {
                                if !id.is_empty() {
                                    is_open.set(!*is_open);
                                    customer.set(UpdateCustomer {
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
                                            <td data-id={customer.clone().id} onclick={handle_row_click.clone()} class="cursor-pointer py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider pr-5">{customer.clone().id}</td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">{customer.clone().name}</td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">{customer.clone().email_address}</td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">
                                                <p>
                                                    {customer.clone().ip_address}
                                                </p>
                                                // <span class="text-12 text-grey-shade-5">{customer.clone().location}</span>
                                            </td>
                                            <td class="py-3 text-left text-14   font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">
                                                <p>
                                                    {customer.clone().device.device}
                                                </p>
                                                <span class="text-12 text-grey-shade-5">{customer.clone().device.os}</span>
                                            </td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-14 tracking-wider">
                                                <span class={format!("rounded-full py-1 px-2 w-[24px] {}", if customer.clone().status == "Active" {"bg-success"} else {"bg-warning"})}>
                                                    {if customer.clone().status == "Active" {"Active"} else {"Banned"}}
                                                </span>
                                            </td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider relative group cursor-pointer">
                                                <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                                <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                                    <li id="InActive" data-id={customer.clone().id} onclick={status_callback.clone()} class={format!("px-4 py-2 text-grey-shade-0 {}", if customer.clone().status == "Active" {"hover:text-grey-shade-2 hover:bg-grey-shade-12 cursor-pointer"} else {"cursor-not-allowed"})}>{ "Ban" }</li>
                                                    <li id="Active" data-id={customer.clone().id} onclick={status_callback.clone()} class={format!("px-4 py-2 text-grey-shade-0 {}", if customer.clone().status == "Inactive" {"hover:text-grey-shade-2 hover:bg-grey-shade-12 cursor-pointer"} else {"cursor-not-allowed"})}>{ "Unban" }</li>
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
                                                <li id="InActive" data-id={customer.clone().id} onclick={status_callback.clone()} class={format!("px-4 py-2 text-grey-shade-0 {}", if customer.clone().status == "Active" {"hover:text-grey-shade-2 hover:bg-grey-shade-12 cursor-pointer"} else {"cursor-not-allowed"})}>{ "Ban" }</li>
                                                <li id="Active" data-id={customer.clone().id} onclick={status_callback.clone()} class={format!("px-4 py-2 text-grey-shade-0 {}", if customer.clone().status == "InActive" {"hover:text-grey-shade-2 hover:bg-grey-shade-12 cursor-pointer"} else {"cursor-not-allowed"})}>{ "Unban" }</li>
                                            </ul>
                                        </div>
                                    </div>
                                    <div class="flex flex-row justify-between">
                                        <div class="flex flex-col gap-1 p-1">
                                            // <div class="text-grey-shade-5 text-12">{customer.clone().location}</div>
                                            <div class="text-14">{customer.clone().ip_address}</div>
                                        </div>
                                        <div class="flex flex-col gap-1 p-1">
                                            <div class="text-grey-shade-5 text-12">{customer.clone().device.os}</div>
                                            <div class="text-14">{customer.clone().device.device}</div>
                                        </div>
                                    </div>
                                    <div class="flex flex-row justify-between items-center">
                                        <div class={format!("rounded-full py-1 px-2 flex-row gap-1 text-white {} ", if customer.clone().status == "Active"{ " bg-success" } else { "bg-warning" })}>
                                            {if customer.clone().status == "Active" {"Active"} else {"Banned"}}
                                        </div>
                                    </div>
                                </div>}
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
                            fetch_handle_customers={fetch_handle_customers.clone()}
                            customer={customer.clone()}
                        />
                    }
                } else {
                    html! {}
                }
            }
        </>
    )
}

#[derive(Properties, PartialEq)]
struct StatusModalProps {
    #[prop_or_default]
    pub on_ok_response: Callback<()>,
    pub fetch_handle_customers: Callback<()>,
    pub customer: UseStateHandle<UpdateCustomer>,
    modal_handle: Callback<MouseEvent>,
}

#[function_component(StatusConfirmModal)]
fn status_confirm_modal(props: &StatusModalProps) -> Html {
    let (auth_store, _) = use_store::<AuthStore>();

    let token = auth_store.token.clone();

    let toasts_data = use_context::<ToastProps>().expect("no ctx found");

    let update_status_handler = {
        let on_ok = props.on_ok_response.clone();
        let on_handle_customers = props.fetch_handle_customers.clone();
        let customer = props.customer.clone();
        let token = token.clone();
        let add_toast = toasts_data.add_toast.clone();
        Callback::from(move |_event: MouseEvent| {
            let on_ok = on_ok.clone();
            let on_handle_customers = on_handle_customers.clone();
            let customer = customer.clone();
            let token = token.clone();
            let add_toast = add_toast.clone();
            spawn_local(async move {
                if !customer.id.is_empty() && !customer.status.is_empty() {
                    let response = update_user(token, UpdateCustomer {
                        id: customer.id.clone(),
                        status: customer.status.clone(),
                    }).await;

                    match response {
                        Ok(_response) => {
                            on_ok.emit(());
                            on_handle_customers.emit(());
                            add_toast.emit(
                                format!("Customer status {}.", customer.status.to_lowercase())
                            );
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
