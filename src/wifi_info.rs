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
    pub fn ssid(mut self, ssid: &str) -> Self {
        self.ssid = ssid.to_string();
        self
    }

    pub fn encryption(mut self, encryption: &str) -> Self {
        self.encryption = encryption.to_string();
        self
    }
    pub fn password(mut self, password: &str) -> Self {
        self.password = password.to_string();
        self
    }

    pub fn build(self) -> Result<String, String> {
        if self.ssid.is_empty() || self.password.is_empty() {
            Err("SSID and password are required".to_string())
        } else {
            Ok(format!("WIFI:S:{};T:{};P:{};;", self.ssid, self.encryption, self.password))
        }
    }
}
