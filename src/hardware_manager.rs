use crate::hardware_layer::gpio_input::GpioInput;
use crate::hardware_layer::gpio_output::GpioOutput;
use crate::hardware_layer::smart_led_bus::PioSmartLedBus;
use embassy_rp::Peripherals;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::Pio;
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => embassy_rp::pio::InterruptHandler<embassy_rp::peripherals::PIO0>;
    DMA_IRQ_0 => embassy_rp::dma::InterruptHandler<embassy_rp::peripherals::DMA_CH0>;
});

/// Hardware Manager - manages hardware peripherals
pub struct HardwareManager {}

impl HardwareManager {
    /// Initialize hardware peripherals
    pub fn init(p: Peripherals) -> HardwareResources {
        // Initialize LED output
        let led = Output::new(p.PIN_25, Level::Low);
        let gpio_output = GpioOutput::new(led);

        // Initialize button input
        let button = Input::new(p.PIN_10, Pull::Up);
        let gpio_input = GpioInput::new(button);

        // Initialize LED bus (WS2812)
        let Pio {
            mut common, sm0, ..
        } = Pio::new(p.PIO0, Irqs);

        let program = PioWs2812Program::new(&mut common);
        let ws2812 = PioWs2812::new(&mut common, sm0, p.DMA_CH0, Irqs, p.PIN_1, &program);
        let led_bus = PioSmartLedBus::new(ws2812);

        HardwareResources {
            gpio_output,
            gpio_input,
            led_bus,
        }
    }
}

/// Hardware resources bundle
pub struct HardwareResources {
    pub gpio_output: GpioOutput<'static>,
    pub gpio_input: GpioInput<'static>,
    pub led_bus: PioSmartLedBus<'static, PIO0, 0>,
}
