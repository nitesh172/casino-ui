use yew::prelude::*;
use std::ops::Deref;
use crate::{
    components::organisms::paginator::{PaginationDataProps, PaginationFucProps, Paginator},
    render_svg,
};

#[function_component(Participants)]
pub fn participants() -> Html {
    let t_head = vec![
        "ID",
        "Username",
        "Game name",
        "Bet amount",
        "Won/lost amount",
        "Total amount",
        "Email ID",
        "Last active date/time",
        "Status",
        "Actions"
    ];

    #[derive(Clone, Properties, PartialEq)]
    pub struct UserProps {
        pub name: String,
        pub id: String,
        pub game_name: String,
        pub bet_amount: String,
        pub won_lost_amount: String,
        pub total_amount: String,
        pub email_address: String,
        pub last_active: String,
        pub status: String,
    }

    let users = vec![
        UserProps {
            id: "WW20011".into(),
            name: "Dianne Russell".into(),
            game_name: "Roulet".into(),
            bet_amount: "25.50".into(),
            won_lost_amount: "25.50".into(),
            total_amount: "51.00".into(),
            email_address: "JaneCooper@gmail.com".into(),
            last_active: "12-10-2023 | 12:00 AM".into(),
            status: "Winner".into(),
        },
        UserProps {
            id: "WW20011".into(),
            name: "Dianne Russell".into(),
            game_name: "Roulet".into(),
            bet_amount: "25.50".into(),
            won_lost_amount: "25.50".into(),
            total_amount: "-".into(),
            email_address: "JaneCooper2@gmail.com".into(),
            last_active: "12-10-2023 | 12:00 AM".into(),
            status: "Looser".into(),
        }
    ];

    let initial = use_state(|| true);
    let pagination = use_state(|| PaginationDataProps {
        per_page: 10,
        total_items: 0,
        total_pages: 0,
        current_page: 1,
    });
    let is_open = use_state(|| false);

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

    html!(
        <>
            <div class="bg-grey-shade-13 py-4 px-8">
                <div class="container mx-auto md:w-auto space-y-4" >
                    <div class="flex md:justify-between items-start md:items-center flex-col gap-6 md:flex-row">
                        <div class="flex flex-row justify-between lg:justify-normal gap-2 items-center w-full">
                            <h1>{"Participate"}</h1>
                            <div class="hidden lg:flex items-center rounded border justify-start border-grey-shade-11 bg-white px-2 w-40">
                                <span>{html! { render_svg!("mynaui:search",  color="#000000", width="18px")}} </span>
                                <input
                                    id="search"
                                    autocomplete="off"
                                    name="search"
                                    placeholder={"search"}
                                    class="px-2.5 py-2 h-7 bg-white placeholder:text-grey-shade-6 text-14  leading-20 font-300 font-sans outline-none pr-2 pl-2 w-full "
                                />
                            </div>
                            <select class="border lg:hidden w-[180px] border-gray-300 flex-col-reverse rounded px-2 focus:outline-none">
                                <option>{"Evolution"}</option>
                                <option>{"Ezugi"}</option>
                                <option>{"Qtech"}</option>
                                <option>{"All platforms"}</option>
                            </select>
                        </div>
                        <div class="flex flex-col lg:flex-row w-full md:w-fit gap-2 items-center">
                            <select class="border hidden lg:block border-gray-300 flex-col-reverse rounded px-2 py-1 focus:outline-none w-[250px] ">
                                <option>{"Evolution"}</option>
                                <option>{"Ezugi"}</option>
                                <option>{"Qtech"}</option>
                                <option>{"All platforms"}</option>
                            </select>
                            <button
                                class="bg-grey-shade-0 w-full lg:w-fit flex flex-row justify-center items-center rounded p-2 text-grey-shade-14 text-12 font-400 "
                            >
                                <span class="pr-2">
                                {html! { render_svg!("majesticons:file", color="#ffff",width="14px")}}
                                </span>
                                {"Export"}
                            </button>
                        </div>
                    </div>
                </div>
            </div>
            <div class="relative w-full">
                <div class="container mx-auto p-8 w-full">
                    <div class="flex flex-col lg:flex-row justify-between mb-3">
                        <div class="font-600">{"EVOLUTION"}</div>
                        <div class="flex flex-col md:flex-row gap-4 mt-4 lg:mt-0 mb-4 lg:mb-0">
                            <select class="border border-gray-300 flex-col-reverse rounded px-2 py-1 focus:outline-none w-[250px] ">
                                <option>{"Roulet"}</option>
                            </select>
                            <select class="border border-gray-300 flex-col-reverse rounded px-2 py-1 focus:outline-none w-[250px] ">
                                <option>{"Current game"}</option>
                            </select>
                        </div>
                    </div>
                    <ContextProvider<PaginationDataProps> context={(*pagination).clone()}>
                        <Paginator update_pagination={update_pagination.clone()} />
                    </ContextProvider<PaginationDataProps>>
                    <table class="w-full table-auto mt-3 hidden xl:inline-table">
                        <thead>
                            <tr class="">
                                {
                                    t_head.clone().into_iter().map(|name| {
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
                                            <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{user.clone().id}</td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1">
                                                <div class="flex flex-row items-center gap-2 w-full">
                                                    <img src="img/circle_profile.png" class="w-8 h-8" alt=""/>
                                                    <span>
                                                        {user.clone().name}
                                                    </span>
                                                </div>
                                            </td>
                                            <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{user.clone().game_name}</td>
                                            <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{format!("{}{}", "$", user.clone().bet_amount)}</td>
                                            <td class={format!("py-3  text-left text-14 font-medium tracking-wider {} ", if user.clone().status == "Winner"{ " text-success" } else { "text-warning" })}>
                                                {format!("{}{}", "$", user.clone().won_lost_amount)}</td>
                                            <td class={format!("py-3  text-left text-14 font-medium tracking-wider {} ", if user.clone().status == "Winner"{ " text-success" } else { "text-warning" })}>
                                                {if user.clone().status == "Winner" {format!("{}{}", "$", user.clone().total_amount)} else {format!("{}", "-")}}</td>
                                            <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{user.clone().email_address}</td>
                                            <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider flex flex-row gap-0.5">
                                                <span>{html! { render_svg!    ("uis:calender", color="#000000",width="18px")}}</span>
                                                {user.clone().last_active}
                                            </td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">
                                                <span class={format!("rounded-full py-1 px-2 flex-row gap-1 {} ", if user.clone().status == "Winner"{ " bg-info text-white" } else { "bg-grey-shade-11" })}>
                                                    {user.clone().status}
                                                </span>
                                            </td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider relative group cursor-pointer">
                                                <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                                <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                                    <li onclick={modal_handle.clone()} class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">
                                                        
                                                        { "View" }
                                                    </li>
                                                    <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">
                                                        
                                                        { "Ban" }
                                                    </li>
                                                </ul>
                                            </td>
                                        </tr>
                                    }
                                }).collect::<Html>()
                            }
                        </tbody>
                    </table>
                    <div class="grid md:grid-cols-2 lg:grid-cols-2 xl:hidden gap-4 mt-4">
                        {
                            users.clone().iter().map(|user| {
                                html!{<div class="border p-4 flex flex-col rounded-md gap-2.5">
                                    <div class="flex flex-row justify-between items-center">
                                        <div class="flex flex-col gap-2.5">
                                            <div class="text-14 font-medium text-grey-shade-1">{user.clone().id}</div>
                                            <div class="flex flex-row gap-1.5 items-center">
                                                <img src="img/circle_profile.png" class="w-8 h-8" alt=""/>
                                                <div class="flex flex-col gap-1">
                                                    <div class="text-grey-shade-5 text-12">{user.clone().name}</div>
                                                    <div class="text-14">{user.clone().email_address}</div>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="text-12 font-medium text-grey-shade-1 relative group cursor-pointer">
                                            <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                            <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                                <li onclick={modal_handle.clone()} class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">
                                                    
                                                    { "View" }
                                                </li>
                                                <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">
                                                    
                                                    { "Ban" }
                                                </li>
                                            </ul>
                                        </div>
                                    </div>
                                    <div class="flex flex-row">
                                        <div class="flex flex-col gap-1 p-1">
                                            <div class="text-grey-shade-5 text-12">{"Game name"}</div>
                                            <div class="text-14">{user.clone().game_name}</div>
                                        </div>
                                        <div class="flex flex-col gap-1 p-1">
                                            <div class="text-grey-shade-5 text-12">{"Bet amount"}</div>
                                            <div class="text-14">{format!("{}{}", "$", user.clone().bet_amount)}</div>
                                        </div>
                                        <div class="flex flex-col gap-1 p-1">
                                            <div class="text-grey-shade-5 text-12">{"Won/lost amount"}</div>
                                            <div class={format!("text-14 font-medium {} ", if user.clone().status == "Winner"{ " text-success" } else { "text-warning" })}>
                                                {format!("{}{}", "$", user.clone().won_lost_amount)}
                                            </div>
                                        </div>
                                        <div class="flex flex-col gap-1 p-1">
                                            <div class="text-grey-shade-5 text-12">{"Total amount"}</div>
                                            <div class={format!("text-14 font-medium {} ", if user.clone().status == "Winner"{ " text-success" } else { "text-warning" })}>
                                                {if user.clone().status == "Winner" {format!("{}{}", "$", user.clone().total_amount)} else {format!("{}", "-")}}
                                            </div>
                                        </div>
                                    </div>
                                    <div class="flex flex-row justify-between items-center">
                                        <div class={format!("rounded-full py-1 px-2 flex-row gap-1 {} ", if user.clone().status == "Winner"{ " bg-info text-white" } else { "bg-grey-shade-11" })}>
                                            {user.clone().status}
                                        </div>
                                        <div class="text-12 font-medium text-grey-shade-1 flex flex-row gap-0.5">
                                            <span>{html! { render_svg!    ("uis:calender", color="#000000",width="18px")}}</span>
                                            {user.clone().last_active}
                                        </div>
                                    </div>
                                </div>}
                            }).collect::<Html>()
                        }
                    </div>
                </div>
                <div class="container mx-auto p-8 w-full">
                    <div class="flex flex-col lg:flex-row justify-between mb-3">
                        <div class="font-600">{"EZUGI"}</div>
                        <div class="flex flex-col md:flex-row gap-4 mt-4 lg:mt-0 mb-4 lg:mb-0">
                            <select class="border border-gray-300 flex-col-reverse rounded px-2 py-1 focus:outline-none w-[250px] ">
                                <option>{"Roulet"}</option>
                            </select>
                            <select class="border border-gray-300 flex-col-reverse rounded px-2 py-1 focus:outline-none w-[250px] ">
                                <option>{"Current game"}</option>
                            </select>
                        </div>
                    </div>
                    <ContextProvider<PaginationDataProps> context={(*pagination).clone()}>
                        <Paginator update_pagination={update_pagination.clone()} />
                    </ContextProvider<PaginationDataProps>>
                    <table class="w-full table-auto mt-3 hidden xl:inline-table">
                        <thead>
                            <tr class="">
                                {
                                    t_head.clone().into_iter().map(|name| {
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
                                            <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{user.clone().id}</td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1">
                                                <div class="flex flex-row items-center gap-2 w-full">
                                                    <img src="img/circle_profile.png" class="w-8 h-8" alt=""/>
                                                    <span>
                                                        {user.clone().name}
                                                    </span>
                                                </div>
                                            </td>
                                            <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{user.clone().game_name}</td>
                                            <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{format!("{}{}", "$", user.clone().bet_amount)}</td>
                                            <td class={format!("py-3  text-left text-14 font-medium tracking-wider {} ", if user.clone().status == "Winner"{ " text-success" } else { "text-warning" })}>
                                                {format!("{}{}", "$", user.clone().won_lost_amount)}</td>
                                            <td class={format!("py-3  text-left text-14 font-medium tracking-wider {} ", if user.clone().status == "Winner"{ " text-success" } else { "text-warning" })}>
                                                {if user.clone().status == "Winner" {format!("{}{}", "$", user.clone().total_amount)} else {format!("{}", "-")}}</td>
                                            <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{user.clone().email_address}</td>
                                            <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider flex flex-row gap-0.5">
                                                <span>{html! { render_svg!    ("uis:calender", color="#000000",width="18px")}}</span>
                                                {user.clone().last_active}
                                            </td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">
                                                <span class={format!("rounded-full py-1 px-2 flex-row gap-1 {} ", if user.clone().status == "Winner"{ " bg-info text-white" } else { "bg-grey-shade-11" })}>
                                                    {user.clone().status}
                                                </span>
                                            </td>
                                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider relative group cursor-pointer">
                                                <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                                <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                                    <li onclick={modal_handle.clone()} class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">
                                                        
                                                        { "View" }
                                                    </li>
                                                    <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">
                                                        
                                                        { "Ban" }
                                                    </li>
                                                </ul>
                                            </td>
                                        </tr>
                                    }
                                }).collect::<Html>()
                            }
                        </tbody>
                    </table>
                    <div class="grid md:grid-cols-2 lg:grid-cols-2 xl:hidden gap-4 mt-4">
                        {
                            users.clone().iter().map(|user| {
                                html!{<div class="border p-4 flex flex-col rounded-md gap-2.5">
                                    <div class="flex flex-row justify-between items-center">
                                        <div class="flex flex-col gap-2.5">
                                            <div class="text-14 font-medium text-grey-shade-1">{user.clone().id}</div>
                                            <div class="flex flex-row gap-1.5 items-center">
                                                <img src="img/circle_profile.png" class="w-8 h-8" alt=""/>
                                                <div class="flex flex-col gap-1">
                                                    <div class="text-grey-shade-5 text-12">{user.clone().name}</div>
                                                    <div class="text-14">{user.clone().email_address}</div>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="text-12 font-medium text-grey-shade-1 relative group cursor-pointer">
                                            <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                            <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                                <li onclick={modal_handle.clone()} class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">
                                                    
                                                    { "View" }
                                                </li>
                                                <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">
                                                    
                                                    { "Ban" }
                                                </li>
                                            </ul>
                                        </div>
                                    </div>
                                    <div class="flex flex-row">
                                        <div class="flex flex-col gap-1 p-1">
                                            <div class="text-grey-shade-5 text-12">{"Game name"}</div>
                                            <div class="text-14">{user.clone().game_name}</div>
                                        </div>
                                        <div class="flex flex-col gap-1 p-1">
                                            <div class="text-grey-shade-5 text-12">{"Bet amount"}</div>
                                            <div class="text-14">{format!("{}{}", "$", user.clone().bet_amount)}</div>
                                        </div>
                                        <div class="flex flex-col gap-1 p-1">
                                            <div class="text-grey-shade-5 text-12">{"Won/lost amount"}</div>
                                            <div class={format!("text-14 font-medium {} ", if user.clone().status == "Winner"{ " text-success" } else { "text-warning" })}>
                                                {format!("{}{}", "$", user.clone().won_lost_amount)}
                                            </div>
                                        </div>
                                        <div class="flex flex-col gap-1 p-1">
                                            <div class="text-grey-shade-5 text-12">{"Total amount"}</div>
                                            <div class={format!("text-14 font-medium {} ", if user.clone().status == "Winner"{ " text-success" } else { "text-warning" })}>
                                                {if user.clone().status == "Winner" {format!("{}{}", "$", user.clone().total_amount)} else {format!("{}", "-")}}
                                            </div>
                                        </div>
                                    </div>
                                    <div class="flex flex-row justify-between items-center">
                                        <div class={format!("rounded-full py-1 px-2 flex-row gap-1 {} ", if user.clone().status == "Winner"{ " bg-info text-white" } else { "bg-grey-shade-11" })}>
                                            {user.clone().status}
                                        </div>
                                        <div class="text-12 font-medium text-grey-shade-1 flex flex-row gap-0.5">
                                            <span>{html! { render_svg!    ("uis:calender", color="#000000",width="18px")}}</span>
                                            {user.clone().last_active}
                                        </div>
                                    </div>
                                </div>}
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
                <div class="flex flex-col md:flex-row items-center gap-8">
                    <img src="img/circle_profile.png" class="h-20" alt=""/>
                    <div class="flex flex-col gap-6 w-full flex-wrap">
                        <div class="flex flex-row gap-12">
                            <div class="flex flex-col">
                                <div class="font-400">{"Beckham Andy"}</div>
                                <div class="text-12 text-grey-shade-5">{"Username"}</div>
                            </div>
                            <div class="flex flex-col">
                                <div class="font-400">{32}</div>
                                <div class="text-12 text-grey-shade-5">{"Age"}</div>
                            </div>
                            <div class="flex flex-col">
                                <div class="font-400">{"Male"}</div>
                                <div class="text-12 text-grey-shade-5">{"Gender"}</div>
                            </div>
                        </div>
                        <div class="flex flex-col">
                            <div class="font-400">{"23, Brook St, Dubai, UAE"}</div>
                            <div class="text-12 text-grey-shade-5">{"Address"}</div>
                        </div>
                    </div>
                </div>
                <div class="flex flex-row items-center p-4 rounded-xl flex-wrap justify-between lg:justify-normal gap-3 bg-grey-shade-13">
                    <div class="flex flex-col w-20">
                        <div class="font-400">{12}</div>
                        <div class="text-12 text-grey-shade-5">{"Total games"}</div>
                    </div>
                    <div class="flex flex-col w-16">
                        <div class="font-400">{10}</div>
                        <div class="text-12 text-grey-shade-5">{"Won"}</div>
                    </div>
                    <div class="flex flex-col w-16">
                        <div class="font-400">{10}</div>
                        <div class="text-12 text-grey-shade-5">{"Lost"}</div>
                    </div>
                    <div class="flex flex-col w-16">
                        <div class="font-400">{"$245"}</div>
                        <div class="text-12 text-grey-shade-5">{"Tot bet"}</div>
                    </div>
                    <div class="flex flex-col w-16">
                        <div class="font-400">{"$792"}</div>
                        <div class="text-12 text-grey-shade-5">{"Won"}</div>
                    </div>
                    <div class="flex flex-col w-16">
                        <div class="font-400">{"$67"}</div>
                        <div class="text-12 text-grey-shade-5">{"Lost"}</div>
                    </div>
                </div>
                <div class="flex flex-col md:flex-row gap-8 lg:gap-0">
                    <div class="flex-1 flex flex-col gap-3">
                        <div class="text-grey-shade-1">{"Banking details"}</div>
                        <div class="flex flex-col gap-4">
                            <div class="flex flex-col gap-1.5">
                                <div class="text-lg">{"Beckham ande"}</div>
                                <div class="text-grey-shade-5 text-12">{"Account name"}</div>
                            </div>
                            <div class="flex flex-col gap-1.5">
                                <div class="text-lg">{234567889123}</div>
                                <div class="text-grey-shade-5 text-12">{"Account number"}</div>
                            </div>
                            <div class="flex flex-col gap-1.5">
                                <div class="text-lg">{"WW1000234"}</div>
                                <div class="text-grey-shade-5 text-12">{"IFSC code"}</div>
                            </div>
                        </div>
                    </div>
                    <div class="flex-1 flex flex-col gap-3 lg:ml-8">
                        <div class="text-grey-shade-1">{"Login details"}</div>
                        <div class="flex flex-col gap-4">
                            <div class="flex flex-col gap-1.5">
                                <div class="text-lg">{"JaneCooper@gmail.com"}</div>
                                <div class="text-grey-shade-5 text-12">{"Email ID"}</div>
                            </div>
                            <div class="flex flex-col gap-1.5">
                                <div class="text-lg">{"12-10-2023 | 12:00 AM"}</div>
                                <div class="text-grey-shade-5 text-12">{"Last active date/time"}</div>
                            </div>
                        </div>
                    </div>
                </div>
                <button class="bg-grey-shade-14 flex items-center rounded p-2 text-primary text-12 font-400" onclick={props.modal_handle.clone()}>
                    {"Close"}
                </button>
            </div>
        </div>
    }
}
