use crate::{
    apis::notification::{
        create_notification, delete_notification, fetch_notification, fetch_notifications,
        update_notification, Notification, NotificationResponse,
    },
    components::organisms::{
        export_button::{ExportButton, download_csv_file},
        paginator::{PaginationDataProps, PaginationFucProps, Paginator},
    },
    render_svg,
    utils::format_dates::format_date_to_readable,
};
use gloo_console::log;
use std::ops::Deref;
use web_sys::{wasm_bindgen::JsCast, HtmlElement, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*};

#[function_component(Notifications)]
pub fn notifications() -> Html {
    let is_open = use_state(|| false);
    let is_delete_open = use_state(|| false);
    let search_text = use_state(|| String::default());
    let initial = use_state(|| true);
    let notifications = use_state(|| Vec::<NotificationResponse>::default());
    let pagination = use_state(|| PaginationDataProps {
        per_page: 10,
        total_items: 0,
        total_pages: 0,
        current_page: 1,
    });
    let notification_id = use_state(|| String::default());

    let modal_handle = {
        let is_open = is_open.clone();
        let notification_id = notification_id.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
            notification_id.set("".to_string());
        })
    };

    let delete_modal_handle = {
        let is_delete_open = is_delete_open.clone();
        let notification_id = notification_id.clone();
        Callback::from(move |_| {
            is_delete_open.set(!*is_delete_open);
            notification_id.set("".to_string());
        })
    };

    let on_ok_response = {
        let is_open = is_open.clone();
        let notification_id = notification_id.clone();
        Callback::from(move |_| {
            is_open.set(false);
            notification_id.set("".to_string());
        })
    };

    let on_ok_delete_handle = {
        let is_delete_open = is_delete_open.clone();
        let notification_id = notification_id.clone();
        Callback::from(move |_| {
            is_delete_open.set(!*is_delete_open);
            notification_id.set("".to_string());
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

    let cloned_notifications = notifications.clone();
    let cloned_pagination = pagination.clone();
    let cloned_initial = initial.clone();
    let cloned_search_text = search_text.clone();
    let fetch_handle_notifications = move |()| {
        let notifications = cloned_notifications.clone();
        let pagination = cloned_pagination.clone();
        let cloned_initial = cloned_initial.clone();
        let search_text = cloned_search_text.clone();
        spawn_local(async move {
            let response = fetch_notifications(pagination.per_page, search_text.to_string()).await;

            match response {
                Ok(response) => {
                    notifications.set(response.result);
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

    let edit_callback = {
        let is_open = is_open.clone();
        let notification_id = notification_id.clone();
        Callback::from(move |event: MouseEvent| {
            if let Some(target) = event.target() {
                if let Some(li) = target.dyn_ref::<HtmlElement>() {
                    if let Some(id) = li.get_attribute("data-id") {
                        if !id.is_empty() {
                            is_open.set(!*is_open);
                            notification_id.set(id.clone());
                        }
                    }
                }
            }
        })
    };

    let delete_callback = {
        let is_delete_open = is_delete_open.clone();
        let notification_id = notification_id.clone();
        Callback::from(move |event: MouseEvent| {
            if let Some(target) = event.target() {
                if let Some(li) = target.dyn_ref::<HtmlElement>() {
                    if let Some(id) = li.get_attribute("data-id") {
                        if !id.is_empty() {
                            is_delete_open.set(!*is_delete_open);
                            notification_id.set(id.clone());
                        }
                    }
                }
            }
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

    let export = {
        let notifications = notifications.clone();
        Callback::from(move |_: MouseEvent| {
            let mut csv_data =
                "Notification Message|Status|Offer available from|Offer available until|Created on"
                    .to_string();

            for notification in notifications.iter() {
                let data: String = format!(
                    "\n{}|{}|{}|{}|{}",
                    notification.clone().description.as_str(),
                    notification.clone().status,
                    format_date_to_readable(notification.clone().starts_at, "%Y-%m-%d"),
                    format_date_to_readable(notification.clone().ends_at, "%Y-%m-%d"),
                    format_date_to_readable(notification.clone().created_at, "%Y-%m-%d")
                );
                csv_data = csv_data + data.as_str();
            }

            download_csv_file(csv_data.as_str())
        })
    };

    {
        let onfetch = fetch_handle_notifications.clone();
        let pagination = pagination.clone();
        let search_text = search_text.clone();
        use_effect_with((pagination.clone(),search_text.clone()), move |_| {
            if *initial {
                onfetch(());
            }

            || {}
        });
    }

    html! {
        <>
        <div class="bg-grey-shade-13 py-4 px-8">
            <div class="container mx-auto md:w-auto space-y-4 " >
                <div class="flex md:justify-between md:items-center flex-col gap-6 md:flex-row">
                    <div class="flex flex-row justify-between md:justify-normal gap-2 items-center w-full md:w-auto">
                        <h1>{"Notifications"}</h1>
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
                            class="bg-primary flex flex-1 md:flex-none items-center rounded justify-center md:justify-normal p-2 text-grey-shade-14 text-12 font-400"
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
        <div class="container mx-auto z-1">
            <table class="w-full table-auto hidden xl:inline-table">
                <thead class="bg-grey-shade-13">
                    <tr>
                        <th class="py-3 max-w-[100px] text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Notification Message"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Status"}</th>
                        <th class="py-3 pl-5 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Offer available from"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Offer available until"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 pr-5 tracking-wider">{"Created on"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Action"}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        notifications.clone().iter().map(|notification| {
                            html!{
                                <tr>
                                    <td class="py-3 max-w-lg text-left text-14 font-medium text-grey-shade-1 tracking-wider">{notification.clone().description}</td>
                                    <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider">
                                        <span class="bg-grey-shade-11 rounded-full py-1 px-2 capitalize">
                                            {notification.clone().status}
                                        </span>
                                    </td>
                                    <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider pl-5">{format_date_to_readable(notification.clone().starts_at, "%d %h %Y | %H:%M")}</td>
                                    <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider">{format_date_to_readable(notification.clone().ends_at, "%d %h %Y | %H:%M")}</td>
                                    <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider pr-5">{format_date_to_readable(notification.clone().created_at, "%d %h %Y")}</td>
                                    <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider relative group cursor-pointer">
                                        <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                        <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                            <li onclick={edit_callback.clone()} data-id={notification.clone().id} class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">{"Edit"}</li>
                                            <li onclick={delete_callback.clone()} data-id={notification.clone().id} class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">{"Delete"}</li>
                                        </ul>
                                    </td>
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                </tbody>
            </table>
            <div class="grid md:grid-cols-2 xl:hidden gap-4 mt-4">
                {
                    notifications.clone().iter().map(|notification| {
                        html!{
                            <div class="rounded-xl border p-4 flex flex-col justify-between gap-3">
                                <div class="flex flex-col gap-3">
                                    <div class="flex flex-row justify-between">
                                        <div class="flex flex-row gap-3 justify-between">
                                            <div class="flex flex-col">
                                                <div class="text-12 text-grey-shade-5">{"Offer available from"}</div>
                                                <div class="text-14 font-400">{format_date_to_readable(notification.clone().starts_at, "%d %h %Y | %H:%M")}</div>
                                            </div>
                                            <div class="flex flex-col">
                                                <div class="text-12 text-grey-shade-5">{"Offer available until"}</div>
                                                <div class="text-14 font-400">{format_date_to_readable(notification.clone().ends_at, "%d %h %Y | %H:%M")}</div>
                                            </div>
                                        </div>
                                        <div class="text-12 font-medium text-grey-shade-1 relative group cursor-pointer">
                                            <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                            <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                                <li onclick={edit_callback.clone()} data-id={notification.clone().id} class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">{"Edit"}</li>
                                                <li onclick={delete_callback.clone()} data-id={notification.clone().id} class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">{"Delete"}</li>
                                            </ul>
                                        </div>
                                    </div>
                                    <div class="flex flex-col">
                                        <div class="text-12 text-grey-shade-5">{"Notification message"}</div>
                                        <div class="text-14 line-clamp-2">{notification.clone().description}</div>
                                    </div>
                                </div>
                                <div class="flex flex-row justify-between items-center">
                                    <div class="rounded-full py-1 px-2 flex-row gap-1 bg-grey-shade-11">
                                        {notification.clone().status}
                                    </div>
                                    <div class="text-14 font-medium text-grey-shade-1 flex flex-row gap-0.5">
                                        {format_date_to_readable(notification.clone().created_at, "%d %h %Y | %H:%M")}
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
                    <Modal
                        modal_handle={modal_handle.clone()}
                        on_ok_response={on_ok_response.clone()}
                        fetch_handle_notifications={fetch_handle_notifications.clone()}
                        notification_id={notification_id.to_string()}
                    />
                }
            } else {
                html! {}
            }
        }
        { if *is_delete_open {
            html! {
                    <DeleteModal
                        delete_modal_handle={delete_modal_handle.clone()}
                        on_ok_response={on_ok_delete_handle.clone()}
                        fetch_handle_notifications={fetch_handle_notifications.clone()}
                        notification_id={notification_id.to_string()}
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
struct ModalProps {
    #[prop_or_default]
    pub on_ok_response: Callback<()>,
    pub fetch_handle_notifications: Callback<()>,
    pub notification_id: String,
    modal_handle: Callback<MouseEvent>,
}

#[function_component(Modal)]
fn edit_modal(props: &ModalProps) -> Html {
    let state = use_state(Notification::default);

    let cloned_state = state.clone();
    let state_changed = Callback::from(move |event: Event| {
        let mut data = cloned_state.deref().clone();
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        let name = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .name();
        let checked = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .checked();

        match name.as_str() {
            "description" => {
                data.description = value;
            }
            "ends_at" => {
                data.ends_at = value;
            }
            "has_expiry_date" => {
                data.has_expiry_date = checked;
            }
            "is_email_enabled" => {
                data.is_email_enabled = checked;
            }
            "is_push_enabled" => {
                data.is_push_enabled = checked;
            }
            "is_sms_enabled" => {
                data.is_sms_enabled = checked;
            }
            "starts_at" => {
                data.starts_at = value;
            }
            _ => (),
        }
        cloned_state.set(data);
    });

    let save_notification_handler = {
        let st = (*state).clone();
        let on_ok = props.on_ok_response.clone();
        let on_handle_notifications = props.fetch_handle_notifications.clone();
        let notification_id = props.notification_id.clone();
        Callback::from(move |_event: MouseEvent| {
            let notification: Notification = st.clone();
            let on_ok = on_ok.clone();
            let on_handle_notifications = on_handle_notifications.clone();
            let notification_id = notification_id.clone();
            spawn_local(async move {
                if !notification_id.is_empty() {
                    let response = update_notification(notification, notification_id).await;

                    match response {
                        Ok(_response) => {
                            on_ok.emit(());
                            on_handle_notifications.emit(());
                        }
                        Err(e) => log!("Error: ", e.to_string()),
                    }
                } else {
                    let response = create_notification(notification).await;

                    match response {
                        Ok(_response) => {
                            on_ok.emit(());
                            on_handle_notifications.emit(());
                        }
                        Err(e) => log!("Error: ", e.to_string()),
                    }
                }
            });
        })
    };

    let cloned_notification = state.clone();
    let fetch_handle_notification = move |id: String| {
        let notification = cloned_notification.clone();
        spawn_local(async move {
            let response = fetch_notification(id).await;

            match response {
                Ok(response) => {
                    notification.set(Notification {
                        description: response.description,
                        ends_at: format_date_to_readable(response.ends_at, "%Y-%m-%dT%H:%M"),
                        has_expiry_date: response.has_expiry_date,
                        is_email_enabled: response.is_email_enabled,
                        is_push_enabled: response.is_push_enabled,
                        is_sms_enabled: response.is_sms_enabled,
                        starts_at: format_date_to_readable(response.starts_at, "%Y-%m-%dT%H:%M"),
                    });
                }
                Err(e) => log!("Error: {}", e.to_string()),
            }
        });
    };

    let onfetch = fetch_handle_notification.clone();
    let notification_id = props.notification_id.clone();
    use_effect_with((), move |_| {
        if !notification_id.is_empty() {
            onfetch(notification_id); // Call the async function
        }
        || {}
    });

    html! {
        <div class="z-10 fixed inset-0 bg-grey-shade-0/70 w-screen flex h-screen items-center justify-center p-5">
            <div class=" bg-white p-7 rounded-sm space-y-7 w-full md:w-auto">
                <div class="flex items-center justify-between">
                    <p>{"Create notification"}</p>
                    <span class="cursor-pointer" onclick={props.modal_handle.clone()}>
                        {html! {render_svg!("mdi:multiply",color="#232323",width="25px")}}
                    </span>
                </div>

                <div class="flex flex-col space-y-1.5 md:w-[600px]">
                    <label
                        for="message"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"Notification message here"}
                    </label>
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start p-2" >
                        <textarea
                            id="message"
                            name="description"
                            placeholder="Message"
                            class="px-3.5 py-3placeholder:text-grey-shade-6 text-14 leading-20
                            w-full  resize-none
                            bg-white
                            border-grey-shade-11
                            font-300 font-sans outline-none"
                            value={state.description.clone()}
                            onchange={state_changed.clone()}
                        />
                    </div>
                </div>
                <div class="flex flex-col space-y-4">
                    <label class="flex items-center space-x-2">
                        <input onchange={state_changed.clone()} checked={state.has_expiry_date.clone()} name="has_expiry_date" type="checkbox" class="border-2 border-primary focus:ring-grey-shade-5 w-[20px] h-[20px] toggle-checkbox:checked " />
                        <span class="text-gray-700">{"Offer expiry date"}</span>
                    </label>
                    <div class="flex flex-col gap-3 md:flex-row justify-between md:items-center">
                        <div>
                            <label for="datetime" class="block text-sm font-medium text-gray-700">{"Offer available from"}</label>
                            <input value={state.starts_at.clone()} onchange={state_changed.clone()} name="starts_at" type="datetime-local" id="datetime" placeholder="Select a date and time" class="mt-1 p-2 border rounded-md w-full focus:outline-none focus:border-blue-500 focus:ring focus:ring-blue-200 my-custom-tailwind-class" />

                        </div>
                        <div>
                            <label for="datetime" class="block text-sm font-medium text-gray-700">{"Offer available until"}</label>
                            <input value={state.ends_at.clone()} onchange={state_changed.clone()} name="ends_at" type="datetime-local" id="datetime" class="mt-1 p-2 border rounded-md w-full" />
                        </div>
                    </div>
                </div>
                <div class="space-y-4">
                    <p>{"Notify this through"}</p>
                    <div class="flex flex-col space-y-4">
                        <label class="flex items-center space-x-2">
                            <input type="checkbox" onchange={state_changed.clone()} name="is_email_enabled" checked={state.is_email_enabled.clone()} />
                            <span class="text-gray-700">{"Email"}</span>
                        </label>
                        <label class="flex items-center space-x-2 toggle-label ">
                            <input type="checkbox" onchange={state_changed.clone()} name="is_push_enabled" checked={state.is_push_enabled.clone()} />
                            <span class="text-gray-700">{"Push notification"}</span>
                        </label>
                        <label class="flex items-center space-x-2 toggle-label ">
                            <input type="checkbox" onchange={state_changed.clone()} name="is_sms_enabled" checked={state.is_sms_enabled.clone()} />
                            <span class="text-gray-700">{"SMS"}</span>
                        </label>
                    </div>
                </div>
                <div class="flex space-x-4  items-center justify-start p-0 rounded-sm">
                    <button type="button" onclick={save_notification_handler} class="bg-primary flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
                        {"Save"}
                    </button>
                    <button class="bg-grey-shade-14 flex items-center rounded p-2 text-primary text-12 font-400" onclick={props.modal_handle.clone()}>
                        {"Close"}
                    </button>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct DeleteModalProps {
    #[prop_or_default]
    pub on_ok_response: Callback<()>,
    pub fetch_handle_notifications: Callback<()>,
    pub notification_id: String,
    delete_modal_handle: Callback<MouseEvent>,
}

#[function_component(DeleteModal)]
fn delete_modal(props: &DeleteModalProps) -> Html {
    let delete_notification_handler = {
        let on_ok = props.on_ok_response.clone();
        let on_handle_notifications = props.fetch_handle_notifications.clone();
        let notification_id = props.notification_id.clone();
        Callback::from(move |_event: MouseEvent| {
            let on_ok = on_ok.clone();
            let on_handle_notifications = on_handle_notifications.clone();
            let notification_id = notification_id.clone();
            spawn_local(async move {
                if !notification_id.is_empty() {
                    let response = delete_notification(notification_id).await;

                    match response {
                        Ok(_response) => {
                            on_ok.emit(());
                            on_handle_notifications.emit(());
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
                    <span class="cursor-pointer" onclick={props.delete_modal_handle.clone()}>
                        {html! {render_svg!("mdi:multiply",color="#232323",width="25px")}}
                    </span>
                </div>
                <div>{"Are you sure you want to proceed with this action?"}</div>
                <div class="flex space-x-4  items-center justify-start p-0 rounded-sm">
                    <button type="button" onclick={delete_notification_handler} class="bg-primary flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
                        {"Confirm"}
                    </button>
                    <button class="bg-grey-shade-14 flex items-center rounded p-2 text-primary text-12 font-400" onclick={props.delete_modal_handle.clone()}>
                        {"Close"}
                    </button>
                </div>
            </div>
        </div>
    }
}
