use qrcode::QrCode;

struct WifiInfoBuilder {
    ssid: String,
    encryption: String,
    password: String,
}

impl WifiInfoBuilder {
    fn new() -> Self {
        Self {
            ssid: String::new(),
            encryption: String::new(),
            password: String::new(),
        }
    }

    fn ssid(mut self, ssid: &str) -> Self {
        self.ssid = ssid.to_string();
        self
    }

    fn encryption(mut self, encryption: &str) -> Self {
        self.encryption = encryption.to_string();
        self
    }

    fn password(mut self, password: &str) -> Self {
        self.password = password.to_string();
        self
    }

    fn build(self) -> Result<String, String> {
        if self.ssid.is_empty() || self.password.is_empty() {
            Err("SSID y contraseña no pueden estar vacíos".to_string())
        } else {
            Ok(format!("WIFI:S:{};T:{};P:{};;", self.ssid, self.encryption, self.password))
        }
    }
}
//
trait QrRenderer {
    fn render (&self, code: &QrCode)-> String;
}
struct TextQrRenderer;
Impl QrRenderer  for TextQrRenderer {
    fn render(&self, code:&QrCode) ->String {
    code.render::<char>()
        .quiet_zone(false)
        .module_dimensions(2,1)
        .build()
    }
}

fn generate_qr_code (data:  &str) -> Result<QrCode, String>{
    QrCode::new(data).map_err(|e|format!("Error to generate a QR code: {}", e))
}

fn main() {
    let ssid = "NAME-OF.RED";
    let encryption = "SECURYTT"; //Example WPA2
    let password = "NETWORK-PASSWORD"; 

    let wifi_info = match WifiInfoBuilder::new ()
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
    }

    let code = match generate_qr_code(&wifi_info){
        Ok(code) => code,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    }

    let renderer = TextQrRenderer;
    let qr_string = renderer.render(&code);
    print!("{}", qr_string);
}
