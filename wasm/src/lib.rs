#![allow(unused)]
use image::{
    imageops::{resize, FilterType},
    DynamicImage, GenericImageView, ImageBuffer, Rgba,
};
use CNNModel;
// use wasm_bindgen::prelude::*;
// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
pub fn find_similarity(img: DynamicImage) -> () {
    resize(&img, 224, 224, FilterType::Triangle);
    let (width, height) = img.dimensions();
    let mut normalized_buffer: Vec<Vec<[f32; 4]>> = Vec::with_capacity(height as usize);
    for n in 0..height {
        normalized_buffer.push(Vec::with_capacity(width as usize));
    }
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let [r, g, b, a] = pixel.0;
            let new_r = (r as f32 / 255.0);
            let new_g = (g as f32 / 255.0);
            let new_b = (b as f32 / 255.0);
            normalized_buffer[y as usize][x as usize] = [new_r, new_g, new_b, a as f32];
        }
    }
    let im = DataSetImage {
        data: normalized_buffer,
        label: "smtng".to_string(),
    };
}

struct DataSetImage {
    data: Vec<Vec<[f32; 4]>>,
    label: String,
}
