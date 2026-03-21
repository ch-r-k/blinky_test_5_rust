use crate::device_layer_abstraction::i_ui::IUi;
use crate::hardware_layer_abstraction::i_gpio::IGpio;

pub struct UserIndication<GpioType: IGpio> {
    gpio: GpioType,
}

impl<G: IGpio> UserIndication<G> {
    pub fn new(gpio: G) -> Self {
        UserIndication { gpio }
    }
}

impl<G: IGpio> IUi for UserIndication<G> {
    fn set(&mut self) {
        self.gpio.set();
    }

    fn reset(&mut self) {
        self.gpio.reset();
    }
}
