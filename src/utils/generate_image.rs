use qrcode::types::Color;

use image::{DynamicImage, ImageBuffer, Luma};
// use std::fs::File;
use std::path::Path;
use crate::qr_code::generate_qr_code;

pub fn save_qr_to_file(data: &str, file_path: &str, scale_factor: u32) -> Result<(), String> {

    let code = match generate_qr_code(data) {
        Ok(code) => code,
        Err(e) => return  Err(format!("Error to generating QR code: {}",e)),
    };

    let size: u32 = code.width().try_into().unwrap();

    let image = ImageBuffer::<Luma<u8>, Vec<u8>>::from_fn(size, size, |x, y|{
        if code[(x as usize, y as usize)] == Color::Dark {
            Luma([0u8])  // Black Color
        } else {
            Luma([255u8])  // White Color
        }
    });
    
    let scale_image = image::imageops::resize(
        &DynamicImage::ImageLuma8(image),
        size * scale_factor,
        size * scale_factor,
        image::imageops::FilterType::Lanczos3
        );

    let path = Path::new(file_path);

    if let Err(e) = scale_image.save(path){
        return Err(format!("Error saving QR code to file: {}",e));
    }
    Ok(())

}