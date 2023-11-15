use yew::prelude::*;

use crate::render_svg;

#[function_component(Notifications)]
pub fn notifications() -> Html {
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
                <div class="flex justify-between items-center">
                    <div class="flex space-x-4 items-center">
                        <h1>{"Notifications"}</h1>
                            <div class="flex items-center rounded border justify-start border-grey-shade-11 px-2 w-40">
                                <span>{html! { render_svg!("mynaui:search",  color="#000000", width="18px")}} </span>
                                <input
                                    id="search"
                                    autocomplete="off"
                                    name="search"
                                    placeholder={"search"}
                                    // value={
                                    // oninput={input_handler}
                                    // type={input_type}
                                    class="px-2.5 py-2 h-7 bg-white placeholder:text-grey-shade-6 text-14  leading-20 font-300 font-sans outline-none pr-2 pl-2 w-full"
                                />
                        </div>
                    </div>
                    <div class="flex space-x-4 items-center">
                        <button
                            class="bg-grey-shade-0 flex items-center rounded p-2 text-grey-shade-14 text-12 font-400"
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
                <div class="flex justify-between items-center text-grey-shade-5">
                    <div class="flex space-x-4 items-center">
                        <p>{"Showing 1 - 10 of 20 results"}</p>
                        <ul class="flex items-center space-x-1 text-grey-shade-0">
                            <li class="bg-grey-shade-8 py-1 px-2 rounded-sm text-grey-shade-0">{"1"}</li>
                            <li>{"2"}</li>
                            <li>{"3"}</li>
                            <li>{"4"}</li>
                            <li>{"5"}</li>
                        </ul>
                    </div>
                    <div class="flex space-x-4 items-center">
                        <p>{"Items per page"}</p>
                        <select class="focus:outline-none p-1 border-grey-shade-6 rounded">
                            <option selected=true>{"10"}</option>
                            <option>{"20"}</option>
                            <option>{"30"}</option>
                            <option>{"40"}</option>
                            <option>{"50"}</option>
                        </select>
                        <span>
                            {html! { render_svg!("ic:round-arrow-left", color="#000000",width="24px")}}
                        </span>
                        <span>
                            {html! { render_svg!("ic:round-arrow-right", color="#000000",width="24px")}}
                        </span>
                    </div>
                </div>
            </div>
        </div>
        <div class="relative">
        <div class="absolute -z-10 top-0 left-0 h-[45px] w-full bg-grey-shade-13"></div>
        <div class="container mx-auto z-1">
            <table class="w-full table-auto">
                <thead class="bg-grey-shade-13">
                    <tr class="">
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Notification Message"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Status"}</th>
                        <th class="py-3 pl-5 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Offer available from"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Offer available until"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 pr-5 tracking-wider">{"Created on"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Action"}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{"Weekends just got better! Play our featured slots and compete for a share of the $10,000 jackpot."}</td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider">
                            <span class="bg-grey-shade-11 rounded-full py-1 px-2">
                                {"Active"}
                            </span>
                        </td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider pl-5">{"12 Sept 2023 | 12:00"}</td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider">{"12 Sept 2023 | 12:00"}</td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider pr-5">{"12 Sept 2023"}</td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider"> {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</td>
                    </tr>
                    <tr>
                        <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{"Weekends just got better! Play our featured slots and compete for a share of the $10,000 jackpot."}</td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider">
                            <span class="bg-grey-shade-11 rounded-full py-1 px-2">
                                {"Active"}
                            </span>
                        </td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider pl-5">{"12 Sept 2023 | 12:00"}</td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider">{"12 Sept 2023 | 12:00"}</td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider pr-5">{"12 Sept 2023"}</td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider"> {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</td>
                    </tr>
                    <tr>
                        <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{"Weekends just got better! Play our featured slots and compete for a share of the $10,000 jackpot."}</td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider">
                            <span class="bg-grey-shade-11 rounded-full py-1 px-2">
                                {"Active"}
                            </span>
                        </td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider pl-5">{"12 Sept 2023 | 12:00"}</td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider">{"12 Sept 2023 | 12:00"}</td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider pr-5">{"12 Sept 2023"}</td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider"> {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</td>
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
                            name="message"
                            placeholder="Message"
                            class="px-3.5 py-3placeholder:text-grey-shade-6 text-14 leading-20
                            w-full  resize-none
                            bg-white
                            border-grey-shade-11
                            font-300 font-sans outline-none"
                        />
                    </div>
                </div>
                <div class="flex flex-col space-y-4">
                    <label class="flex items-center space-x-2">
                        <input type="checkbox" class="border-2 border-primary focus:ring-grey-shade-5 w-[20px] h-[20px] toggle-checkbox:checked " />
                        <span class="text-gray-700">{"Offer expiry date"}</span>
                    </label>
                    <div class="flex justify-between items-center">
                        <div>
                            <label for="datetime" class="block text-sm font-medium text-gray-700">{"Offer available from"}</label>
                            <input type="datetime-local" id="datetime" placeholder="Select a date and time" class="mt-1 p-2 border rounded-md w-full focus:outline-none focus:border-blue-500 focus:ring focus:ring-blue-200 my-custom-tailwind-class" />

                        </div>
                        <div>
                            <label for="datetime" class="block text-sm font-medium text-gray-700">{"Offer available until"}</label>
                            <input type="datetime-local" id="datetime" name="datetime" class="mt-1 p-2 border rounded-md w-full" />
                        </div>
                    </div>
                </div>
                <div class="space-y-4">
                    <p>{"Notify this through"}</p>
                    <div class="flex flex-col space-y-4">
                    <label class="flex items-center space-x-2">
                        <input type="checkbox" class="" />
                        <span class="text-gray-700">{"Offer expiry date"}</span>
                    </label>
                    <label class="flex items-center space-x-2 toggle-label ">
                        <input type="checkbox" class="" />
                        <span class="text-gray-700">{"Offer expiry date"}</span>
                    </label>
                    <label class="flex items-center space-x-2">
                    <input type="checkbox" class="" />
                    <span>{"Offer expiry date"}</span>
                </label>
                    </div>
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
