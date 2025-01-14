use qrcode::QrCode;

fn generate_qr_code (data:  &str) -> Result<QrCode, String>{
    QrCode::new(data).map_err(|e|format!("Error to generate a QR code: {}", e))
}

fn render_qr_code_to_string(code:&QrCode) ->String {
    code.render::<char>()
        .quiet_zone(false)
        .module_dimensions(2,1)
        .build()
}

fn main() {
    let ssid = "NAME-OF.RED";
    let encryption = "SECURYTT"; //Example WPA2
    let password = "NETWORK-PASSWORD"; 

    if ssid.is_empty() || pasasword.is_empty(){
        eprintln!("SSID and password are required");
        return;
    }


    let wifi_info = format!("WIFI:S:{};T:{};P:{};;", ssid, encryption, password);

    let code = match generate_qr_code(&wifi_info){
        Ok(code) => code,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    }

    let qr_string = render_qr_code_to_string(&code);
    print!("{}", qr_string);
}
