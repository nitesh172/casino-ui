use crate::{
    apis::integration::{
        create_integration, delete_integration, fetch_integration, fetch_integrations,
        update_integration, Integration, IntegrationResponse,
    },
    components::{
        atoms::{
            label::{Label, LabelStyle},
            text_input::TextInput,
        },
        organisms::{
            export_button::{download_csv_file, ExportButton},
            paginator::{PaginationDataProps, PaginationFucProps, Paginator},
        },
    },
    render_svg,
    stores::auth_store::AuthStore,
};
use gloo_console::log;
use std::ops::Deref;
use web_sys::{wasm_bindgen::JsCast, HtmlElement, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*};
use yewdux::prelude::*;

#[function_component(Integrations)]
pub fn integrations() -> Html {
    let is_open = use_state(|| false);
    let is_delete_open = use_state(|| false);
    let search_text = use_state(|| String::default());
    let initial = use_state(|| true);
    let integrations = use_state(|| Vec::<IntegrationResponse>::default());
    let pagination = use_state(|| PaginationDataProps {
        per_page: 10,
        total_items: 0,
        total_pages: 0,
        current_page: 1,
    });
    let integration_id = use_state(|| String::default());

    let (auth_store, _) = use_store::<AuthStore>();

    let token = auth_store.token.clone();

    let modal_handle = {
        let integration_id = integration_id.clone();
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
            integration_id.set("".to_string());
        })
    };

    let delete_modal_handle = {
        let is_delete_open = is_delete_open.clone();
        let integration_id = integration_id.clone();
        Callback::from(move |_| {
            is_delete_open.set(!*is_delete_open);
            integration_id.set("".to_string());
        })
    };

    let on_ok_response = {
        let is_open = is_open.clone();
        let integration_id = integration_id.clone();
        Callback::from(move |_| {
            is_open.set(false);
            integration_id.set("".to_string());
        })
    };

    let on_ok_delete_handle = {
        let is_delete_open = is_delete_open.clone();
        let integration_id = integration_id.clone();
        Callback::from(move |_| {
            is_delete_open.set(!*is_delete_open);
            integration_id.set("".to_string());
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

    let edit_callback = {
        let is_open = is_open.clone();
        let integration_id = integration_id.clone();
        Callback::from(move |event: MouseEvent| {
            if let Some(target) = event.target() {
                if let Some(li) = target.dyn_ref::<HtmlElement>() {
                    if let Some(id) = li.get_attribute("data-id") {
                        if !id.is_empty() {
                            is_open.set(!*is_open);
                            integration_id.set(id.clone());
                        }
                    }
                }
            }
        })
    };

    let delete_callback = {
        let is_delete_open = is_delete_open.clone();
        let integration_id = integration_id.clone();
        Callback::from(move |event: MouseEvent| {
            if let Some(target) = event.target() {
                if let Some(li) = target.dyn_ref::<HtmlElement>() {
                    if let Some(id) = li.get_attribute("data-id") {
                        if !id.is_empty() {
                            is_delete_open.set(!*is_delete_open);
                            integration_id.set(id.clone());
                        }
                    }
                }
            }
        })
    };

    let cloned_integrations = integrations.clone();
    let cloned_pagination = pagination.clone();
    let cloned_initial = initial.clone();
    let cloned_search_text = search_text.clone();
    let token = token.clone();
    let fetch_handle_integrations = move |()| {
        let integrations = cloned_integrations.clone();
        let pagination = cloned_pagination.clone();
        let cloned_initial = cloned_initial.clone();
        let search_text = cloned_search_text.clone();
        let token = token.clone();
        spawn_local(async move {
            let response = fetch_integrations(
                token,
                pagination.per_page,
                pagination.current_page,
                search_text.to_string(),
            )
            .await;

            match response {
                Ok(response) => {
                    integrations.set(response.data);
                    cloned_initial.set(false);
                    // pagination.set(PaginationDataProps {
                    //     current_page: response.page,
                    //     per_page: response.per_page,
                    //     total_items: response.total,
                    //     total_pages: response.total_pages,
                    // })
                }
                Err(e) => log!("Error: {}", e.to_string()),
            }
        });
    };

    let export = {
        let integrations = integrations.clone();
        Callback::from(move |_: MouseEvent| {
            let mut csv_data = "Name|API key|Secret key|Status".to_string();

            for integration in integrations.iter() {
                let data: String = format!(
                    "\n{}|{}|{}|{}",
                    integration.clone().name,
                    integration.clone().api_key,
                    integration.clone().secret_key,
                    integration.clone().status,
                );
                csv_data = csv_data + data.as_str();
            }

            download_csv_file(csv_data.as_str())
        })
    };

    {
        let onfetch = fetch_handle_integrations.clone();
        let pagination = pagination.clone();
        let search_text = search_text.clone();
        use_effect_with((pagination.clone(), search_text.clone()), move |_| {
            if *initial {
                onfetch(());
            }

            || {}
        });
    }

    html!(
        <>
            <div class="bg-grey-shade-13 py-4 px-8">
                <div class="container mx-auto md:w-auto space-y-4">
                    <div class="flex md:justify-between md:items-center flex-col gap-6 md:flex-row">
                        <div class="flex flex-row justify-between md:justify-normal gap-2 items-center w-full md:w-auto">
                            <h1>{"Integrations"}</h1>
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
                                {"Add new integration"}
                            </button>
                        </div>
                    </div>
                    <ContextProvider<PaginationDataProps> context={(*pagination).clone()}>
                        <Paginator update_pagination={update_pagination.clone()} />
                    </ContextProvider<PaginationDataProps>>
                </div>
            </div>
            <div class="container mx-auto px-8  py-4  grid gap-3 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
                {
                    integrations.clone().iter().map(|integration| {
                        html!{
                            <Card
                                integration={integration.clone()}
                                edit_callback={edit_callback.clone()}
                                delete_callback={delete_callback.clone()}
                            />
                        }
                    }).collect::<Html>()
                }
            </div>
            { if *is_open {
                    html! {
                        <Modal
                            modal_handle={modal_handle.clone()}
                            on_ok_response={on_ok_response.clone()}
                            integration_id={integration_id.to_string()}
                            fetch_handle_integrations={fetch_handle_integrations.clone()}
                        />
                    }
                } else {
                    html! ("")
                }
            }
            { if *is_delete_open {
                html! {
                        <DeleteModal
                            delete_modal_handle={delete_modal_handle.clone()}
                            on_ok_response={on_ok_delete_handle.clone()}
                            fetch_handle_integrations={fetch_handle_integrations.clone()}
                            integration_id={integration_id.to_string()}
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
struct CardProps {
    integration: IntegrationResponse,
    edit_callback: Callback<MouseEvent>,
    delete_callback: Callback<MouseEvent>,
}
#[function_component(Card)]
fn card(props: &CardProps) -> Html {
    let integration = props.integration.clone();
    let edit_callback = props.edit_callback.clone();
    let delete_callback = props.delete_callback.clone();
    let api_key = use_state(String::default);
    let show_apikey = use_state(|| true);

    let secret_key = use_state(String::default);
    let show_secret = use_state(|| true);

    let on_apikey_input = {
        let apikey_handle = api_key.clone();

        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            apikey_handle.set(value.clone());
        })
    };

    let toggle_secretkey = {
        let show_secret = show_secret.clone();

        Callback::from(move |_| {
            show_secret.set(!*show_secret);
        })
    };

    let on_secretkey_input = {
        let secretkey_handle = secret_key.clone();

        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            secretkey_handle.set(value.clone());
        })
    };

    let toggle_apikey = {
        let show_apikey = show_apikey.clone();

        Callback::from(move |_| {
            show_apikey.set(!*show_apikey);
        })
    };

    html! {
        <div class="p-4  rounded-xl border border-grey-shade-11 space-y-3">
            <div class="flex justify-between items-center">
                <h1 class="text-18 font-sans">
                    {integration.clone().name}
                </h1>
                <div class="w-[4px] relative group cursor-pointer px-4">
                    <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                    <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                        <li onclick={edit_callback.clone()} data-id={integration.clone().id} class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">{"Edit"}</li>
                        <li onclick={delete_callback.clone()} data-id={integration.clone().id} class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">{"Delete"}</li>
                    </ul>
                </div>
            </div>
            <div class="flex flex-col space-y-1">
                <Label
                    label =  "API key"
                    label_for = "apikey"
                    label_style= {LabelStyle::Secondary}
                />
                <div class="flex items-center rounded border justify-start border-grey-shade-11 px-2">
                    <TextInput
                        id="apikey"
                        value={integration.clone().api_key}
                        placeholder="API key"
                        helper_text="Enter valid api key"
                        input_handler={on_apikey_input.clone()}
                        input_type={if *show_apikey { "password" } else { "text" }}
                        left_icon="mdi:key"
                    />
                    <button
                        type="button"
                        class="cursor-pointer"
                        onclick={toggle_apikey}
                    >
                        {html! { render_svg!("mdi:eye", color="#949494" )}}
                    </button>
                </div>
            </div>

            <div class="flex flex-col space-y-1">
                <Label
                    label =  "Secret key"
                    label_for = "secretkey"
                    label_style= {LabelStyle::Secondary}
                />
                <div class="flex items-center rounded border justify-start border-grey-shade-11 px-2">
                    <TextInput
                        id="secretkey"
                        value={integration.clone().secret_key}
                        placeholder="Secret key"
                        helper_text="Enter valid secret key"
                        input_handler={on_secretkey_input.clone()}
                        input_type={if *show_secret { "password" } else { "text" }}
                        left_icon="mdi:key"
                    />
                    <button
                        type="button"
                        class="cursor-pointer"
                        onclick={toggle_secretkey}
                    >
                        {html! { render_svg!("mdi:eye", color="#949494" )}}
                    </button>
                </div>
            </div>

            <div class="flex space-x-4  items-center justify-start p-0 rounded-sm">
                <div  class="bg-success flex capitalize items-center rounded-lg px-3 py-1 text-grey-shade-14 text-14 font-400">
                    {integration.clone().status}
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ModalProps {
    modal_handle: Callback<MouseEvent>,
    on_ok_response: Callback<()>,
    integration_id: String,
    fetch_handle_integrations: Callback<()>,
}

#[function_component(Modal)]
fn edit_modal(props: &ModalProps) -> Html {
    let integration = use_state(Integration::default);

    let (auth_store, _) = use_store::<AuthStore>();

    let token = auth_store.token.clone();

    let cloned_integration = integration.clone();
    let state_changed = Callback::from(move |event: Event| {
        let mut data = cloned_integration.deref().clone();
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
            "api_key" => {
                data.api_key = value;
            }
            "name" => {
                data.name = value;
            }
            "secret_key" => {
                data.secret_key = value;
            }
            "status" => {
                if checked == true {
                    data.status = "Active".to_string();
                } else {
                    data.status = "Inactive".to_string();
                }
            }
            _ => (),
        }
        cloned_integration.set(data);
    });

    let save_integration_handler = {
        let cloned_integration = (*integration).clone();
        let on_ok = props.on_ok_response.clone();
        let on_handle_integrations = props.fetch_handle_integrations.clone();
        let integration_id = props.integration_id.clone();
        let token = token.clone();
        Callback::from(move |_event: MouseEvent| {
            let integration: Integration = cloned_integration.clone();
            let on_ok = on_ok.clone();
            let on_handle_integrations = on_handle_integrations.clone();
            let integration_id = integration_id.clone();
            let token = token.clone();
            spawn_local(async move {
                if !integration_id.is_empty() {
                    let response = update_integration(token, integration, integration_id).await;

                    match response {
                        Ok(_response) => {
                            on_ok.emit(());
                            on_handle_integrations.emit(());
                        }
                        Err(e) => log!("Error: ", e.to_string()),
                    }
                } else {
                    let response = create_integration(token, integration).await;

                    match response {
                        Ok(_response) => {
                            on_ok.emit(());
                            on_handle_integrations.emit(());
                        }
                        Err(e) => log!("Error: ", e.to_string()),
                    }
                }
            });
        })
    };

    let cloned_integration = integration.clone();
    let fetch_handle_integration = move |id: String| {
        let integration = cloned_integration.clone();
        spawn_local(async move {
            let response = fetch_integration(token, id).await;

            match response {
                Ok(response) => {
                    integration.set(Integration {
                        api_key: response.clone().api_key,
                        name: response.clone().name,
                        secret_key: response.clone().secret_key,
                        status: response.clone().status,
                    });
                }
                Err(e) => log!("Error: {}", e.to_string()),
            }
        });
    };

    let onfetch = fetch_handle_integration.clone();
    let integration_id = props.integration_id.clone();
    use_effect_with((), move |_| {
        if !integration_id.is_empty() {
            onfetch(integration_id); // Call the async function
        }
        || {}
    });

    html! {
        <div class="z-10 fixed inset-0 bg-grey-shade-0/70 w-screen flex h-screen items-center justify-center">
            <div class=" bg-white p-7 rounded-sm space-y-7">
                <div class="flex items-center justify-between">
                    <p>{"Add new integration"}</p>
                    <span class="cursor-pointer" onclick={props.modal_handle.clone()}>
                        {html! {render_svg!("mdi:multiply",color="#232323",width="25px")}}
                    </span>
                </div>

                <div class="flex flex-col space-y-1.5 w-full md:w-[600px]">
                    <label
                        for="name"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"Platform name"}
                    </label>
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start px-2" >
                        <input
                            id="name"
                            name="name"
                            placeholder="Platform name"
                            onchange={state_changed.clone()}
                            value={integration.name.clone()}
                            class="px-3.5 py-3placeholder:text-grey-shade-6 text-14 leading-20
                            bg-white
                            h-10
                            w-full  
                            md:w-72
                            border-grey-shade-11
                            font-300 font-sans outline-none
                            pr-2 pl-2"
                        />
                    </div>
                </div>

                <div class="flex flex-col space-y-1.5 w-full md:w-[600px]">
                    <label
                        for="api_key"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                        {"API key"}
                    </label>
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start px-2" >
                        <input
                            id="api_key"
                            name="api_key"
                            placeholder="API key"
                            onchange={state_changed.clone()}
                            value={integration.api_key.clone()}
                            class="px-3.5 py-3placeholder:text-grey-shade-6 text-14 leading-20 bg-white h-10 w-full md:w-72 border-grey-shade-11 font-300 font-sans outline-none pr-2 pl-2"
                        />
                    </div>
                </div>

                <div class="flex flex-col space-y-1.5 w-full md:w-[600px]">
                    <label
                        for="secret_key"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"Secret key"}
                    </label>
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start px-2" >
                        <input
                            id="secret_key"
                            name="secret_key"
                            placeholder="Secret key"
                            onchange={state_changed.clone()}
                            value={integration.secret_key.clone()}
                            class="px-3.5 py-3placeholder:text-grey-shade-6 text-14 leading-20
                            bg-white
                            h-10
                            w-full  
                            md:w-72
                            border-grey-shade-11
                            font-300 font-sans outline-none
                            pr-2 pl-2"
                        />
                    </div>
                </div>

                <div class="flex items-center justify-start  toggle-switch">
                    <label for="toggler">
                        {"Enabled"}
                    </label>
                    <input
                        type="checkbox"
                        id="toggler"
                        name="status"
                        onchange={state_changed.clone()}
                        checked={if integration.clone().status.to_uppercase() == "ACTIVE" {true} else {false}}
                        class="appearance-none"
                    />
                </div>

                <div class="flex space-x-4  items-center justify-start p-0 rounded-sm">
                    <button type="button" onclick={save_integration_handler}  class="bg-primary flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
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
    pub fetch_handle_integrations: Callback<()>,
    pub integration_id: String,
    delete_modal_handle: Callback<MouseEvent>,
}

#[function_component(DeleteModal)]
fn delete_modal(props: &DeleteModalProps) -> Html {
    let (auth_store, _) = use_store::<AuthStore>();

    let token = auth_store.token.clone();

    let delete_integration_handler = {
        let on_ok = props.on_ok_response.clone();
        let on_handle_integrations = props.fetch_handle_integrations.clone();
        let integration_id = props.integration_id.clone();
        let token = token.clone();
        Callback::from(move |_event: MouseEvent| {
            let on_ok = on_ok.clone();
            let on_handle_integrations = on_handle_integrations.clone();
            let integration_id = integration_id.clone();
            let token = token.clone();
            spawn_local(async move {
                if !integration_id.is_empty() {
                    let response = delete_integration(token, integration_id).await;

                    match response {
                        Ok(_response) => {
                            on_ok.emit(());
                            on_handle_integrations.emit(());
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
                    <button type="button" onclick={delete_integration_handler} class="bg-primary flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
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
