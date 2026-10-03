use core::time::Duration;
use std::ops::Mul;

use esp_idf_svc::hal::gpio::Gpio60;
use esp_idf_svc::hal::rmt::config::TxChannelConfig;
use esp_idf_svc::hal::rmt::encoder::{BytesEncoder, BytesEncoderConfig};
use esp_idf_svc::hal::rmt::{PinState, Symbol, TxChannelDriver};
use esp_idf_svc::hal::units::Hertz;
use esp_idf_svc::sys::EspError;

const RESOLUTION: Hertz = Hertz(10_000_000); // 1 tick = 0.1µs

/// Ratio betwen 0 and 1
#[derive(Debug, Clone, Copy)]
pub(crate) struct Ratio8(pub u8);

impl Mul<Ratio8> for u8 {
    type Output = u8;

    fn mul(self, rhs: Ratio8) -> Self::Output {
        // Ratio8(255) == 1.0; round to nearest. Max is (255*255 + 127) / 255 = 255, so no overflow
        ((u16::from(self) * u16::from(rhs.0) + 127) / 255) as u8
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Grb8(pub [u8; 3]);

impl Grb8 {
    pub(crate) fn interpolate(self, other: Grb8, frac: Ratio8) -> Self {
        // a*(1-t) + b*t: no underflow when other < self, and the sum stays ≤ 255
        let inv = Ratio8(255 - frac.0);
        Self(std::array::from_fn(|i| self.0[i] * inv + other.0[i] * frac))
    }
}

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
        let ws2812 = BytesEncoder::with_config(&BytesEncoderConfig {
            bit0: bit(350, 800),
            bit1: bit(700, 600),
            msb_first: true,
            ..Default::default()
        })?;
        Ok(Self { led, ws2812 })
    }

    pub(crate) fn set(&mut self, c: Grb8) -> Result<(), EspError> {
        // WS2812 expects GRB order
        self.led
            .send_and_wait(&mut self.ws2812, &c.0, &Default::default())
    }
}
