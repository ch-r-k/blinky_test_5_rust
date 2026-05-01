use crate::device_layer::user_indication_led::UserIndication;
use crate::device_layer::user_input::UserInput;
use crate::hardware_layer::gpio_input::GpioInput;
use crate::hardware_layer::gpio_output::GpioOutput;
use crate::hardware_manager::HardwareResources;
use rp2040_hal::gpio::bank0::{Gpio10, Gpio25};
use rp2040_hal::gpio::{PullDown, PullUp};

pub type DeviceUserIndication = UserIndication<GpioOutput<Gpio25, PullDown>>;
pub type DeviceGpioInput = GpioInput<Gpio10, PullUp>;
pub type DeviceUserInput = UserInput<GpioInput<Gpio10, PullUp>>;

/// Device resources bundle
pub struct DeviceResources {
    pub user_indication: DeviceUserIndication,
    pub user_input: DeviceUserInput,
}

pub struct DeviceManager;

impl DeviceManager {
    pub fn init(hardware: HardwareResources) -> DeviceResources {
        let HardwareResources {
            gpio_output,
            gpio_input,
        } = hardware;

        DeviceResources {
            user_indication: UserIndication::new(gpio_output),
            user_input: UserInput::new(gpio_input),
        }
    }
}
