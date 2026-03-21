use crate::hardware_layer_abstraction::i_gpio::IGpio;
use embedded_hal::digital::v2::OutputPin;

/// Erase the HAL pin type; any OutputPin works.
pub struct Gpio<PIN>
where
    PIN: OutputPin,
{
    pin: PIN,
}

impl<PIN> Gpio<PIN>
where
    PIN: OutputPin,
{
    pub fn from_pin(pin: PIN) -> Self {
        Gpio { pin }
    }
}

impl<PIN> IGpio for Gpio<PIN>
where
    PIN: OutputPin,
{
    fn set(&mut self) {
        let _ = self.pin.set_high();
    }

    fn reset(&mut self) {
        let _ = self.pin.set_low();
    }
}
