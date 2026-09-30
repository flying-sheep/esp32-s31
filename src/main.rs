mod hardware;
mod services;

use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_idf_svc::{hal::peripherals::Peripherals, wifi::{AsyncWifi, EspWifi}};

use crate::hardware::WS2812;


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    run(spawner).await.unwrap();
}

async fn run(spawner: Spawner) -> anyhow::Result<()> {
    let peripherals = Peripherals::take()?;
    
    let wifi = hardware::wifi::setup(peripherals.modem).await?;
    spawner.spawn(http_server(wifi)?);
    
    let ws2812 = WS2812::new(peripherals.pins.gpio60)?;
    spawner.spawn(cycle_colors(ws2812)?);
    
    Ok(())
}

#[embassy_executor::task]
async fn http_server(_wifi: AsyncWifi<EspWifi<'static>>) {
    let _server = services::http_server::server().unwrap();
    core::future::pending::<()>().await; // keep `_wifi` and `_server` alive forever
}

#[embassy_executor::task]
async fn cycle_colors(mut ws2812: WS2812<'static>) {
    for (r, g, b) in [(16, 0, 0), (0, 16, 0), (0, 0, 16)].into_iter().cycle() {
        ws2812.set(r, g, b).unwrap();
        Timer::after_secs(1).await;
    }
}
