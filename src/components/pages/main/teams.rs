use crate::{
    apis::{
        team::{
            create_user,
            delete_user,
            fetch_user,
            fetch_users,
            update_user,
            User,
            UserResponse,
        },
        user::api_forgot_password,
    },
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
use web_sys::{ wasm_bindgen::JsCast, HtmlElement, HtmlInputElement, HtmlSelectElement };
use yew::{ platform::spawn_local, prelude::* };
use yewdux::prelude::*;

#[function_component(Teams)]
pub fn teams() -> Html {
    let t_head = vec!["Username", "Email ID", "Reset password", "Role", "Status", "Actions"];

    let (auth_store, _) = use_store::<AuthStore>();

    let token = auth_store.token.clone();

    let is_open = use_state(|| false);
    let is_delete_open = use_state(|| false);
    let search_text = use_state(|| String::default());
    let initial = use_state(|| true);
    let users = use_state(|| Vec::<UserResponse>::default());
    let pagination = use_state(|| PaginationDataProps {
        per_page: 10,
        total_items: 0,
        total_pages: 0,
        current_page: 1,
    });
    let user_id = use_state(|| String::default());

    let toasts_data = use_context::<ToastProps>().expect("no ctx found");

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

    let modal_handle = {
        let is_open = is_open.clone();
        let user_id = user_id.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
            user_id.set("".to_string());
        })
    };

    let delete_modal_handle = {
        let is_delete_open = is_delete_open.clone();
        let user_id = user_id.clone();
        Callback::from(move |_| {
            is_delete_open.set(!*is_delete_open);
            user_id.set("".to_string());
        })
    };

    let on_ok_response = {
        let is_open = is_open.clone();
        let user_id = user_id.clone();
        Callback::from(move |_| {
            is_open.set(false);
            user_id.set("".to_string());
        })
    };

    let on_ok_delete_handle = {
        let is_delete_open = is_delete_open.clone();
        let user_id = user_id.clone();
        Callback::from(move |_| {
            is_delete_open.set(!*is_delete_open);
            user_id.set("".to_string());
        })
    };

    let edit_callback = {
        let is_open = is_open.clone();
        let user_id = user_id.clone();
        Callback::from(move |event: MouseEvent| {
            if let Some(target) = event.target() {
                if let Some(li) = target.dyn_ref::<HtmlElement>() {
                    if let Some(id) = li.get_attribute("data-id") {
                        if !id.is_empty() {
                            is_open.set(!*is_open);
                            user_id.set(id.clone());
                        }
                    }
                }
            }
        })
    };

    let reset_password_callback = {
        let add_toast = toasts_data.add_toast.clone();
        Callback::from(move |event: MouseEvent| {
            if let Some(target) = event.target() {
                if let Some(div) = target.dyn_ref::<HtmlElement>() {
                    if let Some(email_address) = div.get_attribute("data-id") {
                        if !email_address.is_empty() {
                            let add_toast = add_toast.clone();
                            spawn_local(async move {
                                let response = api_forgot_password(email_address).await;

                                match response {
                                    Ok(_response) => {
                                        add_toast.emit("Reset password mail sent sucessfully.".to_string());
                                    }
                                    Err(e) => log!("Error: ", e.to_string()),
                                }
                            });
                        }
                    }
                }
            }
        })
    };

    let delete_callback = {
        let is_delete_open = is_delete_open.clone();
        let user_id = user_id.clone();
        Callback::from(move |event: MouseEvent| {
            if let Some(target) = event.target() {
                if let Some(li) = target.dyn_ref::<HtmlElement>() {
                    if let Some(id) = li.get_attribute("data-id") {
                        if !id.is_empty() {
                            is_delete_open.set(!*is_delete_open);
                            user_id.set(id.clone());
                        }
                    }
                }
            }
        })
    };

    let cloned_search_text = search_text.clone();
    let cloned_initial = initial.clone();
    let search_text_changed = Callback::from(move |event: Event| {
        let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        cloned_initial.set(true);
        cloned_search_text.set(value);
    });

    let cloned_users = users.clone();
    let cloned_pagination = pagination.clone();
    let cloned_initial = initial.clone();
    let cloned_search_text = search_text.clone();
    let token = token.clone();
    let fetch_handle_users = move |()| {
        let users = cloned_users.clone();
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
                    users.set(response.result);
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

    let export = {
        let users = users.clone();
        let add_toast = toasts_data.add_toast.clone();
        Callback::from(move |_: MouseEvent| {
            let mut csv_data = "Username|Email ID|Role|Status".to_string();
            let add_toast = add_toast.clone();

            for user in users.iter() {
                let data: String = format!(
                    "\n{}|{}|{}|{}",
                    user.clone().name,
                    user.clone().email_address,
                    user.clone().roles,
                    user.clone().status
                );
                csv_data = csv_data + data.as_str();
            }

            download_csv_file(csv_data.as_str(), add_toast)
        })
    };

    {
        let onfetch = fetch_handle_users.clone();
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
            <div class="container mx-auto md:w-auto space-y-4" >
                <div class="flex md:justify-between md:items-center flex-col gap-6 md:flex-row">
                    <div class="flex flex-row justify-between md:justify-normal gap-2 items-center w-full md:w-auto">
                        <h1>{"Teams"}</h1>
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
                            {"Invite team member"}
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
                        <tr class="">
                            {
                                t_head.into_iter().map(|name| {
                                    html!{<th key={name} class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{name}</th>}
                                }).collect::<Html>()
                            }
                        </tr>
                    </thead>
                    <tbody class="overflow-y-auto">
                        {
                            users.clone().iter().map(|user| {
                                html!{
                                    <tr>
                                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1">{user.clone().name}</td>
                                        <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{user.clone().email_address}</td>
                                        <td class="py-3  text-left text-14 font-medium cursor-pointer text-warning tracking-wider">
                                            <div onclick={reset_password_callback.clone()} data-id={user.clone().email_address} class="flex flex-row gap-0.5">
                                                <span>{html! { render_svg!    ("material-symbols-light:lock-sharp", color="#C53A3A",width="18px")}}</span>
                                                {"Reset password"}
                                            </div>
                                        </td>
                                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">
                                            <span class="rounded-full py-1 px-2 flex-row gap-1 bg-grey-shade-11 capitalize">
                                                {user.clone().roles}
                                            </span>
                                        </td>
                                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">
                                            <span class={format!("rounded-full py-1 px-2 flex-row gap-1 text-white capitalize {} ", if user.clone().status == "Active" {"bg-success"} else {"bg-warning"})}>
                                                {user.clone().status}
                                            </span>
                                        </td>
                                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider relative group cursor-pointer">
                                            <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                            <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                                <li onclick={edit_callback.clone()} data-id={user.clone().id}  class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">{"Edit"}</li>
                                                <li onclick={delete_callback.clone()} data-id={user.clone().id} class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">{"Delete"}</li>
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
                        users.clone().iter().map(|user| {
                            html!{
                                <div class="rounded-xl border p-4 flex flex-col gap-2.5">
                                    <div class="flex flex-row justify-between">
                                        <div class="flex flex-col gap-2.5">
                                            <div class="rounded-full w-fit text-sm py-1 px-2 flex-row gap-1 bg-grey-shade-11 capitalize">
                                                {user.clone().roles}
                                            </div>
                                            <div class="flex flex-col">
                                                <div class="font-400">{user.clone().name}</div>
                                                <div class="text-12 text-grey-shade-5">{user.clone().email_address}</div>
                                            </div>
                                        </div>
                                        <div class="text-12 font-medium text-grey-shade-1 relative group cursor-pointer">
                                            <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                            <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                                <li onclick={edit_callback.clone()} data-id={user.clone().id} class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">{"Edit"}</li>
                                                <li onclick={delete_callback.clone()} data-id={user.clone().id} class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">{"Delete"}</li>
                                            </ul>
                                        </div>
                                    </div>
                                    <div class="flex flex-row justify-between items-center">
                                        <div onclick={reset_password_callback.clone()} data-id={user.clone().email_address} class="text-12 font-medium cursor-pointer text-warning flex flex-row gap-0.5">
                                            <span>{html! { render_svg!    ("material-symbols-light:lock-sharp", color="#C53A3A",width="18px")}}</span>
                                            {"Reset password"}
                                        </div>
                                        <div class={format!("rounded-full py-1 px-2 flex-row gap-1 text-white capitalize {} ", if user.clone().status == "Active" {"bg-success"} else {"bg-warning"})}>
                                            {user.clone().status}
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
                        fetch_handle_users={fetch_handle_users.clone()}
                        user_id={user_id.to_string()}
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
                        fetch_handle_users={fetch_handle_users.clone()}
                        user_id={user_id.to_string()}
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
struct ModalProps {
    pub on_ok_response: Callback<()>,
    pub fetch_handle_users: Callback<()>,
    pub user_id: String,
    modal_handle: Callback<MouseEvent>,
}

#[function_component(Modal)]
fn edit_modal(props: &ModalProps) -> Html {
    let user = use_state(User::default);

    let cloned_user = user.clone();
    let state_changed = Callback::from(move |event: Event| {
        let mut data = cloned_user.deref().clone();
        let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        let select_value = event.target().unwrap().unchecked_into::<HtmlSelectElement>().value();
        let name = event.target().unwrap().unchecked_into::<HtmlInputElement>().name();
        let checked = event.target().unwrap().unchecked_into::<HtmlInputElement>().checked();

        log!(checked.clone());

        match name.as_str() {
            "email_address" => {
                data.email_address = value;
            }
            "name" => {
                data.name = value;
            }
            "password" => {
                data.password = value;
            }
            "roles" => {
                data.roles = select_value;
            }
            "status" => {
                if checked == true {
                    data.status = "Active".to_string();
                } else {
                    data.status = "InActive".to_string();
                }
            }
            _ => (),
        }
        cloned_user.set(data);
    });

    let (auth_store, _) = use_store::<AuthStore>();

    let token = auth_store.token.clone();

    let toasts_data = use_context::<ToastProps>().expect("no ctx found");

    let save_user_handler = {
        let st = (*user).clone();
        let on_ok = props.on_ok_response.clone();
        let on_handle_users = props.fetch_handle_users.clone();
        let user_id = props.user_id.clone();
        let token = token.clone();
        let add_toast = toasts_data.add_toast.clone();
        Callback::from(move |_event: MouseEvent| {
            let user: User = st.clone();
            let on_ok = on_ok.clone();
            let on_handle_users = on_handle_users.clone();
            let user_id = user_id.clone();
            let token = token.clone();
            let add_toast = add_toast.clone();
            spawn_local(async move {
                if !user_id.is_empty() {
                    let response = update_user(token, user, user_id).await;

                    match response {
                        Ok(_response) => {
                            on_ok.emit(());
                            on_handle_users.emit(());
                            add_toast.emit("User updated successfully.".to_string());
                        }
                        Err(e) => log!("Error: ", e.to_string()),
                    }
                } else {
                    let response = create_user(token, user).await;

                    match response {
                        Ok(_response) => {
                            on_ok.emit(());
                            on_handle_users.emit(());
                            add_toast.emit("User created successfully.".to_string());
                        }
                        Err(e) => log!("Error: ", e.to_string()),
                    }
                }
            });
        })
    };

    let cloned_user = user.clone();
    let fetch_handle_user = move |id: String| {
        let user = cloned_user.clone();
        let token = token.clone();
        spawn_local(async move {
            let response = fetch_user(token, id).await;

            match response {
                Ok(response) => {
                    log!("user");
                    user.set(User {
                        email_address: response.email_address,
                        name: response.name,
                        roles: response.roles,
                        status: response.status,
                        password: "".to_string(),
                    });
                }
                Err(e) => log!("Error: {}", e.to_string()),
            }
        });
    };

    let onfetch = fetch_handle_user.clone();
    let user_id = props.user_id.clone();
    use_effect_with((), move |_| {
        if !user_id.is_empty() {
            onfetch(user_id); // Call the async function
        }
        || {}
    });

    html! {
        <div class="z-10 fixed inset-0 bg-grey-shade-0/70 w-screen flex h-screen items-center justify-center p-5">
            <div class=" bg-white p-7 rounded-sm space-y-7 w-full md:w-auto">
                <div class="flex items-center justify-between">
                    <p>{"Invite team member"}</p>
                    <span class="cursor-pointer" onclick={props.modal_handle.clone()}>
                        {html! {render_svg!("mdi:multiply",color="#232323",width="25px")}}
                    </span>
                </div>
                <div class="flex flex-col space-y-1.5 w-full md:w-[400px]">
                    <label
                        for="name"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"Username"}
                    </label>
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start px-2" >
                        <input
                            id="name"
                            name="name"
                            placeholder="Name"
                            value={user.name.clone()}
                            onchange={state_changed.clone()}
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

                {
                    if props.user_id.clone().is_empty() {
                        html!{
                            <div class="flex flex-col space-y-1.5 w-full md:w-[400px]">
                                <label
                                    for="email_address"
                                    class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                                >
                                        {"Email ID"}
                                </label>
                                <div class="flex items-center rounded border border-grey-shade-11 justify-start px-2" >
                                    <input
                                        id="email_address"
                                        name="email_address"
                                        placeholder="Email ID"
                                        value={user.email_address.clone()}
                                        onchange={state_changed.clone()}
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
                        }
                    } else {
                        html!{}
                    }
                }

                {
                    if props.user_id.clone().is_empty() {
                        html!{
                            <div class="flex flex-col space-y-1.5 w-full md:w-[400px]">
                                <label
                                    for="password"
                                    class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                                >
                                        {"Password"}
                                </label>
                                <div class="flex items-center rounded border border-grey-shade-11 justify-start px-2" >
                                    <input
                                        id="password"
                                        name="password"
                                        placeholder="Password"
                                        type="password"
                                        value={user.password.clone()}
                                        onchange={state_changed.clone()}
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
                        }
                    } else {
                        html!{}
                    }
                }

                <div class="flex flex-col space-y-1.5 md:w-[400px]">
                    <label
                        for="roles"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"Role"}
                    </label>
                    <div class="relative inline-flex w-full ">
                        <select
                            value={user.roles.clone()}
                            id="roles"
                            name="roles"
                            onchange={state_changed.clone()}
                            class="appearance-none border border-gray-300 rounded px-3 py-2 focus:outline-none w-full"
                        >
                            <option selected={user.roles.clone() == ""} >{"Select"}</option>
                            <option selected={user.roles.clone() == "Admin"}>{"Admin"}</option>
                            <option selected={user.roles.clone() == "Member"}>{"Member"}</option>
                        </select>
                        <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
                            <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 11.586l3.293-3.293a1 1 0 011.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
                            </svg>
                        </div>
                    </div>
                </div>

                <div class="flex items-center justify-start  toggle-switch">
                    <label for="status">
                        {"Enabled"}
                    </label>
                    <input
                        type="checkbox"
                        id="toggler"
                        name="status"
                        checked={if user.status.clone() == "Active" {true} else {false}}
                        onchange={state_changed.clone()}
                        class="appearance-none"
                    />
                </div>

                <div class="flex space-x-4  items-center justify-start p-0 rounded-sm">
                    <button type="button" onclick={save_user_handler} class="bg-primary flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
                        {"Invite"}
                    </button>
                    <button class="bg-grey-shade-14 flex items-center rounded p-2 text-primary text-12 font-400" onclick={props.modal_handle.clone()}>
                        {"Cancel"}
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
    pub fetch_handle_users: Callback<()>,
    pub user_id: String,
    delete_modal_handle: Callback<MouseEvent>,
}

#[function_component(DeleteModal)]
fn delete_modal(props: &DeleteModalProps) -> Html {
    let (auth_store, _) = use_store::<AuthStore>();

    let token = auth_store.token.clone();

    let toasts_data = use_context::<ToastProps>().expect("no ctx found");

    let delete_user_handler = {
        let on_ok = props.on_ok_response.clone();
        let on_handle_users = props.fetch_handle_users.clone();
        let user_id = props.user_id.clone();
        let token = token.clone();
        let add_toast = toasts_data.add_toast.clone();
        Callback::from(move |_event: MouseEvent| {
            let on_ok = on_ok.clone();
            let on_handle_users = on_handle_users.clone();
            let user_id = user_id.clone();
            let token = token.clone();
            let add_toast = add_toast.clone();
            spawn_local(async move {
                if !user_id.is_empty() {
                    let response = delete_user(token, user_id).await;

                    match response {
                        Ok(_response) => {
                            on_ok.emit(());
                            on_handle_users.emit(());
                            add_toast.emit("User deleted successfully.".to_string())
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
                    <button type="button" onclick={delete_user_handler} class="bg-primary flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
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
