use crate::qr_code::generate_qr_code;
use crate::render::{QrRenderer, TextQrRenderer};

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
    Ok(())
}