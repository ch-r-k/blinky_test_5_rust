use crate::hardware_layer_abstraction::i_smart_led_bus::ISmartLedBus;
use crate::hardware_layer_abstraction::i_smart_led_bus::Rgb;
use embassy_rp::pio::Instance;
use embassy_rp::pio_programs::ws2812::{Grb, PioWs2812};
use smart_leds::RGB8;

pub struct PioSmartLedBus<'d, P: Instance, const SM: usize> {
    driver: PioWs2812<'d, P, SM, 64, Grb>,
}

impl<'d, P: Instance, const SM: usize> PioSmartLedBus<'d, P, SM> {
    pub fn new(driver: PioWs2812<'d, P, SM, 64, Grb>) -> Self {
        Self { driver }
    }
}
impl<'d, P: Instance, const SM: usize> ISmartLedBus for PioSmartLedBus<'d, P, SM> {
    async fn write(&mut self, data: &[Rgb]) {
        // simple fixed buffer (adjust size as needed)
        let mut buf = [RGB8 { r: 0, g: 0, b: 0 }; 64];

        for (i, c) in data.iter().enumerate() {
            if i >= 64 {
                break;
            }
            buf[i] = RGB8 {
                r: c.red,
                g: c.green,
                b: c.blue,
            };
        }

        self.driver.write(&buf).await;
    }
}
