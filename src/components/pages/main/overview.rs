use chrono::Duration;
use chrono::NaiveDate;
use chrono::Utc;
use gloo_console::log;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::window;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::binding;

use crate::render_svg;

#[derive(Serialize, Deserialize, Clone)]
pub struct DateData {
    key: String,
    value: i32,
}

#[function_component(Overview)]
pub fn overview() -> Html {
    html!(
        <>
        <div class="bg-grey-shade-13  py-4">
            <div class="container mx-auto  md:w-auto space-y-4 " >
                <div class="flex justify-center space-y-2  flex-1  md:items-center flex-col md:flex-row  md:space-y-0 md:justify-between ">
                    <div class="flex flex-col justify-start">
                        <p class="flex items-center text-24 font-600 leading-32">
                             {"Analytics"}

                        </p>
                        <p class="text-14 text-grey-shade-5 font-sans">{"Result here updates every minute"}</p>
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
        // content
        <div class="container mx-auto space-y-6 py-5">
            // overviews
            <div class="grid gap-2 grid-cols-1 sm:grid-cols-2 md:grid-cols-4">
                <InfoCard
                    title = "Active users"
                    value = "112,245"
                />
                <InfoCard
                    title = "Total bets"
                    value = "$341,235"
                    percent = "18%"
                />
                <InfoCard
                    title = "Total Top ups"
                    value = "$69,129"
                    percent = "18%"
                />
                <InfoCard
                    title = "Inactive users"
                    value = "12,623"
                />
            </div>

            // Charts
            <div class="grid gap-2 grid-cols-1 md:grid-cols-2">
                <TotalUsersCard />
                <TotalVolumeCard />
                <TopPlatformCard />
                <TopupsAndCommissionCard />
            </div>
        </div>
        </>
    )
}

#[derive(Properties, PartialEq, Clone)]
struct InfoCardProps {
    title: String,
    value: String,
    #[prop_or_default]
    percent: Option<String>,
}

#[function_component(InfoCard)]
fn info_card(props: &InfoCardProps) -> Html {
    html! {
        <div class="flex  justify-start rounded-xl border border-grey-shade-11 p-4 space-x-5">
            <div class="bg-grey-shade-13 rounded-xl p-4">
                {render_svg!("mdi:user", width="24px", height="24px", color="#232323")}
            </div>
            <div class="space-y-1.5">
                <p class="text-14 font-sans font-400 leading-20 text-grey-shade-5">{&props.title}</p>
                <h2 class="text-24 font-sans font-600 leading-32 text-grey-shade-1">{&props.value}</h2>
            </div>
            if props.percent.is_some() {
            <div class="flex items-center space-x-2">
                {render_svg!("teenyicons:up-solid", width="8px", height="8px", color="#83BF94")}
                <span class="text-success  text-14 font-600 ">{props.percent.clone().unwrap()}</span>
            </div>
            } else {
                <span></span>
            }
        </div>
    }
}

// fn format_date_interval(from: &str, until: &str) -> String {
//     // let from_date = NaiveDate::parse_from_str(from, "%Y-%m-%d").unwrap();
//     // let until_date = NaiveDate::parse_from_str(until, "%Y-%m-%d").unwrap();

//     // let day_diff = (until_date - from_date).num_days();

//     // if day_diff <= 31 {
//     // // Less than or equal to 30 days, show individual dates
//     // let mut current_date = from_date;
//     // let mut dates = Vec::new();

//     // while current_date <= until_date {
//     //     dates.push(current_date.format("%d").to_string());
//     //     current_date = current_date + Duration::days(1);
//     // }

//     // format!("{}", dates.join(", "))

//     // } else if day_diff <= 365 {
//     // let mut current_date = from_date;
//     // let mut months = Vec::new();

//     // while current_date <= until_date {
//     //     months.push(current_date.format("%h").to_string());
//     //     current_date = current_date + Duration::days(30);
//     // }

//     // format!("{}", months.join(", "))
//     // } else {
//     //     let mut current_date = from_date;
//     //     let mut years = Vec::new();

