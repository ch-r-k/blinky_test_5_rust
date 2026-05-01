use crate::hardware_layer::gpio_input::GpioInput;
use crate::hardware_layer::gpio_output::GpioOutput;
use rp2040_hal::clocks::init_clocks_and_plls;
use rp2040_hal::gpio::bank0::{Gpio10, Gpio25};
use rp2040_hal::gpio::{Interrupt, Pins, PullDown, PullUp};
use rp2040_hal::pac;
use rp2040_hal::Sio;
use rp2040_hal::Watchdog;

/// Hardware resources bundle (LED + button, no LED strip)
pub struct HardwareResources {
    pub gpio_output: GpioOutput<Gpio25, PullDown>,
    pub gpio_input: GpioInput<Gpio10, PullUp>,
}

pub struct HardwareManager;

impl HardwareManager {
    /// Initialize hardware peripherals using rp2040-hal.
    /// NOTE: `pac.TIMER` must be passed to `Mono::start` before calling this
    /// (in the RTIC init function), so it is excluded from the arguments here.
    pub fn init(
        watchdog_periph: pac::WATCHDOG,
        xosc: pac::XOSC,
        clocks_block: pac::CLOCKS,
        pll_sys: pac::PLL_SYS,
        pll_usb: pac::PLL_USB,
        mut resets: pac::RESETS,
        sio_periph: pac::SIO,
        io_bank0: pac::IO_BANK0,
        pads_bank0: pac::PADS_BANK0,
    ) -> HardwareResources {
        let mut watchdog = Watchdog::new(watchdog_periph);

        let _clocks = init_clocks_and_plls(
            12_000_000u32,
            xosc,
            clocks_block,
            pll_sys,
            pll_usb,
            &mut resets,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        let sio = Sio::new(sio_periph);
        let pins = Pins::new(io_bank0, pads_bank0, sio.gpio_bank0, &mut resets);

        let led = pins.gpio25.into_push_pull_output();
        let gpio_output = GpioOutput::new(led);

        let button = pins.gpio10.into_pull_up_input();
        button.set_interrupt_enabled(Interrupt::EdgeLow, true);
        let gpio_input = GpioInput::new(button);

        HardwareResources {
            gpio_output,
            gpio_input,
        }
    }
}
