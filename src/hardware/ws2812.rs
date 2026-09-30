use core::time::Duration;

use esp_idf_svc::hal::gpio::Gpio60;
use esp_idf_svc::hal::rmt::config::TxChannelConfig;
use esp_idf_svc::hal::rmt::encoder::{BytesEncoder, BytesEncoderConfig};
use esp_idf_svc::hal::rmt::{PinState, Symbol, TxChannelDriver};
use esp_idf_svc::hal::units::Hertz;
use esp_idf_svc::sys::EspError;

const RESOLUTION: Hertz = Hertz(10_000_000); // 1 tick = 0.1µs

pub(crate) struct WS2812<'a> {
    led: TxChannelDriver<'a>,
    ws2812: BytesEncoder,
}

impl<'a> WS2812<'a> {
    pub(crate) fn new(gpio60: Gpio60<'a>) -> Result<Self, EspError> {
        // Onboard WS2812 RGB LED
        let config = TxChannelConfig {
            resolution: RESOLUTION,
            ..Default::default()
        };
        let led = TxChannelDriver::new(gpio60, &config).unwrap();

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
        let ws2812 =  BytesEncoder::with_config(&BytesEncoderConfig {
            bit0: bit(350, 800),
            bit1: bit(700, 600),
            msb_first: true,
            ..Default::default()
        })?;
        Ok(Self { led, ws2812 })
    }
    
    pub(crate) fn set(&mut self, r: u8, g: u8, b: u8) -> Result<(), EspError> {
        // WS2812 expects GRB order
        self.led.send_and_wait(&mut self.ws2812, &[g, r, b], &Default::default())
    }
}
