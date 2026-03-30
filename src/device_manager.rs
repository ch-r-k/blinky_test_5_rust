use crate::device_layer::user_indication::UserIndication2;
use crate::device_layer::user_input::UserInput;
use crate::hardware_layer::smart_led_bus::PioSmartLedBus;
use crate::hardware_manager::HardwareResources;
use embassy_rp::peripherals::PIO0;

/// Device Manager - manages device layer abstractions
pub struct DeviceManager {}

impl DeviceManager {
    /// Initialize device layer components
    pub fn init(hardware: HardwareResources) -> DeviceResources {
        let HardwareResources {
            gpio_output,
            gpio_input,
            led_bus,
        } = hardware;

        // Create device abstractions
        //let user_indication = UserIndication::new(gpio_output);
        let user_indication_2 = UserIndication2::new(led_bus);
        let user_input = UserInput::new(gpio_input);

        DeviceResources {
            //user_indication,
            user_indication_2,
            user_input,
        }
    }
}

/// Device resources bundle
pub struct DeviceResources {
    //pub user_indication: UserIndication,
    pub user_indication_2: UserIndication2<PioSmartLedBus<'static, PIO0, 0>>,
    pub user_input: UserInput<'static>,
}
