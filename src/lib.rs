use base64::{decode, encode};
use chrono::Utc;
use image::{load_from_memory, ImageOutputFormat};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn greyscale(encoded_file: &str) -> String {
    let msg = format!("Grayscale called {}", Utc::now());
    log(&msg.into());

    let base64_to_vector = decode(encoded_file).unwrap();
    let msg = format!("Image decoded {}", Utc::now());
    log(&msg.into());

    let img = load_from_memory(&base64_to_vector).unwrap();
    let msg = format!("Loaded image into ImageResult {}", Utc::now());
    log(&msg.into());

    let img = img.grayscale();
    let msg = format!("Greyscale effect applied {}", Utc::now());
    log(&msg.into());

    let mut buffer = vec![];
    img.write_to(&mut buffer, ImageOutputFormat::Png).unwrap();
    let msg = format!("New image written {}", Utc::now());
    log(&msg.into());

    let encoded_result = encode(buffer);
    let msg = format!("Encoded into base64 {}", Utc::now());
    log(&msg.into());

    let data_url = format!("data:image/png;base64,{}", encoded_result);

    return data_url;
}
