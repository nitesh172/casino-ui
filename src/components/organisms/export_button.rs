use web_sys::{
    wasm_bindgen::{JsCast, JsValue},
    Blob, BlobPropertyBag, Url,
};
use yew::prelude::*;
use crate::render_svg;
use js_sys::Uint8Array;

#[derive(PartialEq, Properties)]
pub struct ExportButtonProps {
    pub export: Callback<MouseEvent>,
}

pub fn preprocess_csv_string(data: &str) -> String {
    // Split the CSV string into lines
    let lines: Vec<&str> = data.lines().collect();

    // Process each line to ensure all fields are enclosed in quotes
    let processed_lines: Vec<String> = lines
        .iter()
        .map(|line| {
            let mut processed_line = String::new();

            // Process each field in the line
            let fields: Vec<&str> = line.split('|').collect();
            for (i, field) in fields.iter().enumerate() {
                if i > 0 {
                    processed_line.push(',');
                }
                processed_line.push('"');
                processed_line.push_str(field);
                processed_line.push('"');
            }

            processed_line
        })
        .collect();
    // Join the processed lines back into a CSV string
    processed_lines.join("\n")
}

pub fn download_csv_file(csv_content: &str) {
    let csv_string = preprocess_csv_string(csv_content);

    let bytes = csv_string.as_bytes();

    // Create a Uint8Array from the bytes
    let uint8_array = js_sys::Uint8Array::new_with_length(bytes.len() as u32);
    uint8_array.copy_from(&bytes);

    // Create a Blob from the Uint8Array
    let blob = Blob::new_with_u8_array_sequence_and_options(
        &js_sys::Array::of1(&JsValue::from(<Uint8Array as AsRef<JsValue>>::as_ref(
            &uint8_array,
        ))),
        BlobPropertyBag::new().type_("text/csv;charset=utf-8"),
    )
    .expect("Failed to create Blob");

    let url = Url::create_object_url_with_blob(&blob).expect("Failed to create URL");

    // Create a link element for triggering the download
    let download_link = web_sys::window()
        .expect("No window")
        .document()
        .expect("No document")
        .create_element("a")
        .expect("Failed to create <a> element");

    // Set attributes for the download link
    download_link
        .set_attribute("href", &url)
        .expect("Failed to set href attribute");
    download_link
        .set_attribute("download", "exported_data.csv")
        .expect("Failed to set download attribute");

    // Trigger a click event on the link to start the download
    if let Some(html_element) = download_link.dyn_ref::<web_sys::HtmlElement>() {
        html_element.click();
    }

    // Revoke the URL to free up resources
    Url::revoke_object_url(&url).expect("Failed to revoke URL");
}

#[function_component(ExportButton)]
pub fn export_button(props: &ExportButtonProps) -> Html {
    

    html! {
        <button
            onclick={props.export.clone()}
            class="bg-grey-shade-0 flex flex-1 md:flex-none items-center justify-center md:justify-normal rounded p-2 text-grey-shade-14 text-12 font-400"
        >
            <span class="pr-2">
            {html! { render_svg!("majesticons:file", color="#ffff",width="14px")}}
            </span>
            {"Export"}
        </button>
    }
}
