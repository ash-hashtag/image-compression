use std::io::Cursor;

use image::{imageops::FilterType, Limits};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Target {
    width: u32,
    height: u32,
    filter: u8,
    format: u8,
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
        format: u8,
        quality: u8,
        max_alloc: u64,
        max_width: u32,
        max_height: u32,
    ) -> Target {
        Self {
            width,
            height,
            filter,
            format,
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

    let has_alpha_channel = match input.color() {
        image::ColorType::L8
        | image::ColorType::Rgb8
        | image::ColorType::L16
        | image::ColorType::Rgb16
        | image::ColorType::Rgb32F => false,
        image::ColorType::La8
        | image::ColorType::La16
        | image::ColorType::Rgba8
        | image::ColorType::Rgba16
        | image::ColorType::Rgba32F => true,
        _ => false,
    };

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
    let mut format = match target.format {
        0 => {
            if has_alpha_channel && target.quality >= 100 {
                image::ImageFormat::WebP
            } else {
                image::ImageFormat::Jpeg
            }
        }
        1 => image::ImageFormat::WebP,
        _ => image::ImageFormat::Jpeg,
    };

    output.clear();
    let writer = Cursor::new(&mut output);

    if !has_alpha_channel {
        format = image::ImageFormat::Jpeg;
    }

    let result = match format {
        image::ImageFormat::Jpeg => {
            let encoder =
                image::codecs::jpeg::JpegEncoder::new_with_quality(writer, target.quality);
            input.write_with_encoder(encoder)
        }
        image::ImageFormat::WebP => {
            let encoder = image::codecs::webp::WebPEncoder::new_lossless(writer);
            input.write_with_encoder(encoder)
        }
        _ => {
            return Err(JsValue::from_str("Unsupported output encoder"));
        }
    };

    result.map_err(|err| JsValue::from_str(&err.to_string()))?;

    if original_image.length() < (output.len() as u32) {
        return Err(JsValue::from_str(
            "Final image is larger than original image",
        ));
    }

    let output_image = original_image.subarray(0, output.len() as u32);

    output_image.copy_from(&output);

    Ok(output_image)
}
