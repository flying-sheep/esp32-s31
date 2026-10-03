#![feature(float_conversions)]

mod hardware;
mod services;

use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_idf_svc::{
    hal::peripherals::Peripherals,
    wifi::{AsyncWifi, EspWifi},
};
use itertools::Itertools as _;

use crate::hardware::{Grb8, Ratio8, WS2812};

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
    let steps = interpolate_sine(256, 0., 256.);
    let colors: Vec<Grb8> = [Grb8([16, 0, 0]), Grb8([0, 16, 0]), Grb8([0, 0, 16])]
        .into_iter()
        .circular_tuple_windows()
        .flat_map(|(c0, c1)| {
            steps
                .iter()
                .copied()
                .map(move |frac| c0.interpolate(c1, frac))
        })
        .collect();

    for color in colors.into_iter().cycle() {
        ws2812.set(color).unwrap();
        Timer::after_millis(10).await;
    }
}

fn interpolate_sine(steps: u32, start: f64, end: f64) -> Vec<Ratio8> {
    let mut results = Vec::new();
    for i in 0..steps {
        // interpolation factor
        let t = i as f64 / steps as f64;
        let sine_value = (t * std::f64::consts::PI / 2.0).sin();
        let v = start + sine_value * (end - start);
        results.push(Ratio8(v.to_int_saturating()));
    }
    results
}