//     //     while current_date <= until_date {
//     //         years.push(current_date.format("%Y").to_string());
//     //         current_date = current_date + Duration::days(365);
//     //     }

//     //     format!("{}", years.join(", "))
//     // }
// }

#[function_component(TotalUsersCard)]
fn total_users_card() -> Html {
    let start_date = use_state(String::default);
    let end_date = use_state(String::default);
    let is_loaded = use_state(|| true);

    use_effect(move || {
        let current_date = Utc::now().naive_utc();
        let start_date = (current_date - Duration::days(30))
            .format("%Y-%m-%d")
            .to_string();
        let end_date = current_date.format("%Y-%m-%d").to_string();
        log!("rendered");

        if *is_loaded {
            render_chart(start_date, end_date);
            is_loaded.set(false);
        }

        || {}
    });

    let from_input_handler = {
        let start_date = start_date.clone();
        let end_date: UseStateHandle<String> = end_date.clone();

        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            start_date.set(value.clone());

            if !(*end_date).clone().is_empty() {
                render_chart(value, (*end_date).clone());
            }
        })
    };

    let until_input_handler = {
        let start_date = start_date.clone();
        let end_date = end_date.clone();
        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            end_date.set(value.clone());

            if !(*start_date).clone().is_empty() {
                render_chart((*start_date).clone(), value);
            }
        })
    };

    fn render_chart(start_date: String, end_date: String) {
        let day_data = [
            DateData {
                key: "10".to_string(),
                value: 1000,
            },
            DateData {
                key: "11".to_string(),
                value: 1500,
            },
            DateData {
                key: "12".to_string(),
                value: 1500,
            },
            DateData {
                key: "13".to_string(),
                value: 3200,
            },
            DateData {
                key: "14".to_string(),
                value: 3200,
            },
            DateData {
                key: "15".to_string(),
                value: 8000,
            },
            DateData {
                key: "16".to_string(),
                value: 8000,
            },
            DateData {
                key: "18".to_string(),
                value: 5000,
            },
            DateData {
                key: "19".to_string(),
                value: 6000,
            },
            DateData {
                key: "20".to_string(),
                value: 7000,
            },
            DateData {
                key: "21".to_string(),
                value: 7000,
            },
        ];

        let month_data = [
            DateData {
                key: "Jan".to_string(),
                value: 3000,
            },
            DateData {
                key: "Feb".to_string(),
                value: 3000,
            },
            DateData {
                key: "Mar".to_string(),
                value: 3000,
            },
            DateData {
                key: "Apr".to_string(),
                value: 3000,
            },
            DateData {
                key: "May".to_string(),
                value: 3000,
            },
            DateData {
                key: "Jun".to_string(),
                value: 3200,
            },
            DateData {
                key: "Jul".to_string(),
                value: 3200,
            },
            DateData {
                key: "Aug".to_string(),
                value: 8000,
            },
            DateData {
                key: "Sep".to_string(),
                value: 8000,
            },
            DateData {
                key: "Oct".to_string(),
                value: 8000,
            },
            DateData {
                key: "Nov".to_string(),
                value: 8000,
            },
            DateData {
                key: "Dec".to_string(),
                value: 8000,
            },
        ];

        let year_data = [
            DateData {
                key: "2020".to_string(),
                value: 1000,
            },
            DateData {
                key: "2021".to_string(),
                value: 1500,
            },
            DateData {
                key: "2022".to_string(),
                value: 3500,
            },
            DateData {
                key: "2023".to_string(),
                value: 3500,
            },
        ];

        let document = window().unwrap().document().unwrap();

        let container = document.get_element_by_id("total-users-chart").unwrap();

        let container_width = container.client_width() as f64;
        let container_height = container.client_height() as f64;

        let from_date = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d").unwrap();
        let until_date = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").unwrap();

        let day_diff = (until_date - from_date).num_days();

        let data: Vec<DateData>;

        if day_diff <= 31 {
            log!("here. days..");
            data = day_data.to_vec();
        } else if day_diff <= 365 {
            log!("here.. month.");
            data = month_data.to_vec();
        } else {
            log!("here. year..");
            data = year_data.to_vec();
        }

        let data = serde_wasm_bindgen::to_value(&data).unwrap();

        log!("here...");
        binding::totalUsersChart(container_width, container_height, &data)
    }

    html! {
        <div class=" rounded-xl border border-grey-shade-11 p-4 space-y-2">
            <div class="flex flex-col md:flex-row items-center justify-between space-x-5">
            <div class="flex justify-between items-center">
                <h1 class="text-16 font-400 leading-20 text-grey-shade-1">{"Total user"}</h1>
            </div>
            <div class="flex  flex-col md:flex-row items-center justify-end space-x-2">
                <div class="flex items-center rounded border border-grey-shade-11 justify-start px-3 py-2" >
                    <label for="datepicker" class="text-12 text-grey-shade-3 font-300">{"From:"}</label>
                    <input id="datepicker" type="date" class=" outline-none text-12 text-grey-shade-3 font-300" oninput={from_input_handler}  />
                </div>
                <div class="flex items-center rounded border border-grey-shade-11 justify-start px-3 py-2" >
                    <label for="datepicker" class="text-12 text-grey-shade-3 font-300">{"To:"}</label>
                    <input id="datepicker" type="date" class=" outline-none text-12 text-grey-shade-3 font-300" oninput={until_input_handler}  />
                </div>
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
            <div class="flex items-center justify-start space-x-4">
                <div class="flex flex-col">
                    <p class="text-14 font-400 leading-20 text-grey-shade-5">{"Total users"}</p>
                    <h3 class="text-24 font-600 leading-32 text-primary" id="users">{"$210,691"}</h3>
                </div>
            </div>
            <div class="h-[360px] w-full"  id="total-users-chart">
            </div>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct MyProps {
    pub start_date: String,
    pub end_date: String,
}

#[function_component(TotalVolumeCard)]
fn total_volume_card() -> Html {
    let start_date = use_state(String::default);
    let end_date = use_state(String::default);
    let is_loaded = use_state(|| true);

    use_effect(move || {
        let current_date = Utc::now().naive_utc();
        let start_date = (current_date - Duration::days(30))
            .format("%Y-%m-%d")
            .to_string();
        let end_date = current_date.format("%Y-%m-%d").to_string();
        log!("rendered");

        if *is_loaded {
            render_chart(start_date, end_date);
            is_loaded.set(false);
        }

        || {}
    });

    let from_input_handler = {
        let start_date = start_date.clone();
        let end_date: UseStateHandle<String> = end_date.clone();

        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            start_date.set(value.clone());

            if !(*end_date).clone().is_empty() {
                render_chart(value, (*end_date).clone());
            }
        })
    };

    let until_input_handler = {
        let start_date = start_date.clone();
        let end_date = end_date.clone();
        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            end_date.set(value.clone());

            if !(*start_date).clone().is_empty() {
                render_chart((*start_date).clone(), value);
            }
        })
    };

    fn render_chart(start_date: String, end_date: String) {
        let day_data = [
            DateData {
                key: "10".to_string(),
                value: 1000,
            },
            DateData {
                key: "11".to_string(),
                value: 1500,
            },
            DateData {
                key: "12".to_string(),
                value: 1500,
            },
            DateData {
                key: "13".to_string(),
                value: 3200,
            },
            DateData {
                key: "14".to_string(),
                value: 3200,
            },
            DateData {
                key: "15".to_string(),
                value: 8000,
            },
            DateData {
                key: "16".to_string(),
                value: 8000,
            },
            DateData {
                key: "18".to_string(),
                value: 5000,
            },
            DateData {
                key: "19".to_string(),
                value: 6000,
            },
            DateData {
                key: "20".to_string(),
                value: 7000,
            },
            DateData {
                key: "21".to_string(),
                value: 7000,
            },
        ];

        let month_data = [
            DateData {
                key: "Jan".to_string(),
                value: 3000,
            },
            DateData {
                key: "Feb".to_string(),
                value: 3000,
            },
            DateData {
                key: "Mar".to_string(),
                value: 3000,
            },
            DateData {
                key: "Apr".to_string(),
                value: 3000,
            },
            DateData {
                key: "May".to_string(),
                value: 3000,
            },
            DateData {
                key: "Jun".to_string(),
                value: 3200,
            },
            DateData {
                key: "Jul".to_string(),
                value: 3200,
            },
            DateData {
                key: "Aug".to_string(),
                value: 8000,
            },
            DateData {
                key: "Sep".to_string(),
                value: 8000,
            },
            DateData {
                key: "Oct".to_string(),
                value: 8000,
            },
            DateData {
                key: "Nov".to_string(),
                value: 8000,
            },
            DateData {
                key: "Dec".to_string(),
                value: 8000,
            },
        ];

        let year_data = [
            DateData {
                key: "2020".to_string(),
                value: 1000,
            },
            DateData {
                key: "2021".to_string(),
                value: 1500,
            },
            DateData {
                key: "2022".to_string(),
                value: 3500,
            },
            DateData {
                key: "2023".to_string(),
                value: 3500,
            },
        ];

        let document = window().unwrap().document().unwrap();

        let container = document
            .get_element_by_id("profit-loss-chart-container")
            .unwrap();

        let container_width = container.client_width() as f64;
        let container_height = container.client_height() as f64;

        let from_date = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d").unwrap();
        let until_date = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").unwrap();

        let day_diff = (until_date - from_date).num_days();

        let data: Vec<DateData>;

        if day_diff <= 31 {
            log!("here. days..");
            data = day_data.to_vec();
        } else if day_diff <= 365 {
            log!("here.. month.");
            data = month_data.to_vec();
        } else {
            log!("here. year..");
            data = year_data.to_vec();
        }

        let data = serde_wasm_bindgen::to_value(&data).unwrap();

        log!("here...");
        binding::createProfitLossChart(container_width, container_height, &data)
    }

    html! {
        <div class=" rounded-xl border border-grey-shade-11 p-4 space-y-2">
            <div class="flex flex-col md:flex-row items-center justify-between space-x-5">
                <div class="flex justify-between items-center">
                    <h1 class="text-16 font-400 leading-20 text-grey-shade-1">{"Total Volume & Profit"}</h1>
                </div>
                <div class="flex flex-col md:flex-row items-center justify-end space-x-1">
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start px-3 py-2" >
                        <label for="datepicker" class="text-12 text-grey-shade-3 font-300">{"From:"}</label>
                        <input id="datepicker" type="date" class=" outline-none text-12 text-grey-shade-3 font-300"  oninput={from_input_handler}  />
                    </div>
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start px-3 py-2" >
                        <label for="datepicker" class="text-12 text-grey-shade-3 font-300">{"To:"}</label>
                        <input id="datepicker" type="date" class=" outline-none text-12 text-grey-shade-3 font-300" oninput={until_input_handler}   />
                    </div>
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

            <div class="flex items-center justify-start space-x-4">
                <div class="flex flex-col">
                    <p class="text-14 font-400 leading-20 text-grey-shade-5">{"Total volume"}</p>
                    <h3 class="text-24 font-600 leading-32 text-primary" id="totalVolume"></h3>
                </div>
                <div class="flex flex-col">
                    <p class="text-14 font-400 leading-20 text-grey-shade-5">{"Total profit earned"}</p>
                    <h3 class="text-24 font-600 leading-32 text-success" id="totalProfit"></h3>
                </div>
            </div>
            <div class="h-[360px] w-full" id="profit-loss-chart-container"></div>
        </div>
    }
}

