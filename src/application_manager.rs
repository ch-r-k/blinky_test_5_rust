use crate::application_layer::blinky::Blinky;
use crate::application_layer::button::{Button, ButtonIrqCallback};
use crate::framework::event::ButtonEvent;
use crate::framework::message_bus::BusSender;
use crate::device_manager::DeviceUserIndication;

pub type AppBlinky = Blinky<DeviceUserIndication>;
pub type AppButton = Button;
pub type AppButtonIrqCallback = ButtonIrqCallback;

pub struct ApplicationResources {
    pub blinky: AppBlinky,
    pub button_ao: AppButton,
    pub button_irq_callback: AppButtonIrqCallback,
}

pub struct ApplicationManager;

impl ApplicationManager {
    /// Build application-level objects from device-layer resources.
    pub fn init(
        user_indication: DeviceUserIndication,
        button_event_sender: BusSender<'static, ButtonEvent, 8>,
    ) -> ApplicationResources {
        ApplicationResources {
            blinky: Blinky::new(user_indication),
            button_ao: Button::new(),
            button_irq_callback: ButtonIrqCallback::new(button_event_sender),
        }
    }
}
