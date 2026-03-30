use crate::device_layer_abstraction::i_ui::IUi;
use embassy_time::{Duration, Timer};

pub struct Blinky<Ui: IUi> {
    ui: Ui,
    running: bool,
}

impl<Ui: IUi> Blinky<Ui> {
    pub fn new(ui: Ui) -> Self {
        Self { ui, running: false }
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub async fn step(&mut self) {
        if self.running {
            self.ui.set().await;
            Timer::after(Duration::from_millis(500)).await;

            self.ui.reset().await;
            Timer::after(Duration::from_millis(500)).await;
        } else {
            // idle behavior (optional)
            Timer::after(Duration::from_millis(100)).await;
        }
    }
}
