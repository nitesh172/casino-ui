use crate::{
    apis::user::{
        api_delete_user,
        api_me,
        api_update_user,
        fetch_user_notification,
        update_user_notification,
        CurrentUser,
        UserNotification,
        update_email,
        api_forgot_password,
        upload_file,
        Media,
        save_file_url,
        UpdateProfile
    },
    render_svg,
    stores::auth_store::AuthStore,
};
use gloo_console::log;
use std::ops::Deref;
use web_sys::{ wasm_bindgen::{ JsCast, JsValue }, HtmlInputElement, Blob, BlobPropertyBag, Url };
use yew::{ platform::spawn_local, prelude::* };
use yewdux::prelude::*;

#[derive(Clone, Properties, PartialEq)]
struct UserNotificationList {
    name: String,
    value: bool,
    label: String,
}

#[function_component(Settings)]
pub fn settings() -> Html {
    let is_open = use_state(|| false);
    let is_email_update = use_state(|| false);
    let is_reset_password = use_state(|| false);
    let image_s3 = use_state(|| String::default());
    let (auth_store, auth_dispatch) = use_store::<AuthStore>();
    let is_delete_open = use_state(|| false);
    let user = use_state(|| CurrentUser::default());
    let user_notification = use_state(|| UserNotification::default());

    let on_username_input = {
        let user_state = user.clone();
        Callback::from(move |event: InputEvent| {
            let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
            log!(value.clone());

            let mut user = (*user_state).clone();

            user.name = Some(value);

            user_state.set(user);
        })
    };

    let edit_modal_handle = {
        let is_open = is_open.clone();

        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };

    let delete_modal_handle = {
        let is_delete_open = is_delete_open.clone();
        Callback::from(move |_| {
            is_delete_open.set(!*is_delete_open);
        })
    };

    let on_ok_delete_handle = {
        let is_delete_open = is_delete_open.clone();
        Callback::from(move |_| {
            is_delete_open.set(!*is_delete_open);
        })
    };

    let email_update_handle = {
        let is_email_update = is_email_update.clone();
        Callback::from(move |_| {
            is_email_update.set(!*is_email_update);
        })
    };

    let on_ok_email_handle = {
        let is_email_update = is_email_update.clone();
        Callback::from(move |_| {
            is_email_update.set(!*is_email_update);
        })
    };

    let reset_password_handle = {
        let is_reset_password = is_reset_password.clone();
        Callback::from(move |_| {
            is_reset_password.set(!*is_reset_password);
        })
    };

    let on_ok_password_handle = {
        let is_reset_password = is_reset_password.clone();
        Callback::from(move |_| {
            is_reset_password.set(!*is_reset_password);
        })
    };

    let cloned_token = auth_store.token.clone();
    let cloned_dispatch = auth_dispatch.clone();
    let api_me_fn = move |()| {
        let token = cloned_token.clone();
        let store_dispatch = cloned_dispatch.clone();
        spawn_local(async move {
            let response = api_me(token).await;

            match response {
                Ok(response) => {
                    store_dispatch.reduce_mut(move |store| {
                        store.current_user = response;
                    });
                },
                Err(e) => log!("Error: {}", e.to_string()),
            }
        });
    };

    let update_user = {
        let user_cloned = (*user).clone();
        let token = auth_store.token.clone();
        let api_me_fn = api_me_fn.clone();
        let image_s3 = image_s3.clone();
        let is_open = is_open.clone();
        Callback::from(move |_event: MouseEvent| {
            let user = user_cloned.clone();
            let token = token.clone();
            let api_me_fn = api_me_fn.clone();
            let image_s3 = image_s3.clone();
            let is_open = is_open.clone();
            spawn_local(async move {
                let response = api_update_user(token, UpdateProfile {
                    name: user.name,
                    image_url: format!("{}", 
                        if (*image_s3).clone() == "1" {
                            "".to_string() 
                        } else if !image_s3.is_empty() { 
                            (*image_s3).clone() 
                        } else if !user.image_url.s3_key.is_empty() {
                            user.image_url.s3_key.clone() 
                        } else { 
                            "".to_string() 
                        }
                    ),
                    id: user.id
                }).await;

                match response {
                    Ok(_response) => {
                        api_me_fn(());
                        is_open.set(!*is_open);
                    }
                    // Ok(response) => user.clone().set(response),
                    Err(e) => log!("Error: {}", e.to_string()),
                }
            });
        })
    };

    let auth_user = auth_store.current_user.clone();
    let cloned_user = user.clone();
    use_effect_with(auth_user.clone(), move |_| {
        if !auth_user.id.is_empty() {
            cloned_user.set(auth_user);
        }
        || {}
    });

    let logout_handle = auth_dispatch.reduce_mut_callback_with(|store, _event: MouseEvent|
        store.reset_to_default()
    );

    let list_notification = vec![
        UserNotificationList {
            name: "is_push_enabled".to_string(),
            value: user_notification.clone().is_push_enabled,
            label: "Push notifications".to_string(),
        },
        UserNotificationList {
            name: "is_email_enabled".to_string(),
            value: user_notification.clone().is_email_enabled,
            label: "Email notifications".to_string(),
        },
        UserNotificationList {
            name: "is_sms_enabled".to_string(),
            value: user_notification.clone().is_sms_enabled,
            label: "SMS notifications".to_string(),
        }
    ];

    let cloned_user_notification = user_notification.clone();
    let cloned_token: String = auth_store.token.clone();
    let notification_changed = Callback::from(move |event: Event| {
        let mut data = cloned_user_notification.deref().clone();
        let notification_handler = cloned_user_notification.clone();
        let token = cloned_token.clone();
        let name = event.target().unwrap().unchecked_into::<HtmlInputElement>().name();
        let checked = event.target().unwrap().unchecked_into::<HtmlInputElement>().checked();

        log!(checked.clone());

        match name.as_str() {
            "is_sms_enabled" => {
                data.is_sms_enabled = checked;
            }
            "is_email_enabled" => {
                data.is_email_enabled = checked;
            }
            "is_push_enabled" => {
                data.is_push_enabled = checked;
            }
            _ => (),
        }

        spawn_local(async move {
            let response = update_user_notification(token, data).await;
            match response {
                Ok(response) => {
                    notification_handler.set(UserNotification {
                        is_email_enabled: response.is_email_enabled,
                        is_push_enabled: response.is_push_enabled,
                        is_sms_enabled: response.is_sms_enabled,
                    });
                }
                Err(e) => log!("Error: {}", e.to_string()),
            }
        });
    });

    let token = auth_store.token.clone();
    let cloned_user_notification = user_notification.clone();
    let fetch_notification = move || {
        let token = token.clone();
        let user_notification = cloned_user_notification.clone();
        spawn_local(async move {
            let response = fetch_user_notification(token).await;

            match response {
                Ok(response) => {
                    user_notification.set(UserNotification {
                        is_email_enabled: response.is_email_enabled,
                        is_push_enabled: response.is_push_enabled,
                        is_sms_enabled: response.is_sms_enabled,
                    });
                }
                Err(e) => log!("Error: {}", e.to_string()),
            }
        });
    };

    use_effect_with((), move |_| {
        log!("Ui rendered");
        fetch_notification();
        || {}
    });

    html!(
        <>
        <div class="bg-gradient-to-b from-grey-shade-13 from-20% to-grey-shade-14 to-10% px-2 md:px-0 py-8 ">
            <div class="container mx-auto space-y-6 px-8">
                // Header
                <div class="flex items-center justify-between">
                    <p class="flex items-center text-24 font-600 leading-32">
                        <span class="pr-2">
                        {html! { render_svg!("bx:arrow-back", color="#000000",width="20px")}}
                        </span>
                        {"Settings"}
                    </p>
                    <button  onclick={logout_handle}  class="bg-primary flex items-center rounded p-2 text-grey-shade-14 text-12 font-400 leading-16">
                        <span class="pr-2">
                        {html! { render_svg!("solar:logout-2-bold", color="#ffff",width="14px")}}
                        </span>
                        {"Logout"}
                    </button>
                </div>
                // Profile
                <div class="bg-grey-shade-14 rounded-3xl flex items-center justify-between p-5 shadow-md shadow-grey-shade-0/15 space-x-4">
                    <div>
                        { if !user.clone().image_url.signed_url.is_empty() {
                            html! { <img src={user.image_url.signed_url.clone()} alt="Selected Image" class="w-20 h-20 md:w-22 rounded-full" /> }
                        } else {
                            html! { <img src={"img/circle_profile.png"} alt="Selected Image" class="w-20 md:w-22" /> }
                        }}
                    </div>
                    <div class="flex-1 flex justify-between items-center">
                        <div>
                            <h3>{auth_store.current_user.name.clone()}</h3>
                            <p>{"Username"}</p>
                        </div>
                        <div>
                            <button onclick={edit_modal_handle.clone()} class="bg-grey-shade-0 flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
                                <span class="pr-2">
                                {html! { render_svg!("fe:edit", color="#ffff",width="14px")}}
                                </span>
                                {"Edit"}
                            </button>
                        </div>
                    </div>
                </div>

                // Details
                <div class="flex flex-col md:flex-row space-y-2 md:space-y-0 md:space-x-4">
                    // Login details
                    <div class="bg-grey-shade-12 border border-grey-shade-11 p-5 rounded w-full md:w-1/4">
                        <p class="text-16 font-700 leading-20 text-grey-shade-0">{"Login details"}</p>
                        <div class="space-y-6 pt-3">
                            <div class="space-y-4">
                                <div>
                                    <p class="text-12 font-400 leading-20 text-shade-2">{user.email_address.clone()}</p>
                                    <p class="text-11 font-400 leading-20 text-grey-shade-5">{"Email ID"}</p>
                                </div>
                                    <button onclick={email_update_handle.clone()} class="bg-grey-shade-0 flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
                                        <span class="pr-2">
                                            {html! { render_svg!("fe:edit", color="#ffff",width="14px")}}
                                        </span>
                                        {"Update email ID"}
                                    </button>

                            </div>
                                <div class="space-y-4">
                                    <div>
                                        <p class="text-12 font-400 leading-20 text-shade-2">{"*********"}</p>
                                        <p class="text-11 font-400 leading-20 text-grey-shade-5">{"Password"}</p>
                                    </div>
                                    <button onclick={reset_password_handle.clone()} class="bg-grey-shade-0 flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
                                        <span class="pr-2">
                                            {html! { render_svg!("fe:edit", color="#ffff",width="14px")}}
                                        </span>
                                        {"Update password"}
                                    </button>

                                </div>

                            <div>
                                <p class="text-12 font-400 leading-20 text-shade-2">{"12-10-2023 | 12:00 AM"}</p>
                                <p class="text-11 font-400 leading-20 text-grey-shade-5">{"Last active date/time"}</p>
                            </div>
                        </div>
                    </div>

                    // Notification and remove
                    <div class=" w-full space-y-6">
                        //    Notifications
                        <div class="bg-grey-shade-14 border border-grey-shade-11 p-5 rounded">
                            <p class="text-16 font-700 leading-20 text-grey-shade-0">{"Notifications"}</p>
                            {
                                list_notification.clone().iter().map(|notification| {
                                    html!{
                                        <div class="flex items-center justify-between py-4">
                                        <p>{notification.clone().label}</p>
                                        <div
                                            class="relative inline-block w-10 mr-2 ml-6  align-middle select-none transition duration-200 ease-in"
                                        >
                                            <input
                                                type="checkbox"
                                                name={notification.clone().name}
                                                id={notification.clone().name}
                                                checked={notification.clone().value}
                                                onchange={notification_changed.clone()}
                                                class="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer"
                                            />
                                            <label
                                                for={notification.clone().name}
                                                class="toggle-label block overflow-hidden h-6 rounded-full bg-gray-300 cursor-pointer"
                                            ></label>
                                        </div>
                                    </div>
                                    }
                                }).collect::<Html>()
                            }
                       </div>
                       <div class="bg-grey-shade-14 border border-grey-shade-11 p-5 rounded space-y-2">
                            <p class="text-16 font-700 leading-20 text-grey-shade-0">{"Delete account"}</p>
                            <button onclick={delete_modal_handle.clone()} class="bg-grey-shade-0 flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
                                <span class="pr-2">
                                    {html! { render_svg!("bxs:trash", color="#ffff",width="14px")}}
                                </span>
                                {"Delete"}
                            </button>
                       </div>
                    </div>
                </div>
            </div>
        </div>
        { if *is_open {html! {<EditModal edit_modal_handle={edit_modal_handle.clone()} on_username_input={on_username_input} user={(*user).clone()} update_user={update_user.clone()} image_s3={image_s3.clone()} />}  } else { html! ("") } }
        { if *is_delete_open {
            html! {
                    <DeleteModal
                        delete_modal_handle={delete_modal_handle.clone()}
                        on_ok_response={on_ok_delete_handle.clone()}
                    />
                }
            } else {
                html! {}
            }
        }
        { if *is_email_update {
            html! {
                    <EmailUpdate
                        email_update_handle={email_update_handle.clone()}
                        api_me_fn={api_me_fn.clone()}
                        on_ok_response={on_ok_email_handle.clone()}
                    />
                }
            } else {
                html! {}
            }
        }
        { if *is_reset_password {
            html! {
                    <ResetPasswordModal
                        reset_password_handle={reset_password_handle.clone()}
                        api_me_fn={api_me_fn.clone()}
                        on_ok_response={on_ok_password_handle.clone()}
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
struct EditModalProps {
    edit_modal_handle: Callback<MouseEvent>,
    on_username_input: Callback<InputEvent>,
    update_user: Callback<MouseEvent>,
    user: CurrentUser,
    image_s3: UseStateHandle<String>,
}

#[function_component(EditModal)]
fn edit_modal(props: &EditModalProps) -> Html {
    let image_url = use_state(|| Option::<Blob>::default());

    let file_handle = {
        let image_url = image_url.clone();
        let image_s3 = props.image_s3.clone();
        Callback::from(move |event: Event| {
            // let files = event
            //     .target()
            //     .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
            //     .and_then(|input| input.files());
            let files = event.target().unwrap().unchecked_into::<HtmlInputElement>().files();
            let image_url = image_url.clone();
            let image_s3 = image_s3.clone();
            image_s3.set("".to_string());
            if let Some(files) = files {
                if let Some(file) = files.get(0) {
                    let file_name = file.name();
                    let file_type = file.type_();
                    let file_type_str = file_type.as_str();

                    // Create a Blob from the Uint8Array
                    let blob = Blob::new_with_u8_array_sequence_and_options(
                        &js_sys::Array::of1(&JsValue::from(file.clone())),
                        BlobPropertyBag::new().type_(file_type_str)
                    ).expect("Failed to create Blob");

                    image_url.set(Some(blob));
                    let image_s3 = image_s3.clone();
                    let file = file.clone();
                    spawn_local(async move {
                        let response = upload_file(Media {
                            mime_type: file_type,
                            relative_path: file_name,
                        }).await;

                        match response {
                            Ok(s3) => {
                                let file = file.clone();
                                image_s3.set(s3.s3_key);
                                spawn_local(async move {
                                    let _ = save_file_url(s3.presigned_url, file).await;
                                });
                            }
                            Err(e) => log!("Error: {}", e.to_string()),
                        }
                    });
                } else {
                    log!("No file selected");
                    // Handle the case where no file is selected
                }
            }
        })
    };

    let image_s3 = props.image_s3.clone();

    let remove_handle = {
        let image_s3 = image_s3.clone();
        Callback::from(move |_| {
            image_s3.set("1".to_string());
        })
    };

    let cloned_image_s3 = image_s3.clone();
    let cloned_user = props.user.clone();
    use_effect_with((), move |_| {
        if !cloned_user.image_url.signed_url.is_empty() {
            cloned_image_s3.set("".to_string());
        }
        || {}
    });

    html! {
        <div class="z-10 fixed inset-0 bg-grey-shade-0/70 w-screen flex h-screen items-center justify-center px-4 md:px-0">
            <div class=" bg-white px-2 py-4 md:p-7 rounded-sm space-y-7">
                <div class="flex items-center justify-between space-x-4 md:space-x-0">
                    <p>{"Edit personal information"}</p>
                    <span class="cursor-pointer" onclick={props.edit_modal_handle.clone()}>
                        {html! {render_svg!("mdi:multiply",color="#232323",width="25px")}}
                    </span>
                </div>
                <div class="flex space-x-4 bg-grey-shade-13 items-center justify-start p-6 rounded-sm flex-col md:flex-row space-y-2 md:space-y-0">
                    { 
                        if (*props.image_s3).clone() == "1".to_string() {
                            html! { <img src={"img/circle_profile.png"} alt="Selected Image" class="w-14 md:w-22" /> }
                        } else if let Some(ref blob) = (*image_url).clone() {
                            let data_url = Url::create_object_url_with_blob(blob).expect("Failed to create URL");
    
                            html! { <img src={data_url} alt="Selected Image" class="w-14 h-14 md:w-22 rounded-full" /> }
                        } else if !props.user.image_url.signed_url.is_empty() {
                            html! { <img src={props.user.image_url.signed_url.clone()} alt="Selected Image" class="w-14 h-14 md:w-22 rounded-full" /> }
                        } else {
                            html! { <img src={"img/circle_profile.png"} alt="Selected Image" class="w-14 md:w-22" /> }
                        }
                    }
                    <label for="fileInput" class="bg-grey-shade-14 flex items-center rounded p-2 text-primary text-12 font-400 leading-16 cursor-pointer">
                        {"Replace Image"}
                    </label>
                  
                    <input type="file" id="fileInput" onchange={file_handle} class="hidden" />

                    <button onclick={remove_handle} class="bg-grey-shade-14 flex items-center rounded p-2 text-primary text-12 font-400 leading-16">
                        {"Remove image"}
                    </button>
                </div>
                <div class="flex flex-col space-y-1.5 w-full md:w-[600px]">
                    <label
                        for="username"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"Username"}
                    </label>
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start px-2" >
                        <span>{html! { render_svg!("mdi:user", color="#949494" , width="18px")}} </span>
                        <input
                            id="username"
                            name="username"
                            placeholder="Username"
                            oninput={props.on_username_input.clone()}
                            value={props.user.name.clone()}
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
                <div class="flex space-x-4  items-center justify-start p-0 rounded-sm">
                    <button type="button" onclick={props.update_user.clone()} class="bg-primary flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
                        {"Save"}
                    </button>
                    <button class="bg-grey-shade-14 flex items-center rounded p-2 text-primary text-12 font-400" onclick={props.edit_modal_handle.clone()}>
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
    delete_modal_handle: Callback<MouseEvent>,
}

#[function_component(DeleteModal)]
fn delete_modal(props: &DeleteModalProps) -> Html {
    let (auth_store, auth_dispatch) = use_store::<AuthStore>();

    let token = auth_store.token.clone();
    let current_user = auth_store.current_user.clone();

    let delete_user_handler = {
        let on_ok = props.on_ok_response.clone();
        let current_user = current_user.clone();
        let cloned_token = token.clone();
        let store_dispatch = auth_dispatch.clone();
        Callback::from(move |_event: MouseEvent| {
            let on_ok = on_ok.clone();
            let current_user = current_user.clone();
            let cloned_token = cloned_token.clone();
            let store_dispatch = store_dispatch.clone();
            spawn_local(async move {
                if !current_user.id.clone().is_empty() {
                    let response = api_delete_user(cloned_token, current_user.id).await;

                    match response {
                        Ok(_response) => {
                            on_ok.emit(());
                            store_dispatch.reduce_mut(move |store| store.reset_to_default());
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

#[derive(Properties, PartialEq)]
struct EmailUpdateProps {
    #[prop_or_default]
    email_update_handle: Callback<MouseEvent>,
    api_me_fn: Callback<()>,
    on_ok_response: Callback<()>,
}

#[function_component(EmailUpdate)]
fn email_update_modal(props: &EmailUpdateProps) -> Html {
    let (auth_store, _) = use_store::<AuthStore>();
    let email_address = use_state(|| String::default());

    let cloned_state = email_address.clone();
    let state_changed = Callback::from(move |event: Event| {
        let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        cloned_state.set(value);
    });

    let token = auth_store.token.clone();

    let update_email_handler = {
        let cloned_email_address = email_address.clone();
        let on_ok = props.on_ok_response.clone();
        let handle_api_me = props.api_me_fn.clone();
        let cloned_token = token.clone();
        Callback::from(move |_event: MouseEvent| {
            let cloned_token = cloned_token.clone();
            let on_ok = on_ok.clone();
            let cloned_email_address = (*cloned_email_address).clone();
            let handle_api_me = handle_api_me.clone();
            spawn_local(async move {
                let response = update_email(cloned_token, cloned_email_address).await;

                match response {
                    Ok(_response) => {
                        handle_api_me.emit(());
                        on_ok.emit(());
                    }
                    Err(e) => log!("Error: ", e.to_string()),
                }
            });
        })
    };

    html! {
        <div class="z-10 fixed inset-0 bg-grey-shade-0/70 w-screen flex h-screen items-center justify-center p-5">
            <div class=" bg-white p-7 rounded-sm space-y-7">
                <div class="flex items-center justify-between">
                    <p>{"Update Email address"}</p>
                    <span class="cursor-pointer" onclick={props.email_update_handle.clone()}>
                        {html! {render_svg!("mdi:multiply",color="#232323",width="25px")}}
                    </span>
                </div>
                <div class="flex flex-col space-y-1.5 w-full md:w-[600px]">
                    <label
                        for="emailaddress"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"New email-address"}
                    </label>
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start px-2" >
                        <span>{html! { render_svg!("mdi:user", color="#949494" , width="18px")}} </span>
                        <input
                            id="emailaddress"
                            name="emailaddress"
                            placeholder="Email Address"
                            onchange={state_changed.clone()}
                            class="px-3.5 py-3placeholder:text-grey-shade-6 text-14 leading-20
                            bg-white
                            h-10
                            w-full  
                            md:w-48
                            border-grey-shade-11
                            font-300 font-sans outline-none
                            pr-2 pl-2"
                        />
                    </div>
                </div>
                <div class="flex space-x-4  items-center justify-start p-0 rounded-sm">
                    <button type="button" onclick={update_email_handler.clone()} class="bg-primary flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
                        {"Do you want to update email ID?"}
                    </button>
                    <button class="bg-grey-shade-14 flex items-center rounded p-2 text-primary text-12 font-400" onclick={props.email_update_handle.clone()}>
                        {"Close"}
                    </button>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ResetPasswordProps {
    #[prop_or_default]
    reset_password_handle: Callback<MouseEvent>,
    api_me_fn: Callback<()>,
    on_ok_response: Callback<()>,
}

#[function_component(ResetPasswordModal)]
fn reset_password_modal(props: &ResetPasswordProps) -> Html {
    let email_address = use_state(|| String::default());

    let cloned_state = email_address.clone();
    let state_changed = Callback::from(move |event: Event| {
        let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        cloned_state.set(value);
    });

    let reset_password_handler = {
        let cloned_email_address = email_address.clone();
        let on_ok = props.on_ok_response.clone();
        // let handle_api_me = props.api_me_fn.clone();
        Callback::from(move |_event: MouseEvent| {
            let on_ok = on_ok.clone();
            let cloned_email_address = (*cloned_email_address).clone();
            // let handle_api_me= handle_api_me.clone();
            spawn_local(async move {
                let response = api_forgot_password(cloned_email_address).await;

                match response {
                    Ok(_response) => {
                        // handle_api_me.emit(());
                        on_ok.emit(());
                    }
                    Err(e) => log!("Error: ", e.to_string()),
                }
            });
        })
    };

    html! {
        <div class="z-10 fixed inset-0 bg-grey-shade-0/70 w-screen flex h-screen items-center justify-center p-5">
            <div class=" bg-white p-7 rounded-sm space-y-7">
                <div class="flex items-center justify-between">
                    <p>{"Reset password?"}</p>
                    <span class="cursor-pointer" onclick={props.reset_password_handle.clone()}>
                        {html! {render_svg!("mdi:multiply",color="#232323",width="25px")}}
                    </span>
                </div>
                <div class="flex flex-col space-y-1.5 w-full md:w-[600px]">
                    <label
                        for="emailaddress"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"Enter registered Email ID"}
                    </label>
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start px-2" >
                        <span>{html! { render_svg!("mdi:user", color="#949494" , width="18px")}} </span>
                        <input
                            id="emailaddress"
                            name="emailaddress"
                            placeholder="Email Address"
                            onchange={state_changed.clone()}
                            class="px-3.5 py-3placeholder:text-grey-shade-6 text-14 leading-20
                            bg-white
                            h-10
                            w-full  
                            md:w-48
                            border-grey-shade-11
                            font-300 font-sans outline-none
                            pr-2 pl-2"
                        />
                    </div>
                </div>
                <div class="flex space-x-4  items-center justify-start p-0 rounded-sm">
                    <button type="button" onclick={reset_password_handler} class="bg-primary flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
                        {"Sent"}
                    </button>
                    <button class="bg-grey-shade-14 flex items-center rounded p-2 text-primary text-12 font-400" onclick={props.reset_password_handle.clone()}>
                        {"Close"}
                    </button>
                </div>
            </div>
        </div>
    }
}
