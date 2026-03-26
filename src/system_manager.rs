use crate::application_layer::blinky::Blinky;
use crate::device_layer::ui::UserIndication;
use crate::hardware_layer::gpio::Gpio;

use embassy_rp::gpio::{Level, Output};
use embassy_rp::Peripherals;

pub struct SystemManager {
}

impl SystemManager {
    /// Run the system (sync, because Blinky is sync)
    pub fn run(p: Peripherals) {
        let led: Output<'_> = Output::new(p.PIN_25, Level::Low);

        let gpio = Gpio::new(led);
        let ui = UserIndication::new(gpio);
        let mut blinky = Blinky::new(ui);

        loop {
            blinky.run();
        }
    }
}
