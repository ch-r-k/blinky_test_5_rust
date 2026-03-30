use crate::application_layer::blinky::Blinky;
use crate::application_layer::blinky_control::{BlinkyControl, BlinkyControlEvent};
use crate::device_layer::user_indication::UserIndication2;
use crate::hardware_layer::smart_led_bus::PioSmartLedBus;
use embassy_executor::task;
use embassy_rp::peripherals::PIO0;

/// Blinky task - handles LED blinking based on control events
#[task]
pub async fn blinky_task(ui: UserIndication2<PioSmartLedBus<'static, PIO0, 0>>) {
    let mut blinky = Blinky::new(ui);

    loop {
        // Handle control events
        if let Some(event) = BlinkyControl::try_receive() {
            match event {
                BlinkyControlEvent::Start => blinky.start(),
                BlinkyControlEvent::Stop => blinky.stop(),
                BlinkyControlEvent::Toggle => {
                    if blinky.is_running() {
                        blinky.stop();
                    } else {
                        blinky.start();
                    }
                }
            }
        }

        // Run one step
        blinky.step().await;
    }
}
