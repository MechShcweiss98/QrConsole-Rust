use crate::qr_code::generate_qr_code;
use crate::render::{QrRenderer, TextQrRenderer};
use crate::utils::input::prompt_user;
use crate::utils::generate_image::save_qr_to_file;

pub fn generate_and_display_qr_code(wifi_info: &str) -> Result<(), String>{

    let code = match generate_qr_code(&wifi_info) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("{}", err);
            return Err(err);
        }
    };

    let renderer = TextQrRenderer;
    let qr_string = renderer.render(&code);
    print!("{}", qr_string);

    let save_qr = prompt_user("Would you like to save the QR code as a PNG file? (y/n)");
    if save_qr.trim().to_lowercase() =="y"{
        let file_path = prompt_user("Enter the file name to save the QR code (e.g., wifi_qr.png):");
        let scale_factor: u32 = prompt_user("Enter a scale factor (e.g., 10x for 10x size):").parse().unwrap_or(10);


        if let Err(err) = save_qr_to_file(wifi_info, &file_path, scale_factor){
            eprintln!("Error saving QR code to file: {}", err);
        }else {
            println!("QR code saved successfully to{}", file_path);
        }
    }
    Ok(())
}