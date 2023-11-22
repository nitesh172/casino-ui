use gloo_console::log;
use web_sys::window;
use yew::prelude::*;

use crate::binding;

use crate::render_svg;

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

#[function_component(TotalUsersCard)]
fn total_users_card() -> Html {
    use_effect(move || {
        let document = window().unwrap().document().unwrap();

        let container = document.get_element_by_id("total-users-chart").unwrap();

        let container_width = container.client_width() as f64;
        let container_height = container.client_height() as f64;

        log!(container_height);
        log!(container_width);

        binding::totalUsersChart(container_width, container_height);
        || {}
    });

    html! {
        <div class=" rounded-xl border border-grey-shade-11 p-4 space-y-2">
            <div class="flex items-center justify-between space-x-5">
            <div class="flex justify-between items-center">
                <h1 class="text-16 font-400 leading-20 text-grey-shade-1">{"Total user"}</h1>
            </div>
            <div class="flex items-center justify-end space-x-2">
                <div class="flex items-center rounded border border-grey-shade-11 justify-start px-3 py-2" >
                    <label for="datepicker" class="text-12 text-grey-shade-3 font-300">{"From:"}</label>
                    <input id="datepicker" type="date" class=" outline-none text-12 text-grey-shade-3 font-300" />
                </div>
                <div class="flex items-center rounded border border-grey-shade-11 justify-start px-3 py-2" >
                    <label for="datepicker" class="text-12 text-grey-shade-3 font-300">{"To:"}</label>
                    <input id="datepicker" type="date" class=" outline-none text-12 text-grey-shade-3 font-300" />
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
            <div class="h-[360px] w-full"  id="total-users-chart">
            </div>
        </div>
    }
}

#[function_component(TotalVolumeCard)]
fn total_volume_card() -> Html {
    use_effect(|| {
        binding::createProfitLossChart();
        || {}
    });

    html! {
        <div class=" rounded-xl border border-grey-shade-11 p-4 space-y-2">
            <div class="flex items-center justify-between space-x-5">
                <div class="flex justify-between items-center">
                    <h1 class="text-16 font-400 leading-20 text-grey-shade-1">{"Total Volume & Profit"}</h1>
                </div>
                <div class="flex items-center justify-end space-x-1">
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start px-3 py-2" >
                        <label for="datepicker" class="text-12 text-grey-shade-3 font-300">{"From:"}</label>
                        <input id="datepicker" type="date" class=" outline-none text-12 text-grey-shade-3 font-300" />
                    </div>
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start px-3 py-2" >
                        <label for="datepicker" class="text-12 text-grey-shade-3 font-300">{"To:"}</label>
                        <input id="datepicker" type="date" class=" outline-none text-12 text-grey-shade-3 font-300" />
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
                    <h3 class="text-24 font-600 leading-32 text-primary">{"$210,691"}</h3>
                </div>
                <div class="flex flex-col">
                    <p class="text-14 font-400 leading-20 text-grey-shade-5">{"Total profile earned"}</p>
                    <h3 class="text-24 font-600 leading-32 text-success">{"$210,691"}</h3>
                </div>
            </div>
            <div class="h-[360px]" id="profit-loss-chart-container"></div>
        </div>
    }
}

// #[derive(Serialize, Deserialize)]
// struct CircleChartData {
//     label: String,
//     value: i32,
// }

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
        <div class=" rounded-xl border border-grey-shade-11 p-4 space-y-2">
            <div>
                <div>
                    <h1 class="text-16 font-400 leading-20 text-grey-shade-1">{"Top platform"}</h1>
                </div>
                    <div id="circle-chart-container"></div>
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
    use_effect(move || {
        binding::createTopupsCommissionChart();
        || {}
    });

    html! {
        <div class=" rounded-xl border border-grey-shade-11 p-4 space-y-2">
            <div class="flex items-center justify-between">
                <div class="flex justify-between items-center">
                    <h1 class="text-16 font-400 leading-20 text-grey-shade-1">{"Topups & Commissions earned"}</h1>
                </div>
                <div class="flex items-center justify-end space-x-1">
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start px-2 py-2" >
                        <label for="datepicker" class="text-12 text-grey-shade-3 font-300">{"From:"}</label>
                        <input id="datepicker" type="date" class=" outline-none text-12 text-grey-shade-3 font-300" />
                    </div>
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start px-2 py-2" >
                        <label for="datepicker" class="text-12 text-grey-shade-3 font-300">{"To:"}</label>
                        <input id="datepicker" type="date" class=" outline-none text-12 text-grey-shade-3 font-300" />
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
                    <h3 class="text-24 font-600 leading-32 text-primary">{"$210,691"}</h3>
                </div>
                <div class="flex flex-col">
                    <p class="text-14 font-400 leading-20 text-grey-shade-5">{"Total commission"}</p>
                    <h3 class="text-24 font-600 leading-32 text-success">{"$210,691"}</h3>
                </div>
            </div>

            <div id="topup-chart-container"></div>
        </div>
    }
}
