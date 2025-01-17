
use crate::prompt_user;
use crate::wifi::builder::WifiInfoBuilder;

pub fn get_wifi_info () -> Result<String, String>{

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