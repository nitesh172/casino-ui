use yew::prelude::*;

use crate::render_svg;

#[function_component(Tickets)]
pub fn tickets() -> Html {
    html! {
        <>
        <div class="bg-grey-shade-13  py-4">
            <div class="container mx-auto md:w-auto space-y-4 " >
                <div class="flex justify-between items-center">
                    <div class="flex space-x-4 items-center">
                        <h1>{"Tickets"}</h1>
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
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"ID"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Email ID"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Username"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Query"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Status"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Action"}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{"WWC001"}</td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider pl-5">{"JaneCooper@gmail.com"}</td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider">{"Jane Cooper"}</td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider pr-5">{"I made a deposit of $50, but it's not reflecting in my account. Transaction ID: 123456."}</td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-14 tracking-wider">
                            <span class="bg-warning rounded-full py-1 px-2 w-[24px]">
                                {"Open"}
                            </span>
                        </td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider relative group cursor-pointer">
                            <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                            <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">
                                    <a href="#" class="">
                                    { "Open" }
                                    </a>
                                </li>
                                <li  class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2 hover:bg-grey-shade-12">
                                    <a href="#" class="text-grey-shade-0 ">
                                        { "Close" }
                                    </a>
                                </li>
                            </ul>
                        </td>
                    </tr>
                    <tr>
                    <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{"WWC001"}</td>
                    <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider pl-5">{"JaneCooper@gmail.com"}</td>
                    <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider">{"Jane Cooper"}</td>
                    <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider pr-5">{"I made a deposit of $50, but it's not reflecting in my account. Transaction ID: 123456."}</td>
                    <td class="py-3 text-left text-14 font-medium  text-grey-shade-14 tracking-wider">
                        <span class="bg-success rounded-full py-1 px-2">
                            {"Closed"}
                        </span>
                    </td>
                    <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider relative group cursor-pointer">
                            <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                            <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">
                                    <a href="#" class="">
                                    { "Open" }
                                    </a>
                                </li>
                                <li  class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2 hover:bg-grey-shade-12">
                                    <a href="#" class="text-grey-shade-0 ">
                                        { "Close" }
                                    </a>
                                </li>
                            </ul>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        </div>
        </>
    }
}
