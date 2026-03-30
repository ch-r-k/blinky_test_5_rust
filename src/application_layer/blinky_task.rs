use crate::application_layer::blinky::Blinky;
use crate::device_layer::ui_2::UserIndication2;
use crate::hardware_layer::smart_led_bus::PioSmartLedBus;
use embassy_executor::task;
use embassy_rp::peripherals::PIO0;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;

/// Events
#[derive(Clone, Copy)]
pub enum BlinkyEvent {
    Start,
    Stop,
}

/// Event queue
static BLINKY_CH: Channel<CriticalSectionRawMutex, BlinkyEvent, 4> = Channel::new();

/// Handle to post events
pub struct BlinkyHandle;

impl BlinkyHandle {
    pub async fn start(&self) {
        BLINKY_CH.send(BlinkyEvent::Start).await;
    }

    pub async fn stop(&self) {
        BLINKY_CH.send(BlinkyEvent::Stop).await;
    }
}

/// Blinky task - handles LED blinking based on events
#[task]
pub async fn blinky_task(ui: UserIndication2<PioSmartLedBus<'static, PIO0, 0>>) {
    let mut blinky = Blinky::new(ui);

    loop {
        // Handle events
        if let Ok(evt) = BLINKY_CH.try_receive() {
            match evt {
                BlinkyEvent::Start => blinky.start(),
                BlinkyEvent::Stop => blinky.stop(),
            }
        }

        // Run one step
        blinky.step().await;
    }
}
