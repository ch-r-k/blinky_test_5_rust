use crate::application_layer::blinky_control::BlinkyControl;
use crate::device_layer::user_input::UserInput;
use embassy_time::Duration;

/// Button handler that manages button press detection and debouncing
pub struct Button<'d> {
    input: UserInput<'d>,
}

impl<'d> Button<'d> {
    pub fn new(input: UserInput<'d>) -> Self {
        Self { input }
    }

    /// Handle one button press cycle (wait for press, debounce, toggle, wait for release)
    pub async fn handle_press(&mut self) {
        // Wait for button press (falling edge)
        self.input.wait_for_press().await;

        // Simple debounce delay
        embassy_time::Timer::after(Duration::from_millis(20)).await;

        // Toggle blinky state via independent control interface
        BlinkyControl::toggle().await;

        // Wait for button release before accepting next press
        embassy_time::Timer::after(Duration::from_millis(200)).await;
    }
}
