use core::time::Duration;

use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::rmt::config::TxChannelConfig;
use esp_idf_svc::hal::rmt::encoder::{BytesEncoder, BytesEncoderConfig};
use esp_idf_svc::hal::rmt::{PinState, Symbol, TxChannelDriver};
use esp_idf_svc::hal::units::Hertz;

const RESOLUTION: Hertz = Hertz(10_000_000); // 1 tick = 0.1µs

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    // Onboard WS2812 RGB LED
    let config = TxChannelConfig {
        resolution: RESOLUTION,
        ..Default::default()
    };
    let led = TxChannelDriver::new(peripherals.pins.gpio60, &config).unwrap();

    // WS2812 bit timings (high, low) from the datasheet
    let bit = |high, low| {
        Symbol::new_with(
            RESOLUTION,
            PinState::High,
            Duration::from_nanos(high),
            PinState::Low,
            Duration::from_nanos(low),
        )
        .unwrap()
    };
    let ws2812 = BytesEncoder::with_config(&BytesEncoderConfig {
        bit0: bit(350, 800),
        bit1: bit(700, 600),
        msb_first: true,
        ..Default::default()
    })
    .unwrap();

    spawner.spawn(cycle_colors(led, ws2812).unwrap());
}

#[embassy_executor::task]
async fn cycle_colors(mut led: TxChannelDriver<'static>, mut ws2812: BytesEncoder) {
    for (r, g, b) in [(16, 0, 0), (0, 16, 0), (0, 0, 16)].into_iter().cycle() {
        log::info!("LED: ({r}, {g}, {b})");
        // WS2812 expects GRB order
        led.send_and_wait(&mut ws2812, &[g, r, b], &Default::default())
            .unwrap();
        Timer::after_secs(1).await;
    }
}
