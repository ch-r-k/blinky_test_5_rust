use crate::application_layer::blinky_control::BlinkyControl;
use crate::device_layer::user_input::UserInput;
use embassy_executor::task;

/// Button monitoring task - listens for button presses and toggles blinky
#[task]
pub async fn button_task(mut input: UserInput<'static>) {
    loop {
        // Wait for button press (falling edge)
        input.wait_for_press().await;

        // Simple debounce delay
        embassy_time::Timer::after(embassy_time::Duration::from_millis(20)).await;

        // Toggle blinky state via independent control interface
        BlinkyControl::toggle().await;

        // Wait for button release before accepting next press
        embassy_time::Timer::after(embassy_time::Duration::from_millis(200)).await;
    }
}
