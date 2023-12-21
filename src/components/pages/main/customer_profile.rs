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
                <button
                    class="bg-grey-shade-0 w-full lg:w-fit flex flex-row justify-center items-center rounded p-2 text-grey-shade-14 text-12 font-400 "
                >
                    <span class="pr-2">
                    {html! { render_svg!("majesticons:file", color="#ffff",width="14px")}}
                    </span>
                    {"Export"}
                </button>
            </div>
            // Profile
            <div class="bg-grey-shade-14 rounded-3xl flex items-center justify-between p-5 shadow-md shadow-grey-shade-0/15 space-x-4">
                <div>
                    <img src={"img/circle_profile.png"} alt="Selected Image" class="w-20 md:w-22" />
                </div>
                <div class="flex-1 flex justify-between items-center">
                    <div>
                        // <h3>{auth_store.current_user.name.clone()}</h3>
                        <p>{"Username"}</p>
                    </div>
                    <div>
                        <button class="bg-grey-shade-0 flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
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
                // Banking details
                <div class="bg-grey-shade-12 border border-grey-shade-11 p-5 gap-6 rounded w-full md:w-1/4">
                    <p class="text-16 font-700 leading-20 text-grey-shade-0">{"Banking details"}</p>
                    <div class="space-y-6 pt-3">
                        <div>
                            <p class="text-12 font-400 leading-20 text-shade-2">{"Beckham ande"}</p>
                            <p class="text-11 font-400 leading-20 text-grey-shade-5">{"Account name"}</p>
                        </div>
                        
                        <div>
                            <p class="text-12 font-400 leading-20 text-shade-2">{"234567889123"}</p>
                            <p class="text-11 font-400 leading-20 text-grey-shade-5">{"Account number"}</p>
                        </div>

                        <div>
                            <p class="text-12 font-400 leading-20 text-shade-2">{"WW1000234"}</p>
                            <p class="text-11 font-400 leading-20 text-grey-shade-5">{"IFSC code"}</p>
                        </div>
                    </div>

                    <p class="text-16 font-700 leading-20 text-grey-shade-0">{"Login details"}</p>
                    <div class="space-y-6 pt-3">
                        <div>
                            // <p class="text-12 font-400 leading-20 text-shade-2">{user.email_address.clone()}</p>
                            <p class="text-11 font-400 leading-20 text-grey-shade-5">{"Email ID"}</p>
                        </div>
                        
                        
                        <div>
                            <p class="text-12 font-400 leading-20 text-shade-2">{"12-10-2023 | 12:00 AM"}</p>
                            <p class="text-11 font-400 leading-20 text-grey-shade-5">{"Last active date/time"}</p>
                        </div>

                        <div>
                            <p class="text-12 font-400 leading-20 text-shade-2">{"196.69.80.124"}</p>
                            <p class="text-11 font-400 leading-20 text-grey-shade-5">{"India, Tamilnadu, chennai"}</p>
                        </div>

                        <div>
                            <p class="text-12 font-400 leading-20 text-shade-2">{"Iphone X"}</p>
                            <p class="text-11 font-400 leading-20 text-grey-shade-5">{"Android 11, SM-G991U"}</p>
                        </div>
                    </div>
                </div>

                // Notification and remove
                <div class=" w-full space-y-6">
                    //    Notifications
                    <div class="bg-grey-shade-14 border border-grey-shade-11 p-5 rounded">
                        <p class="text-16 font-700 leading-20 text-grey-shade-0">{"Notifications"}</p>
                        
                   </div>
                   <div class="bg-grey-shade-14 border border-grey-shade-11 p-5 rounded space-y-2">
                        <p class="text-16 font-700 leading-20 text-grey-shade-0">{"Delete account"}</p>
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
        </>
    }
}
