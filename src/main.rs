mod qr_code;
mod render;
mod utils;
mod wifi;

use utils::{
    generate::generate_and_display_qr_code, get_info::get_wifi_info, input::prompt_user,
    loops::prompt_for_repeat, ascii::ascii_print
};
fn main() {
    ascii_print();
    loop {
        match get_wifi_info() {
            Ok(wifi_info) => {
                if let Err(err) = generate_and_display_qr_code(&wifi_info) {
                    eprintln!("Error generating QR code: {}", err);
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        }
        if !prompt_for_repeat() {
            break;
        }
    }
}
