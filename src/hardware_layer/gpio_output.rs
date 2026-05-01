use crate::hardware_layer_abstraction::i_gpio_output::IGpioOutput;
use embedded_hal::digital::OutputPin;
use rp2040_hal::gpio::{FunctionSio, Pin, PinId, PullType, SioOutput};

pub struct GpioOutput<I: PinId, P: PullType> {
    pin: Pin<I, FunctionSio<SioOutput>, P>,
}

impl<I: PinId, P: PullType> GpioOutput<I, P>
where
    Pin<I, FunctionSio<SioOutput>, P>: OutputPin,
{
    pub fn new(pin: Pin<I, FunctionSio<SioOutput>, P>) -> Self {
        Self { pin }
    }
}

impl<I: PinId, P: PullType> IGpioOutput for GpioOutput<I, P>
where
    Pin<I, FunctionSio<SioOutput>, P>: OutputPin,
{
    fn set(&mut self) {
        let _ = self.pin.set_high();
    }

    fn reset(&mut self) {
        let _ = self.pin.set_low();
    }
}
