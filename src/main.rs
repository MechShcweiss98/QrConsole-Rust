mod qr_code;
mod render;
mod wifi;

use qr_code::generate_qr_code;
use render::{QrRenderer, TextQrRenderer};
use wifi::builder::WifiInfoBuilder;

fn main() {
    let ssid = "NAME-OF.RED";
    let encryption = "SECURITY"; //Example WPA2
    let password = "NETWORK-PASSWORD";

    let wifi_info = match WifiInfoBuilder::new()
        .ssid(ssid)
        .encryption(encryption)
        .password(password)
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
