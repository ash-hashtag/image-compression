use std::io::Cursor;

use image::{imageops::FilterType, Limits};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Target {
    width: u32,
    height: u32,
    filter: u8,
    quality: u8,
    max_alloc: u64,
    max_width: u32,
    max_height: u32,
}

#[wasm_bindgen]
impl Target {
    #[wasm_bindgen(constructor)]
    pub fn new(
        width: u32,
        height: u32,
        filter: u8,
        quality: u8,
        max_alloc: u64,
        max_width: u32,
        max_height: u32,
    ) -> Target {
        Self {
            width,
            height,
            filter,
            quality: quality.clamp(1, 100),
            max_alloc,
            max_width,
            max_height,
        }
    }
}

#[wasm_bindgen]
pub fn compress_image(original_image: Uint8Array, target: Target) -> Result<Uint8Array, JsValue> {
    let source_image = original_image.to_vec();

    let input_format =
        image::guess_format(&source_image).map_err(|err| JsValue::from_str(&err.to_string()))?;

    match input_format {
        image::ImageFormat::Gif | image::ImageFormat::WebP | image::ImageFormat::Avif => {
            return Ok(original_image);
        }
        _ => {}
    };

    let mut reader = image::ImageReader::new(Cursor::new(&source_image));
    let mut limits = Limits::default();
    limits.max_image_width = Some(target.max_width);
    limits.max_image_height = Some(target.max_height);
    limits.max_alloc = Some(target.max_alloc);
    reader.limits(limits);

    let mut input = reader
        .decode()
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    let filter = match target.filter {
        1 => FilterType::Triangle,
        2 => FilterType::CatmullRom,
        3 => FilterType::Gaussian,
        4 => FilterType::Lanczos3,
        _ => FilterType::Nearest,
    };

    let should_compress = target.width > 0
        && target.height > 0
        && (target.width < input.width() || target.height < input.height());

    if should_compress {
        input = input.resize(target.width, target.height, filter);
    }

    let mut output = source_image;

    output.clear();
    let writer = Cursor::new(&mut output);
    let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(writer, target.quality);
    input
        .write_with_encoder(encoder)
        .map_err(|err| JsValue::from_str(&err.to_string()))?;
    if original_image.length() < (output.len() as u32) {
        return Err(JsValue::from_str(
            "Final image is larger than original image",
        ));
    }

    let output_image = original_image.subarray(0, output.len() as u32);
    output_image.copy_from(&output);
    Ok(output_image)
}
