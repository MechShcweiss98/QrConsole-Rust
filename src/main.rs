mod qr_code;
mod render;
mod wifi;
mod utils;

use qr_code::generate_qr_code;
use render::{QrRenderer, TextQrRenderer};
use utils::input::prompt_user;
use wifi::builder::WifiInfoBuilder;

fn main() {
    let ssid = prompt_user("Enter the network name (SSID):");
    let encryption = prompt_user("Enter the security system (WEP/WPA/WPA2):");
    let password = prompt_user("Enter the network password:");

    let wifi_info = match WifiInfoBuilder::new()
        .ssid(&ssid)
        .encryption(&encryption)
        .password(&password)
        .build()
    {
        Ok(info) => info,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let code = match generate_qr_code(&wifi_info) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let renderer = TextQrRenderer;
    let qr_string = renderer.render(&code);
    print!("{}", qr_string);
}