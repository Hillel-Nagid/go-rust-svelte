#![allow(unused)]

use image::{
    imageops::{resize, FilterType},
    DynamicImage,
};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// #[wasm_bindgen]
pub fn find_similarity(img: DynamicImage) -> () {
    resize(&img, 224, 224, FilterType::Triangle);
    let im = DataSetImage {
        data: img.as_bytes().into(),
        label: "smtng".to_string(),
    };
}

struct DataSetImage {
    data: Vec<u8>,
    label: String,
}
