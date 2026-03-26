use crate::hardware_layer_abstraction::i_gpio::IGpio;
use embassy_rp::gpio::Output;

/// HAL-erased GPIO output (Embassy-native)
pub struct Gpio<'d> {
    pin: Output<'d>,
}

impl<'d> Gpio<'d> {
    pub fn new(pin: Output<'d>) -> Self {
        Self { pin }
    }
}

impl<'d> IGpio for Gpio<'d> {
    fn set(&mut self) {
        self.pin.set_high();
    }

    fn reset(&mut self) {
        self.pin.set_low();
    }
}