#[function_component(TopPlatformCard)]
fn top_platform_card() -> Html {
    // let circle_data = vec![
    //     CircleChartData {
    //         label: "Evolution".to_string(),
    //         value: 5400,
    //     },
    //     CircleChartData {
    //         label: "Eugi".to_string(),
    //         value: 4520,
    //     },
    //     CircleChartData {
    //         label: "Power".to_string(),
    //         value: 2300,
    //     },
    //     CircleChartData {
    //         label: "Participate".to_string(),
    //         value: 4900,
    //     },
    // ];

    // Convert the data to JsValue
    // let json_string = serde_json::to_string(&circle_data).unwrap();

    // Call createCircleChart when the component is mounted
    use_effect(|| {
        binding::createCircleChart();
        || {}
    });

    html! {
        <div class=" rounded-xl border border-grey-shade-11 p-4 space-y-20 flex flex-col">
            <div>
                <h1 class="text-16 font-400 leading-20 text-grey-shade-1">{"Top platform"}</h1>
            </div>
            <div class="flex flex-col md:flex-row items-center justify-evenly ">
                    <div id="circle-chart-container"></div>
                    <div class="flex flex-col justify-center space-y-2">
                        <div>
                            <div class="text-14 text-grey-shade-5 flex items-center space-x-1"><div class="w-4 h-4 rounded-full bg-primary">{""}</div> <div>{"EVOLUTION"}</div></div>
                            <div>{"24,351 "}<span class="text-14 text-grey-shade-5 font-400">{"users"}</span></div>
                        </div>
                        <div>
                            <div class="text-14 text-grey-shade-5 flex  items-center space-x-1"><div class="w-4 h-4 rounded-full bg-success">{""}</div>  <div>{"EZUGI"}</div> </div>
                            <div class="text-24 font-600 text-grey-shade-1">{"24,351 "}<span class="text-14 text-grey-shade-5 font-400">{"users"}</span></div>
                        </div>
                        <div>
                            <div class="text-14 text-grey-shade-5 flex items-center space-x-1"><div class="w-4 h-4 rounded-full bg-[#1E84E2]">{""}</div>  <div>{"POWER GAMES"}</div> </div>
                            <div class="text-24 font-600 text-grey-shade-1">{"24,351 "}<span class="text-14 text-grey-shade-5 font-400">{"users"}</span></div>
                        </div>
                        <div>
                            <div class="text-14 text-grey-shade-5 flex items-center space-x-2"><div class="w-4 h-4 rounded-full bg-pending">{""}</div>  <div>{"PRAGMATICPLAY"}</div> </div>
                            <div class="text-24 font-600 text-grey-shade-1">{"24,351 "}<span class="text-14 text-grey-shade-5 font-400">{"users"}</span></div>
                        </div>
                    </div>
            </div>
            // <div class="flex items-start justify-start ">
            //     <div class="flex justify-between items-center">
            //         <h1 class="text-16 font-400 leading-20 text-grey-shade-1">{"Top platform"}</h1>
            //     </div>
            // </div>
            // <div class="flex items-center justify-center">

            // </div>
        </div>
    }
}

