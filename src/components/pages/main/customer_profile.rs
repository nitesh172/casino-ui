use yew::prelude::*;

use crate::render_svg;

#[derive(Properties, PartialEq, Clone)]
pub struct CustomerProfileProps {
    pub id: String,
}

#[function_component(CustomerProfile)]
pub fn customer_profile(_props: &CustomerProfileProps) -> Html {
    html! {
        <>
            <div class="bg-grey-shade-13  py-4">
                <div class="container mx-auto md:w-auto space-y-4 " >
                    <div class="flex md:justify-between items-start md:items-center flex-col md:flex-row ">
                        <div class="flex flex-col md:flex-row space-x-4  items-start md:items-center space-y-2 md:space-y-0">
                            <p class="flex items-center text-24 font-600 leading-32">
                                <span class="pr-2">
                                {html! { render_svg!("bx:arrow-back", color="#000000",width="20px")}}
                                </span>
                                {"Customer"}
                            </p>
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
                        </div>
                    </div>
                </div>
            </div>
        </>
    }
}
