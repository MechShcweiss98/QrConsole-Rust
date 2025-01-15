use crate::setter;

pub struct WifiInfoBuilder {
    ssid: String,
    encryption: String,
    password: String,
}

impl WifiInfoBuilder {
    pub fn new() -> Self {
        Self {
            ssid: String::new(),
            encryption: String::new(),
            password: String::new(),
        }
    }

    pub fn build(self) -> Result<String, String> {
        if self.ssid.is_empty() || self.password.is_empty() {
            Err("SSID and password are required".to_string())
        } else {
            Ok(format!(
                "WIFI:S:{};T:{};P:{};;",
                self.ssid, self.encryption, self.password
            ))
        }
    }
}
setter!(WifiInfoBuilder, ssid, ssid);
setter!(WifiInfoBuilder, encryption, encryption);
setter!(WifiInfoBuilder, password, password);