#[function_component(TopupsAndCommissionCard)]
fn topups_commision_card() -> Html {
    let start_date = use_state(String::default);
    let end_date = use_state(String::default);
    let is_loaded = use_state(|| true);

    use_effect(move || {
        let current_date = Utc::now().naive_utc();
        let start_date = (current_date - Duration::days(30))
            .format("%Y-%m-%d")
            .to_string();
        let end_date = current_date.format("%Y-%m-%d").to_string();
        log!("rendered commision");

        if *is_loaded {
            render_chart(start_date, end_date);
            is_loaded.set(false);
        }

        || {}
    });

    let from_input_handler = {
        let start_date = start_date.clone();
        let end_date: UseStateHandle<String> = end_date.clone();

        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            start_date.set(value.clone());

            if !(*end_date).clone().is_empty() {
                render_chart(value, (*end_date).clone());
            }
        })
    };

    let until_input_handler = {
        let start_date = start_date.clone();
        let end_date = end_date.clone();
        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            end_date.set(value.clone());

            if !(*start_date).clone().is_empty() {
                render_chart((*start_date).clone(), value);
            }
        })
    };

    fn render_chart(start_date: String, end_date: String) {
        let day_data = [
            DateData {
                key: "10".to_string(),
                value: 1000,
            },
            DateData {
                key: "11".to_string(),
                value: 1500,
            },
            DateData {
                key: "12".to_string(),
                value: 1500,
            },
            DateData {
                key: "13".to_string(),
                value: 3200,
            },
            DateData {
                key: "14".to_string(),
                value: 3200,
            },
            DateData {
                key: "15".to_string(),
                value: 8000,
            },
            DateData {
                key: "16".to_string(),
                value: 8000,
            },
            DateData {
                key: "18".to_string(),
                value: 5000,
            },
            DateData {
                key: "19".to_string(),
                value: 6000,
            },
            DateData {
                key: "20".to_string(),
                value: 7000,
            },
            DateData {
                key: "21".to_string(),
                value: 7000,
            },
        ];

        let month_data = [
            DateData {
                key: "Jan".to_string(),
                value: 3000,
            },
            DateData {
                key: "Feb".to_string(),
                value: 3000,
            },
            DateData {
                key: "Mar".to_string(),
                value: 3000,
            },
            DateData {
                key: "Apr".to_string(),
                value: 3000,
            },
            DateData {
                key: "May".to_string(),
                value: 3000,
            },
            DateData {
                key: "Jun".to_string(),
                value: 3200,
            },
            DateData {
                key: "Jul".to_string(),
                value: 3200,
            },
            DateData {
                key: "Aug".to_string(),
                value: 8000,
            },
            DateData {
                key: "Sep".to_string(),
                value: 8000,
            },
            DateData {
                key: "Oct".to_string(),
                value: 8000,
            },
            DateData {
                key: "Nov".to_string(),
                value: 8000,
            },
            DateData {
                key: "Dec".to_string(),
                value: 8000,
            },
        ];

        let year_data = [
            DateData {
                key: "2020".to_string(),
                value: 1000,
            },
            DateData {
                key: "2021".to_string(),
                value: 1500,
            },
            DateData {
                key: "2022".to_string(),
                value: 3500,
            },
            DateData {
                key: "2023".to_string(),
                value: 3500,
            },
        ];

        let document = window().unwrap().document().unwrap();

        let container = document.get_element_by_id("topup-chart-container").unwrap();

        let container_width = container.client_width() as f64;
        let container_height = container.client_height() as f64;

        let from_date = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d").unwrap();
        let until_date = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").unwrap();

        let day_diff = (until_date - from_date).num_days();

        let data: Vec<DateData>;

        if day_diff <= 31 {
            log!("here. days..");
            data = day_data.to_vec();
        } else if day_diff <= 365 {
            log!("here.. month.");
            data = month_data.to_vec();
        } else {
            log!("here. year..");
            data = year_data.to_vec();
        }

        let data = serde_wasm_bindgen::to_value(&data).unwrap();

        log!("here...");
        binding::createTopupsCommissionChart(container_width, container_height, &data)
    }

    html! {
        <div class=" rounded-xl border border-grey-shade-11 p-4 space-y-2">
            <div class="flex flex-col md:flex-row items-center justify-between">
                <div class="flex justify-between items-center">
                    <h1 class="text-16 font-400 leading-20 text-grey-shade-1">{"Topups & Commissions earned"}</h1>
                </div>
                <div class="flex flex-col md:flex-row items-center justify-end space-x-1">
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start px-2 py-2" >
                        <label for="datepicker" class="text-12 text-grey-shade-3 font-300">{"From:"}</label>
                        <input id="datepicker" type="date" class=" outline-none text-12 text-grey-shade-3 font-300" oninput={from_input_handler}  />
                    </div>
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start px-2 py-2" >
                        <label for="datepicker" class="text-12 text-grey-shade-3 font-300">{"To:"}</label>
                        <input id="datepicker" type="date" class=" outline-none text-12 text-grey-shade-3 font-300" oninput={until_input_handler}  />
                    </div>
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

            <div class="flex items-center justify-start space-x-4">
                <div class="flex flex-col">
                    <p class="text-14 font-400 leading-20 text-grey-shade-5">{"Total Topups"}</p>
                    <h3 class="text-24 font-600 leading-32 text-primary" id="topups"></h3>
                </div>
                <div class="flex flex-col">
                    <p class="text-14 font-400 leading-20 text-grey-shade-5">{"Total commission"}</p>
                    <h3 class="text-24 font-600 leading-32 text-success" id="commission"></h3>
                </div>
            </div>

            <div class="h-[360px] w-full" id="topup-chart-container"></div>
        </div>
    }
}
