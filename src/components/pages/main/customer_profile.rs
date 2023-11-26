use yew::prelude::*;

use crate::{components::organisms::paginator::Paginator, render_svg};

#[derive(Properties, PartialEq, Clone)]
pub struct CustomerProfileProps {
    pub id: String,
}

#[function_component(CustomerProfile)]
pub fn customer_profile(_props: &CustomerProfileProps) -> Html {
    html! {
        <>
            <div class="bg-grey-shade-13  py-4">
                <div class="container mx-auto  md:w-auto space-y-4 " >
                    <div class="flex justify-center space-y-2  flex-1  md:items-center flex-col md:flex-row  md:space-y-0 md:justify-between ">
                        <div class="flex md:space-x-4  items-between md:items-start space-y-2 md:space-y-0 justify-between md:justify-start flex-1">
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
                        <div class="flex md:space-x-4 md:items-center">
                            <button
                                class="bg-grey-shade-0 flex items-center rounded p-2 text-grey-shade-14 text-12 font-400 w-1/2 md:w-full text-center"
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
            <div class="relative">
                <div class="container mx-auto space-y-2">
                    // Upper section
                    <div class="bg-grey-shade-14 rounded-3xl flex items-center justify-start space-x-4 p-5 shadow-md shadow-grey-shade-0/15">
                        <div>
                            <img src="img/circle_profile.png" class="w-22" />
                        </div>
                        <div>
                            <h3 class="text-16 font-400 leading-20 text-grey-shade-1">{"Beckham Andy"}</h3>
                            <p class="text-12 font-400 leading-16 text-grey-shade-5">{"Username"}</p>
                        </div>
                        <div>
                            <h3 class="text-16 font-400 leading-20 text-grey-shade-1">{"32"}</h3>
                            <p class="text-12 font-400 leading-16 text-grey-shade-5">{"Age"}</p>
                        </div>
                        <div>
                            <h3 class="text-14 font-400 leading-20 text-grey-shade-1">{"Male"}</h3>
                            <p class="text-12 font-400 leading-16 text-grey-shade-5">{"Gender"}</p>
                        </div>
                        <div>
                            <h3 class="text-14 font-400 leading-20 text-grey-shade-1">{"23, Brook St, Dubai, UAE"}</h3>
                            <p class="text-12 font-400 leading-16 text-grey-shade-5">{"Address"}</p>
                        </div>
                    </div>

                    <div class="flex flex-col md:flex-row space-y-2 md:space-y-0 md:space-x-4">
                        // contet left
                        <div class="flex flex-col items-start border border-grey-shade-12 p-5 bg-grey-shade-13 space-y-6 rounded-xl">
                            <div class="space-y-3">
                                <h2 class="text-16 font-700 leading-21 text-grey-shade-1">{"Banking details"}</h2>
                                <div class="space-y-1.5">
                                    <h3 class="text-14 font-400 leading-20 text-grey-shade-2">{"Beckham ande"}</h3>
                                    <p class="text-12 font-400 leading-16 text-grey-shade-5">{"Account name"}</p>
                                </div>
                                <div class="space-y-1.5">
                                    <h3 class="text-14 font-400 leading-20 text-grey-shade-2">{"234567889123"}</h3>
                                    <p class="text-12 font-400 leading-16 text-grey-shade-5">{"Account number"}</p>
                                </div>
                                <div class="space-y-1.5">
                                    <h3 class="text-14 font-400 leading-20 text-grey-shade-2">{"WW1000234"}</h3>
                                    <p class="text-12 font-400 leading-16 text-grey-shade-5">{"IFSC code number"}</p>
                                </div>
                            </div>
                            <div class="space-y-3">
                                <h2 class="text-16 font-700 leading-21 text-grey-shade-1">
                                    {"Login details"}
                                </h2>
                                <div class="space-y-1.5">
                                    <h3 class="text-14 font-400 leading-20 text-grey-shade-2">{"JaneCooper@gmail.com"}</h3>
                                    <p class="text-12 font-400 leading-16 text-grey-shade-5">{"Email ID"}</p>
                                </div>
                                <div class="space-y-1.5">
                                    <h3 class="text-14 font-400 leading-20 text-grey-shade-2">{"12-10-2023 | 12:00 AM"}</h3>
                                    <p class="text-12 font-400 leading-16 text-grey-shade-5">{"Last active date/time"}</p>
                                </div>
                                <div class="space-y-1.5">
                                    <h3 class="text-14 font-400 leading-20 text-grey-shade-2">{"196.69.80.124"}</h3>
                                    <p class="text-12 font-400 leading-16 text-grey-shade-5">{"India, Tamilnadu, chennai"}</p>
                                </div>
                                <div class="space-y-1.5">
                                    <h3 class="text-14 font-400 leading-20 text-grey-shade-2">{"IPhone X"}</h3>
                                    <p class="text-12 font-400 leading-16 text-grey-shade-5">{"Android 11, SM-G991U"}</p>
                                </div>
                            </div>
                        </div>
                        // content right
                        <div class="flex flex-col flex-1 space-y-4">
                            // content upper + label
                            <div class="flex flex-col md:flex-row items-center justify-between space-y-2 md:space-x-2 md:space-y-0 flex-1">
                                <div class="flex flex-col bg-grey-shade-14 rounded-xl border border-grey-shade-11 p-5 space-y-2 md:basis-3/4" >
                                    <h1 class="text-16 leading-21 font-700 font-sans text-grey-shade-1">{"Game Play Data"}</h1>
                                    <div class="flex items-center justify-between">
                                        <div class="flex items-center justify-between space-x-3">
                                            <img src="img/dice.png" class="w-8 h-8" />
                                            <div class="space-y-1.5 flex flex-col">
                                                <h3 class="text-16 font-600 leading-21">{"23"}</h3>
                                                <p class="text-12 text-grey-shade-5 leading-16">{"Total games"}</p>
                                            </div>
                                        </div>
                                        <div class="flex flex-col space-y-1.5">
                                            <h3 class="text-16 font-600 leading-21">{"$25"}</h3>
                                            <p class="text-12 text-grey-shade-5 leading-16">{"Total bet"}</p>
                                        </div>
                                        <div class="flex flex-col space-y-1.5">
                                            <h3 class="text-16 font-600 leading-21">{"$792"}</h3>
                                            <p class="text-12 text-grey-shade-5 leading-16">{"Won"}</p>
                                        </div>
                                        <div class="flex flex-col space-y-1.5">
                                            <h3 class="text-16 font-600 leading-21">{"$67"}</h3>
                                            <p class="text-12 text-grey-shade-5 leading-16">{"Lost"}</p>
                                        </div>
                                    </div>
                                </div>
                                <div class="flex flex-col bg-info p-5 space-y-2 rounded-xl border border-grey-shade-11 md:basis-1/4">
                                    <h1 class="text-16 leading-21 font-700 font-sans text-grey-shade-14">{"Won/Lost"}</h1>
                                    <div class="flex items-center justify-between space-x-6">
                                        <div class="flex justify-between items-center space-x-3">
                                            <img src="img/trophy.png" class="w-8 h-8" />
                                            <div class="space-y-1.5 flex flex-col">
                                                <h3 class="text-16 font-600 leading-21 text-grey-shade-14">{"10"}</h3>
                                                <p class="text-12 text-grey-shade-14 leading-16">{"Won"}</p>
                                            </div>
                                        </div>
                                        <div class="flex justify-between items-center space-x-3">
                                            <img src="img/sad.png" class="w-8 h-8" />
                                            <div  class="space-y-1.5 flex flex-col">
                                                <h3 class="text-16 font-600 leading-21 text-grey-shade-14">{"5"}</h3>
                                                <p class="text-12 text-grey-shade-14 leading-16">{"Lost"}</p>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            // tables
                            <div class="flex flex-col bg-grey-shade-14 rounded-xl border border-grey-shade-11 p-5 flex-1">
                                <div class="flex flex-col">
                                    <h2>{"Successful referrals"}</h2>
                                    <Paginator />
                                </div>
                                <ReferralTable />
                            </div>
                            // table
                            <div class="flex flex-col bg-grey-shade-14 rounded-xl border border-grey-shade-11 p-5 flex-1">
                                <div class="flex flex-col">
                                    <h2>{"Payment data"}</h2>
                                    <Paginator />
                                </div>
                                <PaymentTable />
                            </div>
                        </div>

                    </div>

                </div>
            </div>
        </>
    }
}

