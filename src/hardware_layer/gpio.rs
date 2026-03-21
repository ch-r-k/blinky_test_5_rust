use crate::hardware_layer_abstraction::i_gpio::IGpio;

pub struct Gpio {
    pin: u8,
}

// Implement methods for it

impl Gpio {
    // Constructor-like function
    pub fn new(pin: u8) -> Self {
        Gpio { pin }
    }
}

impl IGpio for Gpio {
    // A method
    fn set(&self) {
        println!("Pin {}: [x]", self.pin);
    }

    fn reset(&self) {
        println!("Pin {}: [ ]", self.pin);
    }
}
