use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/js/chart.js")]
extern "C" {
    #[wasm_bindgen(js_name = "totalUsersChart")]
    pub fn totalUsersChart(containerWidth: f64, containerHeight: f64);
}

#[wasm_bindgen(module = "/src/js/chart.js")]
extern "C" {
    #[wasm_bindgen(js_name = "createProfitLossChart")]
    pub fn createProfitLossChart();
}

#[wasm_bindgen(module = "/src/js/chart.js")]
extern "C" {
    #[wasm_bindgen(js_name = "createCircleChart")]
    pub fn createCircleChart();
}

#[wasm_bindgen(module = "/src/js/chart.js")]
extern "C" {
    #[wasm_bindgen(js_name = "createTopupsCommissionChart")]
    pub fn createTopupsCommissionChart();
}