#[function_component(ReferralTable)]
fn referral_table() -> Html {
    html! {
        <div class="py-2 flex-1">
        <table class="w-full table-auto">
            <thead>
                <tr class="">
                    <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Name"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Commission earned"}</th>
                            <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Total amount"}</th>
                            <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Last active date/time"}</th>
                        </tr>
                    </thead>
                    <tbody class="overflow-y-auto">
                        <tr >
                            <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{"Darlene Robertson"}</td>
                            <td class="py-3 text-left text-14 font-medium text-success tracking-wider font-uppercase pr-5">{"5 points"}</td>
                            <td class="py-3 text-left text-14 font-medium text-success tracking-wider font-uppercase pr-5">{"$ 50.50"}</td>
                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5 flex space-x-2">
                                {render_svg!("lets-icons:date-range-fill", width="18px", height="18px", color="#343434") }
                                <span>
                                    {"12-10-2023 | 12:00 AM"}
                                </span>
                            </td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}

#[function_component(PaymentTable)]
fn payment_table() -> Html {
    html! {
        <div class="py-2 flex-1">
        <table class="w-full table-auto">
            <thead>
                <tr class="">
                    <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Transaction ID"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Point received"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Amount deposited"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Amount deposited on"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Status"}</th>
                        <th class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{"Actions"}</th>
                        </tr>
                    </thead>
                    <tbody class="overflow-y-auto">
                        <tr >
                            <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{"NXlWtIPIz3T5QHmEknzr3MGRf6btzusdger"}</td>
                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">{"1000"}</td>
                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">{"$80"}</td>
                            <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">{"11, Oct 2023"}</td>
                            <td class="py-3 text-left text-14 font-medium text-grey-shade-14 tracking-wider">
                                <span class="bg-success rounded-full py-1 px-2 w-[24px]">
                                    {"Complete"}
                                </span>
                        </td>
                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider relative group cursor-pointer">
                                <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                    <li class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">
                                        <a href="#" class="">
                                        { "Ban" }
                                        </a>
                                    </li>
                                    <li  class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2 hover:bg-grey-shade-12">
                                        <a href="#" class="text-grey-shade-0 ">
                                            { "Unban" }
                                        </a>
                                    </li>
                                </ul>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
