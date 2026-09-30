use core::convert::TryInto;

use embedded_svc::{
    wifi::{self, AuthMethod},
};

use esp_idf_svc::{hal::modem::Modem, timer::EspTaskTimerService, wifi::ClientConfiguration};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    wifi::{AsyncWifi, EspWifi},
};

use log::*;

const SSID: &str = env!("WIFI_SSID");
const PASSWORD: &str = env!("WIFI_PASS");

pub async fn setup(modem: Modem<'static>) -> anyhow::Result<AsyncWifi<EspWifi<'static>>> {
    let sys_loop = EspSystemEventLoop::take()?;
    let timer_service = EspTaskTimerService::new()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = AsyncWifi::wrap(
        EspWifi::new(modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
        timer_service,
    )?;

    connect_wifi(&mut wifi).await?;

    Ok(wifi)
}

async fn connect_wifi(wifi: &mut AsyncWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let wifi_configuration = wifi::Configuration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD.try_into().unwrap(),
        ..Default::default()
    });
    wifi.set_configuration(&wifi_configuration)?;
    wifi.start().await?;
    info!("Wifi started");
    wifi.connect().await?;
    info!("Wifi connected to {SSID}");
    wifi.wait_netif_up().await?;
    info!("Wifi IP: {}", wifi.wifi().sta_netif().get_ip_info()?.ip);
    Ok(())
}
