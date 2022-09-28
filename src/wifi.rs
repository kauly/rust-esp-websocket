// based on https://github.com/ferrous-systems/espressif-trainings/blob/main/common/lib/esp32-c3-dkc02-bsc/src/wifi.rs

use anyhow::bail;
use embedded_svc::wifi::{
    self, AuthMethod, ClientConfiguration, ClientConnectionStatus, ClientIpStatus, ClientStatus,
    Wifi as _,
};
use esp_idf_svc::{
    netif::EspNetifStack, nvs::EspDefaultNvs, sysloop::EspSysLoopStack, wifi::EspWifi,
};
use std::sync::Arc;
use std::time::Duration;

#[allow(unused)]
pub struct Wifi {
    esp_wifi: EspWifi,
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
}

pub fn wifi(ssid: &str, passwd: &str) -> anyhow::Result<Wifi> {
    let auth_method = AuthMethod::WPA2Personal;

    if ssid.is_empty() || passwd.is_empty() {
        bail!("Missing Wifi SSID or password")
    }

    let netif_stack = Arc::new(EspNetifStack::new()?);
    let sys_loop_stack = Arc::new(EspSysLoopStack::new()?);
    let default_nvs = Arc::new(EspDefaultNvs::new()?);

    let mut wifi = EspWifi::new(
        netif_stack.clone(),
        sys_loop_stack.clone(),
        default_nvs.clone(),
    )?;

    println!("Searching for Wifi network {}", ssid);

    let ap_printlns = wifi.scan()?;
    let ours = ap_printlns.into_iter().find(|a| a.ssid == ssid);

    let channel = if let Some(ours) = ours {
        println!(
            "Found configured access point {} on channel {}",
            ssid, ours.channel
        );
        Some(ours.channel)
    } else {
        println!(
            "Configured access point {} not found during scanning, will go with unknown channel",
            ssid
        );
        None
    };

    println!("setting Wifi configuration");
    wifi.set_configuration(&wifi::Configuration::Client(ClientConfiguration {
        ssid: ssid.into(),
        password: passwd.into(),
        channel,
        auth_method,
        ..Default::default()
    }))?;

    println!("getting Wifi status");

    wifi.wait_status_with_timeout(Duration::from_secs(2100), |status| {
        !status.is_transitional()
    })
    .map_err(|err| anyhow::anyhow!("Unexpected Wifi status (Transitional state): {:?}", err))?;

    let status = wifi.get_status();

    if let wifi::Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(
            _ip_settings,
        ))),
        _,
    ) = status
    {
        println!("Wifi connected, IP: {}", _ip_settings.ip);
    } else {
        bail!(
            "Could not connect to Wifi - Unexpected Wifi status: {:?}",
            status
        );
    }

    let wifi = Wifi {
        esp_wifi: wifi,
        netif_stack,
        sys_loop_stack,
        default_nvs,
    };

    Ok(wifi)
}
