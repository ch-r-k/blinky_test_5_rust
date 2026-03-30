use crate::device_layer_abstraction::i_ui::IUi;
use crate::hardware_layer_abstraction::i_gpio::IGpioOutput;

pub struct UserIndication<GpioType: IGpioOutput> {
    gpio: GpioType,
}

impl<GpioType: IGpioOutput> UserIndication<GpioType> {
    pub fn new(gpio: GpioType) -> Self {
        UserIndication { gpio }
    }
}

impl<GpioType: IGpioOutput> IUi for UserIndication<GpioType> {
    async fn set(&mut self) {
        self.gpio.set().await;
    }

    async fn reset(&mut self) {
        self.gpio.reset().await;
    }
}
