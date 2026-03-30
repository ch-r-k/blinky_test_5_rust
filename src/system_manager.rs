use crate::application_layer::blinky_task::{BlinkyHandle, blinky_task};
use crate::application_layer::button_task::button_task;
use crate::device_layer::ui::UserIndication;
use crate::device_layer::ui_2::UserIndication2;
use crate::hardware_layer::gpio::GpioOutput;
use crate::hardware_layer::gpio_input::GpioInput;
use crate::hardware_layer::smart_led_bus::PioSmartLedBus;
use embassy_executor::Spawner;
use embassy_rp::Peripherals;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::pio::Pio;
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => embassy_rp::pio::InterruptHandler<embassy_rp::peripherals::PIO0>;
    DMA_IRQ_0 => embassy_rp::dma::InterruptHandler<embassy_rp::peripherals::DMA_CH0>;
});

pub struct SystemManager {}

impl SystemManager {
    /// Run the system with separate blinky and button tasks
    pub async fn run(p: Peripherals, spawner: Spawner) {
        // --- hardware setup ---
        let led = Output::new(p.PIN_25, Level::Low);
        let gpio = GpioOutput::new(led);

        let Pio {
            mut common, sm0, ..
        } = Pio::new(p.PIO0, Irqs);

        let program = PioWs2812Program::new(&mut common);
        let ws2812 = PioWs2812::new(&mut common, sm0, p.DMA_CH0, Irqs, p.PIN_1, &program);
        let led_bus = PioSmartLedBus::new(ws2812);

        let _ui = UserIndication::new(gpio);
        let ui2 = UserIndication2::new(led_bus);

        // --- spawn blinky task ---
        spawner.spawn(blinky_task(ui2).unwrap());

        // --- button setup (GPIO20 - change PIN_XX as needed) ---
        let button = Input::new(p.PIN_10, Pull::Up);
        let button_gpio = GpioInput::new(button);

        // --- spawn button task ---
        spawner.spawn(button_task(button_gpio).unwrap());

        let blinky = BlinkyHandle;

        // --- start it ---
        blinky.start().await;

        // --- system loop ---
        loop {
            embassy_time::Timer::after(embassy_time::Duration::from_secs(1)).await;
        }
    }
}
