use crate::application_layer::blinky_control::BlinkyControl;
use crate::application_layer::blinky_task::blinky_task;
use crate::application_layer::button_task::button_task;
use crate::device_manager::DeviceResources;
use embassy_executor::Spawner;

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
            embassy_time::Timer::after(embassy_time::Duration::from_secs(1)).await;
        }
    }
}
