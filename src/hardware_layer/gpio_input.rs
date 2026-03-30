use crate::hardware_layer_abstraction::i_gpio_input::IGpioInput;
use embassy_rp::gpio::Input;

/// HAL-erased GPIO input (Embassy-native)
pub struct GpioInput<'d> {
    pin: Input<'d>,
}

impl<'d> GpioInput<'d> {
    pub fn new(pin: Input<'d>) -> Self {
        Self { pin }
    }
}

impl<'d> IGpioInput for GpioInput<'d> {
    async fn wait_for_press(&mut self) {
        self.pin.wait_for_falling_edge().await;
    }
}
