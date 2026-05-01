use crate::hardware_layer_abstraction::icb_gpio_input::IcbGpioInput;

pub trait IGpioInput {
    fn handle_irq<C: IcbGpioInput>(&mut self, callback: &mut C);
}
