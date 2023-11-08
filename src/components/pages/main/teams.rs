use crate::render_svg;
use yew::prelude::*;

#[function_component(Teams)]
pub fn teams() -> Html {
    let is_open = use_state(|| true);

    let edit_modal_handle = {
        let is_open = is_open.clone();

        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };

    html!(
        <>
        <div class="bg-gradient-to-b from-grey-shade-13 from-20% to-grey-shade-14 to-10% py-8 ">
            <div class="container mx-auto space-y-6">
                // Header
                <div class="flex items-center justify-between">
                    <p class="flex items-center text-24 font-600 leading-32">
                        <span class="pr-2">
                        {html! { render_svg!("bx:arrow-back", color="#000000",width="20px")}}
                        </span>
                        {"Settings"}
                    </p>
                    <button class="bg-primary flex items-center rounded p-2 text-grey-shade-14 text-12 font-400 leading-16">
                        <span class="pr-2">
                        {html! { render_svg!("solar:logout-2-bold", color="#ffff",width="14px")}}
                        </span>
                        {"Logout"}
                    </button>
                </div>
                // Profile
                <div class="bg-grey-shade-14 rounded-3xl flex items-center justify-between p-5 shadow-md shadow-grey-shade-0/15 space-x-4">
                    <div>
                        <img src="img/circle_profile.png" class="w-22" />
                    </div>
                    <div class="flex-1 flex justify-between items-center">
                        <div>
                            <h3>{"Beckham Andy"}</h3>
                            <p>{"username"}</p>
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
                <div class="flex flex-col md:flex-row space-x-4">
                    // Login details
                    <div class="bg-grey-shade-12 border border-grey-shade-11 p-5 rounded w-1/4">
                        <p class="text-16 font-700 leading-20 text-grey-shade-0">{"Login details"}</p>
                        <div class="space-y-6 pt-3">
                            <div class="space-y-4">
                                <div>
                                    <p class="text-12 font-400 leading-20 text-shade-2">{"JaneCooper@gmail.com"}</p>
                                    <p class="text-11 font-400 leading-20 text-grey-shade-5">{"Email ID"}</p>
                                </div>
                                    <button class="bg-grey-shade-0 flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
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
                                    <button class="bg-grey-shade-0 flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
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
                            <div class="flex items-center justify-between py-4">
                                <p>{"Push notifications"}</p>
                                <div
                                class="relative inline-block w-10 mr-2 ml-6  align-middle select-none transition duration-200 ease-in"
                              >
                                <input
                                  type="checkbox"
                                  name="toggle"
                                  id="toggle"
                                  class="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer"
                                />
                                <label
                                  for="toggle"
                                  class="toggle-label block overflow-hidden h-6 rounded-full bg-gray-300 cursor-pointer"
                                ></label>
                              </div>
                            </div>
                            <div class="flex items-center justify-between py-4">
                                <p>{"Email notifications"}</p>
                                <div
                                class="relative inline-block w-10 mr-2 ml-6  align-middle select-none transition duration-200 ease-in"
                              >
                                <input
                                  type="checkbox"
                                  name="toggle"
                                  id="toggle"
                                  class="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer"
                                />
                                <label
                                  for="toggle"
                                  class="toggle-label block overflow-hidden h-6 rounded-full bg-gray-300 cursor-pointer"
                                ></label>
                              </div>
                            </div>
                            <div class="flex items-center justify-between py-4">
                                <p>{"SMS notifications"}</p>
                                <div
                                class="relative inline-block w-10 mr-2 ml-6  align-middle select-none transition duration-200 ease-in"
                              >
                                <input
                                  type="checkbox"
                                  name="toggle"
                                  id="toggle"
                                  class="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer"
                                />
                                <label
                                  for="toggle"
                                  class="toggle-label block overflow-hidden h-6 rounded-full bg-gray-300 cursor-pointer"
                                ></label>
                              </div>
                            </div>

                       </div>
                       <div class="bg-grey-shade-14 border border-grey-shade-11 p-5 rounded space-y-2">
                            <p class="text-16 font-700 leading-20 text-grey-shade-0">{"Delet account"}</p>
                            <button class="bg-grey-shade-0 flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
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
        { if *is_open {html! {<EditModal edit_modal_handle={edit_modal_handle.clone()}/>}  } else { html! ("") } }
        </>
    )
}

#[derive(Properties, PartialEq)]
struct EditModalProps {
    edit_modal_handle: Callback<MouseEvent>,
}

#[function_component(EditModal)]
fn edit_modal(props: &EditModalProps) -> Html {
    html! {
        <div class="z-10 fixed inset-0 bg-grey-shade-0/70 w-screen flex h-screen items-center justify-center">
            <div class=" bg-white p-7 rounded-sm space-y-7">
                <div class="flex items-center justify-between">
                    <p>{"Edit personal information"}</p>
                    <span class="cursor-pointer" onclick={props.edit_modal_handle.clone()}>
                        {html! {render_svg!("mdi:multiply",color="#232323",width="25px")}}
                    </span>
                </div>
                <div class="flex space-x-4 bg-grey-shade-13 items-center justify-start p-6 rounded-sm">
                    <img src="img/circle_profile.png" class="w-22" />
                    <button class="bg-grey-shade-14 flex items-center rounded p-2 text-primary text-12 font-400 leading-16">
                        <span class="pr-2">
                        {html! { render_svg!("solar:logout-2-bold", color="#ffff",width="14px")}}
                        </span>
                        {"Replace image"}
                    </button>
                    <button class="bg-grey-shade-14 flex items-center rounded p-2 text-primary text-12 font-400 leading-16">
                        <span class="pr-2">
                        {html! { render_svg!("solar:logout-2-bold", color="#ffff",width="14px")}}
                        </span>
                        {"Remove image"}
                    </button>
                </div>
                <div class="flex flex-col space-y-1.5 md:w-[600px]">
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
                            class="px-3.5 py-3placeholder:text-grey-shade-6 text-14 leading-20
                            bg-white
                            h-10 
                            w-72
                            border-grey-shade-11
                            font-300 font-sans outline-none
                            pr-2 pl-2"
                        />
                    </div>
                </div>
                <div class="flex space-x-4  items-center justify-start p-0 rounded-sm">
                    <button class="bg-primary flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
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
