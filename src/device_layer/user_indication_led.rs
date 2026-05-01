use crate::device_layer_abstraction::i_ui::IUi;
use crate::hardware_layer_abstraction::i_gpio_output::IGpioOutput;

pub struct UserIndication<GpioType: IGpioOutput> {
    gpio: GpioType,
}

impl<GpioType: IGpioOutput> UserIndication<GpioType> {
    pub fn new(gpio: GpioType) -> Self {
        UserIndication { gpio }
    }
}

impl<GpioType: IGpioOutput> IUi for UserIndication<GpioType> {
    fn set(&mut self) {
        self.gpio.set();
    }

    fn reset(&mut self) {
        self.gpio.reset();
    }
}
