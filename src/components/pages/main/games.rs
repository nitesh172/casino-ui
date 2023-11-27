use yew::prelude::*;

use crate::{ components::organisms::paginator::Paginator, render_svg };

#[function_component(Games)]
pub fn games() -> Html {
    let is_open = use_state(|| false);

    let modal_handle = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };

    html! {
        <>
        <div class="bg-grey-shade-13  py-4">
            <div class="container mx-auto md:w-auto space-y-4 " >
                <div class="flex md:justify-between items-start md:items-center flex-col md:flex-row ">
                    <div class="flex flex-col md:flex-row space-x-4  items-start md:items-center space-y-2 md:space-y-0">
                        <h1>{"Game"}</h1>
                        <div class="flex items-center rounded border justify-start border-grey-shade-11 px-2 w-40">
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
                    <div class="flex space-x-4 items-center">
                        <button
                            class="bg-grey-shade-0 flex items-center rounded p-2 text-grey-shade-14 text-12 font-400 "
                        >
                            <span class="pr-2">
                            {html! { render_svg!("majesticons:file", color="#ffff",width="14px")}}
                            </span>
                            {"Export"}
                        </button>
                        <button
                            onclick={modal_handle.clone()}
                            class="bg-primary flex items-center rounded p-2 text-grey-shade-14 text-12 font-400"
                        >
                            <span class="pr-2">
                            {html! { render_svg!("lets-icons:add-round", color="#ffff",width="14px")}}
                            </span>
                            {"Create"}
                        </button>
                     </div>
                </div>
                <Paginator per_page={10} total_pages={1} total_items={5} current_page={1}  />
            </div>
        </div>
        <div class="relative">
        <div class="absolute -z-10 top-0 left-0 h-[45px] w-full bg-grey-shade-13"></div>
        <div class="container mx-auto">
            <table class="w-full table-auto">
                <thead class="bg-grey-shade-13">
                    <tr class="">
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Name"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Category"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Provide"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Action"}</th>
                    </tr>
                </thead>
                <tbody class="overflow-y-auto">
                    <tr>
                        <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{"Blackjack"}</td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider">
                            <span class="bg-grey-shade-11 rounded-full py-1 px-2">
                                {"Slot Machine"}
                            </span>
                        </td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">{"Evolution"}</td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider relative group cursor-pointer">
                            <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                            <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">
                                    <a href="#" class="">
                                    { "Remove" }
                                    </a>
                                </li>
                            </ul>
                        </td>
                    </tr>
                    <tr>
                        <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{"Blackjack"}</td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider">
                            <span class="bg-grey-shade-11 rounded-full py-1 px-2">
                                {"Table Games"}
                            </span>
                        </td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">{"Evolution"}</td>
                        <td class="py-3 text-right text-14 font-medium text-grey-shade-1 tracking-wider relative group cursor-pointer">
                            <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                            <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">
                                    <a href="#" class="">
                                    { "Remove" }
                                    </a>
                                </li>
                            </ul>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        </div>
        { if *is_open {html! {<Modal modal_handle={modal_handle.clone()}  />}  } else { html! ("") } }
        </>
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
                    <p>{"Add game"}</p>
                    <span class="cursor-pointer" onclick={props.modal_handle.clone()}>
                        {html! {render_svg!("mdi:multiply",color="#232323",width="25px")}}
                    </span>
                </div>

                <div class="flex flex-col space-y-1.5 md:w-[600px]">
                    <label
                        for="game"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"Platform name"}
                    </label>
                    <div class="relative inline-flex w-full ">
                        <select class="appearance-none border border-gray-300 rounded px-3 py-2 focus:outline-none w-full ">
                            <option>{"Evolution"}</option>
                            <option>{"Ezugi"}</option>
                            <option>{"Qtech"}</option>
                        </select>
                        <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
                            <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 11.586l3.293-3.293a1 1 0 011.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
                            </svg>
                        </div>
                    </div>
                </div>
                <div class="flex flex-col space-y-1.5 md:w-[600px]">
                    <label
                        for="category"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"Category"}
                    </label>
                    <div class="relative inline-flex w-full ">
                        <select  id="category" class="appearance-none border border-gray-300 rounded px-3 py-2 focus:outline-none w-full ">
                            <option>{"Table games"}</option>
                            <option>{"Slot machines"}</option>
                            <option>{"Video poker"}</option>
                        </select>
                        <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
                            <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 11.586l3.293-3.293a1 1 0 011.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
                            </svg>
                        </div>
                    </div>
                </div>
                <div class="md:w-[600px]">
                <ul class="flex flex-wrap w-full space-y-2 space-x-8 items-baseline justify-start">
                    <li class="bg-grey-shade-2 text-grey-shade-14 text-12 px-3 py-2 rounded-lg">
                        {"Blackjack"}
                    </li>
                    <li class="bg-grey-shade-2 text-grey-shade-14 text-12 px-3 py-2 rounded-lg">{"Roulette"}</li>
                    <li class="bg-grey-shade-2 text-grey-shade-14 text-12 px-3 py-2 rounded-lg">{"Poker"}</li>
                    <li class="bg-grey-shade-13 text-grey-shade-1 text-12 px-3 py-2 rounded-lg">{"Baccarat"}</li>
                    <li class="bg-grey-shade-13 text-grey-shade-1 text-12 px-3 py-2 rounded-lg">{"Craps"}</li>
                    <li class="bg-grey-shade-13 text-grey-shade-1 text-12 px-3 py-2 rounded-lg">{"Pai Gow Poker"}</li>
                    <li class="bg-grey-shade-13 text-grey-shade-1 text-12 px-3 py-2 rounded-lg">{"Sci Bo"}</li>
                </ul>
                </div>
                <div class="flex space-x-4  items-center justify-start p-0 rounded-sm">
                    <button type="submit"  class="bg-primary flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
                        {"Add"}
                    </button>
                    <button class="bg-grey-shade-14 flex items-center rounded p-2 text-primary text-12 font-400" onclick={props.modal_handle.clone()}>
                        {"Cancel"}
                    </button>
                </div>
            </div>
        </div>
    }
}
