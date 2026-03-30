use crate::hardware_layer::gpio_input::GpioInput;
use crate::hardware_layer_abstraction::i_gpio_input::IGpioInput;

/// Device layer abstraction for user input (button)
pub struct UserInput<'d> {
    input: GpioInput<'d>,
}

impl<'d> UserInput<'d> {
    pub fn new(input: GpioInput<'d>) -> Self {
        Self { input }
    }

    /// Wait for a button press
    pub async fn wait_for_press(&mut self) {
        self.input.wait_for_press().await;
    }
}
