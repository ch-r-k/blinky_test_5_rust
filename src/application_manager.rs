use crate::application_layer::blinky::Blinky;
use crate::application_layer::blinky_control::{BlinkyControl, BlinkyControlEvent};
use crate::application_layer::button::Button;
use crate::device_layer::user_indication::UserIndication2;
use crate::device_layer::user_input::UserInput;
use crate::device_manager::DeviceResources;
use crate::hardware_layer::smart_led_bus::PioSmartLedBus;
use embassy_executor::Spawner;
use embassy_executor::task;
use embassy_rp::peripherals::PIO0;
use embassy_time::Duration;

/// Blinky task - handles LED blinking based on control events
#[task]
async fn blinky_task(ui: UserIndication2<PioSmartLedBus<'static, PIO0, 0>>) {
    let mut blinky = Blinky::new(ui);

    loop {
        // Handle control events
        if let Some(event) = BlinkyControl::try_receive() {
            match event {
                BlinkyControlEvent::Start => blinky.start(),
                BlinkyControlEvent::Stop => blinky.stop(),
                BlinkyControlEvent::Toggle => blinky.toggle(),
            }
        }

        // Run one step
        blinky.step().await;
    }
}

/// Button monitoring task - delegates button handling to Button struct
#[task]
async fn button_task(input: UserInput<'static>) {
    let mut button = Button::new(input);

    loop {
        button.handle_press().await;
    }
}

/// Application Manager - manages application logic and tasks
pub struct ApplicationManager {}

impl ApplicationManager {
    /// Initialize application components and spawn tasks
    pub async fn init_and_run(device: DeviceResources, spawner: Spawner) {
        let DeviceResources {
            user_indication_2,
            user_input,
        } = device;

        // Spawn tasks - they create their own application objects internally
        spawner.spawn(blinky_task(user_indication_2).unwrap());
        spawner.spawn(button_task(user_input).unwrap());

        // Start the blinky
        BlinkyControl::start().await;

        // Application loop (could be extended for more logic)
        loop {
            embassy_time::Timer::after(Duration::from_secs(1)).await;
        }
    }
}
