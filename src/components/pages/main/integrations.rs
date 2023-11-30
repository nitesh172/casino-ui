use crate::{
    components::{
        atoms::{
            label::{Label, LabelStyle},
            text_input::TextInput,
        },
        organisms::{
            export_button::ExportButton,
            paginator::{PaginationDataProps, PaginationFucProps, Paginator},
        },
    },
    render_svg,
};
use std::ops::Deref;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;

#[function_component(Integrations)]
pub fn integrations() -> Html {
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
                <Card />
                <Card />
                <Card />
                <Card />
                <Card />
                <Card />
                <Card />
                <Card />
                <Card />
                <Card />
                <Card />
            </div>
            { if *is_open {html! {<Modal modal_handle={modal_handle.clone()}  />}  } else { html! ("") } }
        </>
    )
}

#[function_component(Card)]
fn card() -> Html {
    let api_key = use_state(String::default);
    let show_apikey = use_state(|| false);

    let secret_key = use_state(String::default);
    let show_secret = use_state(|| false);

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
                    {"EVOLUTION"}
                </h1>
                <div class="w-[4px] relative group cursor-pointer px-4">
                    <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                    <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                        <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">
                            <a href="#" class="">
                            { "Edit" }
                            </a>
                        </li>
                        <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">
                            <a href="#" class="">
                                { "Delete" }
                            </a>
                        </li>
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
                        value={(*api_key).clone()}
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
                        value={(*secret_key).clone()}
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
                <button type="submit"  class="bg-success flex items-center rounded-lg px-3 py-1 text-grey-shade-14 text-14 font-400">
                    {"Active"}
                </button>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ModalProps {
    modal_handle: Callback<MouseEvent>,
}

#[function_component(Modal)]
fn edit_modal(props: &ModalProps) -> Html {
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
                        for="platformname"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"Platform name"}
                    </label>
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start px-2" >
                        <input
                            id="platformname"
                            name="platformname"
                            placeholder="Platform name"
                            // oninput={props.on_username_input.clone()}
                            // value={props.user.name.clone()}
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
                        for="apikey"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"API key"}
                    </label>
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start px-2" >
                        <input
                            id="apikey"
                            name="apikey"
                            placeholder="API key"
                            // oninput={props.on_username_input.clone()}
                            // value={props.user.name.clone()}
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
                        for="secretkey"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"Secret key"}
                    </label>
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start px-2" >
                        <input
                            id="secretkey"
                            name="secretkey"
                            placeholder="Secret key"
                            // oninput={props.on_username_input.clone()}
                            // value={props.user.name.clone()}
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
                    <input type="checkbox" id="toggler"         class="appearance-none" />
                </div>

                <div class="flex space-x-4  items-center justify-start p-0 rounded-sm">
                    <button type="submit"  class="bg-primary flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
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
