use crate::hardware_layer_abstraction::i_gpio_input::IGpioInput;
use crate::hardware_layer_abstraction::icb_gpio_input::IcbGpioInput;
use rp2040_hal::gpio::{FunctionSio, Interrupt, Pin, PinId, PullType, SioInput};

pub struct GpioInput<I: PinId, P: PullType> {
    pin: Pin<I, FunctionSio<SioInput>, P>,
}

impl<I: PinId, P: PullType> GpioInput<I, P> {
    pub fn new(pin: Pin<I, FunctionSio<SioInput>, P>) -> Self {
        Self { pin }
    }
}

impl<I: PinId, P: PullType> IGpioInput for GpioInput<I, P> {
    fn handle_irq<C: IcbGpioInput>(&mut self, callback: &mut C) {
        if self.pin.interrupt_status(Interrupt::EdgeLow) {
            self.pin.clear_interrupt(Interrupt::EdgeLow);
            callback.on_press();
        }
    }
}
