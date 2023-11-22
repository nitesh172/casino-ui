use yew::prelude::*;

use crate::{
    components::organisms::paginator::Paginator,
    render_svg,
};

#[function_component(Teams)]
pub fn teams() -> Html {
    let t_head = vec!["Username", "Email ID", "Reset password", "Role", "Status", "Actions"];

    #[derive(Clone, Properties, PartialEq)]
    pub struct UserProps {
        pub name: String,
        pub email_address: String,
        pub role: String,
        pub status: String,
    }

    let users = vec![
        UserProps {
            name: "Dianne Russell".into(),
            email_address: "JaneCooper@gmail.com".into(),
            role: "Administrator".into(),
            status: "Active".into(),
        },
        UserProps {
            name: "Dianne Russell".into(),
            email_address: "JaneCooper2@gmail.com".into(),
            role: "Member".into(),
            status: "Inactive".into(),
        }
    ];

    let is_open = use_state(|| false);

    let modal_handle = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };

    html!(
    <>
        <div class="bg-grey-shade-13 py-4 px-8">
            <div class="container mx-auto md:w-auto space-y-4" >
                <div class="flex md:justify-between items-start md:items-center flex-col gap-6 lg:gap-0 md:flex-row">
                    <div class="flex flex-row justify-between lg:justify-normal gap-2 items-center w-full lg:w-auto">
                        <h1>{"Teams"}</h1>
                        <div class="flex items-center rounded border justify-start bg-white border-grey-shade-11 px-2 w-40">
                            <span>{html! { render_svg!("mynaui:search",  color="#000000", width="18px")}} </span>
                            <input
                                id="search"
                                autocomplete="off"
                                name="search"
                                placeholder={"search"}
                                class="px-2.5 py-2 h-7 bg-white placeholder:text-grey-shade-6 text-14  leading-20 font-300 font-sans outline-none pr-2 pl-2 w-full "
                            />
                        </div>
                    </div>
                    <div class="flex flex-row items-center w-full lg:w-auto gap-2">
                        <button
                            class="bg-grey-shade-0 flex flex-1 lg:flex-none items-center justify-center lg:justify-normal rounded p-2 text-grey-shade-14 text-12 font-400 "
                        >
                            <span class="pr-2">
                            {html! { render_svg!("majesticons:file", color="#ffff",width="14px")}}
                            </span>
                            {"Export"}
                        </button>
                        <button
                            onclick={modal_handle.clone()}
                            class="bg-primary flex flex-1 lg:flex-none items-center rounded justify-center lg:justify-normal p-2 text-grey-shade-14 text-12 font-400"
                        >
                            <span class="pr-2">
                            {html! { render_svg!("lets-icons:add-round", color="#ffff",width="14px")}}
                            </span>
                            {"Invite team member"}
                        </button>
                     </div>
                </div>
                <Paginator  />
            </div>
        </div>
        <div class="relative px-8">
            <div class="absolute hidden lg:block -z-10 top-0 left-0 h-[45px] w-full bg-grey-shade-13"></div>
            <div class="container mx-auto">
                <table class="w-full table-auto hidden lg:inline-table">
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
                                        <td class="py-3  text-left text-14 font-medium text-warning tracking-wider flex flex-row gap-0.5">
                                            <span>{html! { render_svg!    ("material-symbols-light:lock-sharp", color="#C53A3A",width="18px")}}</span>
                                            {"Reset password"}
                                        </td>
                                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">
                                            <span class="rounded-full py-1 px-2 flex-row gap-1 bg-grey-shade-11">
                                                {user.clone().role}
                                            </span>
                                        </td>
                                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">
                                            <span class={format!("rounded-full py-1 px-2 flex-row gap-1 text-white {} ", if user.clone().status == "Active" {"bg-success"} else {"bg-warning"})}>
                                                {user.clone().status}
                                            </span>
                                        </td>
                                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider relative group cursor-pointer">
                                            <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                            <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                                <li onclick={modal_handle.clone()} class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">{"Edit"}</li>
                                                <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">{"Delete"}</li>
                                            </ul>
                                        </td>
                                    </tr>
                                }
                            }).collect::<Html>()
                        }
                    </tbody>
                </table>
                <div class="flex lg:hidden flex-col gap-4 mt-8">
                    {
                        users.clone().iter().map(|user| {
                            html!{
                                <div class="rounded-xl border p-4 flex flex-col gap-2.5">
                                    <div class="flex flex-row justify-between">
                                        <div class="flex flex-col gap-2.5">
                                            <div class="rounded-full py-1 px-2 flex-row gap-1 bg-grey-shade-11">
                                                {user.clone().role}
                                            </div>
                                            <div class="flex flex-col">
                                                <div class="font-400">{user.clone().name}</div>
                                                <div class="text-12 text-grey-shade-5">{user.clone().email_address}</div>
                                            </div>
                                        </div>
                                        <div class="text-12 font-medium text-grey-shade-1 relative group cursor-pointer">
                                            <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                            <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                                <li onclick={modal_handle.clone()} class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">{"Edit"}</li>
                                                <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">{"Delete"}</li>
                                            </ul>
                                        </div>
                                    </div>
                                    <div class="flex flex-row justify-between items-center">
                                        <div class="text-12 font-medium text-warning flex flex-row gap-0.5">
                                            <span>{html! { render_svg!    ("material-symbols-light:lock-sharp", color="#C53A3A",width="18px")}}</span>
                                            {"Reset password"}
                                        </div>
                                        <div class={format!("rounded-full py-1 px-2 flex-row gap-1 text-white {} ", if user.clone().status == "Active" {"bg-success"} else {"bg-warning"})}>
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
        { if *is_open {html! {<Modal modal_handle={modal_handle.clone()}  />}  } else { html! ("") } }
    </>
    )
}


#[derive(Properties, PartialEq)]
struct ModalProps {
    modal_handle: Callback<MouseEvent>,
}

#[function_component(Modal)]
fn edit_modal(props: &ModalProps) -> Html {
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
                        for="username"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"Username"}
                    </label>
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start px-2" >
                        <input
                            id="username"
                            name="username"
                            placeholder="Name"
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

                <div class="flex flex-col space-y-1.5 w-full md:w-[400px]">
                    <label
                        for="emailID"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"Email ID"}
                    </label>
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start px-2" >
                        <input
                            id="emailID"
                            name="emailID"
                            placeholder="Email ID"
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

                <div class="flex flex-col space-y-1.5 md:w-[400px]">
                    <label
                        for="role"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"Role"}
                    </label>
                    <div class="relative inline-flex w-full ">
                        <select  id="role" class="appearance-none border border-gray-300 rounded px-3 py-2 focus:outline-none w-full ">
                            <option>{"Select"}</option>
                            <option>{"Admin"}</option>
                            <option>{"Member"}</option>
                        </select>
                        <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
                            <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 11.586l3.293-3.293a1 1 0 011.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
                            </svg>
                        </div>
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
