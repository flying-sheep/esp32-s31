mod hardware;

use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_idf_svc::hal::peripherals::Peripherals;

use crate::hardware::WS2812;


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    
    let ws2812 = WS2812::new(peripherals.pins.gpio60).unwrap();
    spawner.spawn(cycle_colors(ws2812).unwrap());
}

#[embassy_executor::task]
async fn cycle_colors(mut ws2812: WS2812<'static>) {
    for (r, g, b) in [(16, 0, 0), (0, 16, 0), (0, 0, 16)].into_iter().cycle() {
        log::info!("LED: ({r}, {g}, {b})");
        ws2812.set(r, g, b).unwrap();
        Timer::after_secs(1).await;
    }
}
