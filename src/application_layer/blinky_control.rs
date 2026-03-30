use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;

/// Events for controlling blinky state
#[derive(Clone, Copy)]
pub enum BlinkyControlEvent {
    Start,
    Stop,
    Toggle,
}

/// Global channel for blinky control
static BLINKY_CONTROL_CH: Channel<CriticalSectionRawMutex, BlinkyControlEvent, 4> = Channel::new();

/// Global interface for controlling blinky state
pub struct BlinkyControl;

impl BlinkyControl {
    /// Send start command
    pub async fn start() {
        BLINKY_CONTROL_CH.send(BlinkyControlEvent::Start).await;
    }

    /// Send stop command
    pub async fn stop() {
        BLINKY_CONTROL_CH.send(BlinkyControlEvent::Stop).await;
    }

    /// Send toggle command
    pub async fn toggle() {
        BLINKY_CONTROL_CH.send(BlinkyControlEvent::Toggle).await;
    }

    /// Try to receive a control event (non-blocking)
    pub fn try_receive() -> Option<BlinkyControlEvent> {
        BLINKY_CONTROL_CH.try_receive().ok()
    }
}
