use crate::application_layer::blinky_task::BlinkyHandle;
use crate::hardware_layer::gpio_input::GpioInput;
use crate::hardware_layer_abstraction::i_gpio_input::IGpioInput;
use embassy_executor::task;

/// Button monitoring task - listens for button presses and toggles blinky
#[task]
pub async fn button_task(mut button: GpioInput<'static>) {
    let blinky = BlinkyHandle;
    let mut is_running = false;

    loop {
        // Wait for button press (falling edge)
        button.wait_for_press().await;

        // Simple debounce delay
        embassy_time::Timer::after(embassy_time::Duration::from_millis(20)).await;

        // Toggle blinky state
        if is_running {
            blinky.stop().await;
            is_running = false;
        } else {
            blinky.start().await;
            is_running = true;
        }

        // Wait for button release before accepting next press
        embassy_time::Timer::after(embassy_time::Duration::from_millis(200)).await;
    }
}
