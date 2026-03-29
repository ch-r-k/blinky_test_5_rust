use crate::application_layer::blinky::Blinky;
use crate::device_layer::ui::UserIndication;
use crate::device_layer::ui_2::UserIndication2;
use crate::hardware_layer::gpio::Gpio;
use crate::hardware_layer::smart_led_bus::PioSmartLedBus;
use embassy_rp::Peripherals;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::pio::Pio;
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => embassy_rp::pio::InterruptHandler<embassy_rp::peripherals::PIO0>;
    DMA_IRQ_0 => embassy_rp::dma::InterruptHandler<embassy_rp::peripherals::DMA_CH0>;
});

pub struct SystemManager {}

impl SystemManager {
    /// Run the system (sync, because Blinky is sync)
    pub async fn run(p: Peripherals) {
        // led gpio
        let led = Output::new(p.PIN_25, Level::Low);
        let gpio = Gpio::new(led);

        // led strip via pio

        let Pio {
            mut common, sm0, ..
        } = Pio::new(p.PIO0, Irqs);

        let program = PioWs2812Program::new(&mut common);
        let ws2812 = PioWs2812::new(&mut common, sm0, p.DMA_CH0, Irqs, p.PIN_1, &program);

        let led_bus = PioSmartLedBus::new(ws2812);

        // ui
        let _ui = UserIndication::new(gpio);
        let ui2 = UserIndication2::new(led_bus);

        // ui
        let mut blinky = Blinky::new(ui2);

        loop {
            blinky.run().await;
        }
    }
}
