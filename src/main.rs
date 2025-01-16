mod qr_code;
mod render;
mod wifi;
mod utils;

use qr_code::generate_qr_code;
use render::{QrRenderer, TextQrRenderer};
use utils::input::prompt_user;
use wifi::builder::WifiInfoBuilder;

fn prompt_for_repeat() -> bool {
    let response = prompt_user("Would you like to generate another QR code? (y/n):");
    response.trim().to_lowercase() == "y"
}

fn get_wifi_info () -> Result<String, String>{

    let ssid = prompt_user("Enter the network name (SSID):");
    let encryption = prompt_user("Enter the security system (WEP/WPA/WPA2):");
    let password = prompt_user("Enter the network password:");

    let wifi_info = WifiInfoBuilder::new()
        .ssid(&ssid)
        .encryption(&encryption)
        .password(&password)
        .build();

    wifi_info
}

fn generate_and_display_qr_code(wifi_info: &str) -> Result<(), String>{
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

fn main() {
    loop {
        match get_wifi_info(){
            Ok(wifi_info) => {
                if let Err(err) = generate_and_display_qr_code(&wifi_info){
                    eprintln!("Error generating QR code: {}", err);
                }
            }
            Err(err) => eprintln!("Error: {}",err),
        }
        if !prompt_for_repeat() {
            break;
        }
    }
}
