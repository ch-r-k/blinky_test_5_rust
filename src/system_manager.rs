use crate::application_layer::blinky::Blinky;
use crate::device_layer::ui::UserIndication;
use crate::hardware_layer::gpio::Gpio;

pub struct SystemManager 
{
    blinky: Blinky<UserIndication<Gpio>>,
}

impl SystemManager 
{
    pub fn new() -> Self {
        let gpio = Gpio::new(13);
        let ui = UserIndication::new(gpio);
        let blinky = Blinky::new(ui);

        Self { blinky }
    }

    pub fn run(&self) 
    {
        self.blinky.run();
    }
}