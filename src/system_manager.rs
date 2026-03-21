use crate::application_layer::blinky::Blinky;
use crate::device_layer::ui::UserIndication;
use crate::hardware_layer::gpio::Gpio;

use rp_pico::hal::{
    gpio::{FunctionSio, Pin, Pins, PullDown, SioOutput, bank0::Gpio25},
    pac,
    sio::Sio,
};

type LedPin = Pin<Gpio25, FunctionSio<SioOutput>, PullDown>;

/// Now we don’t care about the exact Pull type
pub struct SystemManager {
    blinky: Blinky<UserIndication<Gpio<LedPin>>>,
}

impl SystemManager {
    pub fn new() -> Self {
        let mut pac = pac::Peripherals::take().unwrap();
        let sio = Sio::new(pac.SIO);

        let pins = Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        let led_pin = pins.gpio25.into_push_pull_output();

        let gpio = Gpio::from_pin(led_pin);
        let ui = UserIndication::new(gpio);
        let blinky = Blinky::new(ui);

        Self { blinky }
    }

    pub fn run(&mut self) {
        self.blinky.run();
    }
}
