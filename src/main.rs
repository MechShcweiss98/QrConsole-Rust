use image::Luma;
use qrcode::QrCode;

fn main() {
    let ssid = "NAME-OF.RED";
    let encryption = "SECURYTT"; //Example WPA2
    let password = "NETWORK-PASSWORD"; 

    let wifi_info = format!("WIFI:S:{};T:{};P:{};;", ssid, encryption, password);

    let code = QrCode::new(wifi_info).expect("Cannot generate the QR code");

    let string = code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();

    print!("{}", string);
}
