use crate::device_layer_abstraction::i_ui::IUi;
use crate::hardware_layer_abstraction::i_gpio::IGpio;

pub struct UserIndication<GpioType: IGpio> {
    gpio: GpioType,
}

impl<GpioType: IGpio> UserIndication<GpioType> {
    pub fn new(gpio: GpioType) -> Self {
        UserIndication { gpio }
    }
}

impl<GpioType: IGpio> IUi for UserIndication<GpioType> {
    fn set(&mut self) {
        self.gpio.set();
    }

    fn reset(&mut self) {
        self.gpio.reset();
    }
}
