use web_sys::{
    wasm_bindgen::{JsCast, JsValue},
    window, Document, HtmlElement,
};
use yew::{prelude::*, virtual_dom::VNode};

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

// const WIDTH: f32 = 533.0;
// const HEIGHT: f32 = 300.0;
// const MARGIN: f32 = 50.0;
// const TICK_LENGTH: f32 = 10.0;

#[function_component(TotalUsersCard)]
fn total_users_card() -> Html {
    // let end_date = Utc::now();
    // let start_date = end_date.sub(Duration::days(4));
    // let timespan = start_date..end_date;

    // let circle_text_labeller = Rc::from(series::circle_text_label("Label")) as Rc<dyn Labeller>;

    // let data_set = Rc::new(vec![
    //     (start_date.timestamp_millis(), 1.0, None),
    //     (
    //         start_date.add(Duration::days(1)).timestamp_millis(),
    //         4.0,
    //         None,
    //     ),
    //     (
    //         start_date.add(Duration::days(2)).timestamp_millis(),
    //         3.0,
    //         None,
    //     ),
    //     (
    //         start_date.add(Duration::days(3)).timestamp_millis(),
    //         2.0,
    //         None,
    //     ),
    //     (
    //         start_date.add(Duration::days(4)).timestamp_millis(),
    //         5.0,
    //         Some(circle_text_labeller),
    //     ),
    // ]);

    // let h_scale = Rc::new(TimeScale::new(timespan, Duration::days(1))) as Rc<dyn Scale<Scalar = _>>;
    // let v_scale = Rc::new(LinearScale::new(0.0..5.0, 1.0)) as Rc<dyn Scale<Scalar = _>>;

    // let tooltip = Rc::from(series::y_tooltip()) as Rc<dyn Tooltipper<_, _>>;
    let js_code = JsValue::from_str(include_str!("../../../js/chart.js"));
    let js_string = js_code.as_string().unwrap();

    // Create a new div element
    let div = window()
        .unwrap()
        .document()
        .expect("Window not found")
        .create_element("div")
        .unwrap();
    div.set_inner_html(&js_string);

    // Convert the div element to HtmlElement
    let html_code: HtmlElement = div.unchecked_into();

    html! {
        <>
        <div id="chart-container">
            {VNode::VRef(html_code.into())}
            // <script>{ html_code }</script>
        </div>

        </>
        // <div class=" rounded-xl border border-grey-shade-11 p-4 space-y-2">
        //     <div class="flex items-center justify-between space-x-5">
        //     <div class="flex justify-between items-center">
        //         <h1 class="text-16 font-400 leading-20 text-grey-shade-1">{"Total user"}</h1>
        //     </div>
        //     <div class="flex items-center justify-end space-x-2">
        //         <div class="flex items-center rounded border border-grey-shade-11 justify-start px-3 py-2" >
        //             <label for="datepicker" class="text-12 text-grey-shade-3 font-300">{"From:"}</label>
        //             <input id="datepicker" type="date" class=" outline-none text-12 text-grey-shade-3 font-300" />
        //         </div>
        //         <div class="flex items-center rounded border border-grey-shade-11 justify-start px-3 py-2" >
        //             <label for="datepicker" class="text-12 text-grey-shade-3 font-300">{"To:"}</label>
        //             <input id="datepicker" type="date" class=" outline-none text-12 text-grey-shade-3 font-300" />
        //         </div>
        //         <button
        //             class="bg-grey-shade-0 flex items-center rounded p-2 text-grey-shade-14 text-12 font-400 "
        //         >
        //             <span class="pr-2">
        //                 {html! { render_svg!("majesticons:file", color="#ffff",width="14px")}}
        //             </span>
        //             {"Export"}
        //         </button>
        //     </div>
        //     </div>
        //     <div class="h-[360px]">
        //         <svg class="chart" viewBox={format!("0 0 {} {}", WIDTH, HEIGHT)} preserveAspectRatio="none">
        //                 <Series<i64, f32>
        //                     series_type={Type::Line}
        //                     name="some-series"
        //                     data={data_set}
        //                     horizontal_scale={Rc::clone(&h_scale)}
        //                     horizontal_scale_step={Duration::days(2).num_milliseconds()}
        //                     tooltipper={Rc::clone(&tooltip)}
        //                     vertical_scale={Rc::clone(&v_scale)}
        //                     x={MARGIN} y={MARGIN} width={WIDTH - (MARGIN * 2.0)} height={HEIGHT - (MARGIN * 2.0)} />

        //                 <Axis<f32>
        //                     name="some-y-axis"
        //                     orientation={Orientation::Left}
        //                     scale={Rc::clone(&v_scale)}
        //                     x1={MARGIN} y1={MARGIN} xy2={HEIGHT - MARGIN}
        //                     tick_len={TICK_LENGTH}
        //                     // title={"Some Y thing".to_string()}
        //                 />

        //                 <Axis<i64>
        //                     name="some-x-axis"
        //                     orientation={Orientation::Bottom}
        //                     scale={Rc::clone(&h_scale)}
        //                     x1={MARGIN} y1={HEIGHT - MARGIN} xy2={WIDTH - MARGIN}
        //                     tick_len={TICK_LENGTH}
        //                     // title={"Some X thing".to_string()}
        //                 />

        //         </svg>
        //     </div>
        // </div>
    }
}

#[function_component(TotalVolumeCard)]
fn total_volume_card() -> Html {
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
            <div class="h-[360px]"></div>
        </div>
    }
}

#[function_component(TopPlatformCard)]
fn top_platform_card() -> Html {
    html! {
        <div class=" rounded-xl border border-grey-shade-11 p-4 space-y-2">
            <div class="flex items-start justify-start ">
                <div class="flex justify-between items-center">
                    <h1 class="text-16 font-400 leading-20 text-grey-shade-1">{"Top platform"}</h1>
                </div>
            </div>
            <div class="h-[360px]"></div>
        </div>
    }
}

#[function_component(TopupsAndCommissionCard)]
fn topups_commision_card() -> Html {
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

            <div class="h-[360px]"></div>
        </div>
    }
}
