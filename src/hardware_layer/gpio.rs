use crate::hardware_layer_abstraction::i_gpio_output::IGpioOutput;
use embassy_rp::gpio::Output;

/// HAL-erased GPIO output (Embassy-native)
pub struct GpioOutput<'d> {
    pin: Output<'d>,
}

impl<'d> GpioOutput<'d> {
    pub fn new(pin: Output<'d>) -> Self {
        Self { pin }
    }
}

impl<'d> IGpioOutput for GpioOutput<'d> {
    async fn set(&mut self) {
        self.pin.set_high();
    }

    async fn reset(&mut self) {
        self.pin.set_low();
    }
}
