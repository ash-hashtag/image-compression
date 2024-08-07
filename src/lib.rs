use std::io::Cursor;

use image::imageops::FilterType;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Target {
    width: u32,
    height: u32,
    filter: u8,
    format: u8,
    quality: u8,
    speed: u8,
}

#[wasm_bindgen]
impl Target {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, filter: u8, format: u8, quality: u8, speed: u8) -> Target {
        Self {
            width,
            height,
            filter,
            format,
            quality: quality.clamp(1, 100),
            speed: speed.clamp(1, 10),
        }
    }
}

#[wasm_bindgen]
pub fn compress_image(original_image: Uint8Array, target: Target) -> Result<Uint8Array, JsValue> {
    let source_image = original_image.to_vec();

    let input_format =
        image::guess_format(&source_image).map_err(|err| JsValue::from_str(&err.to_string()))?;

    match input_format {
        image::ImageFormat::Gif 
        | image::ImageFormat::WebP 
        | image::ImageFormat::Avif => {
            return Ok(original_image);
        }
        _ => {}
    };

    let mut input = image::load_from_memory_with_format(&source_image, input_format)
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
    let format = match target.format {
        1 => image::ImageFormat::WebP,
        2 => image::ImageFormat::Avif,
        _ => image::ImageFormat::Jpeg,
    };

    output.clear();
    let writer = Cursor::new(&mut output);

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
        // image::ImageFormat::Avif => {
        //     let encoder = image::codecs::avif::AvifEncoder::new_with_speed_quality(
        //         writer,
        //         target.speed,
        //         target.quality,
        //     );
        //     input.write_with_encoder(encoder)
        // }
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
