use crate::device_layer_abstraction::i_ui::IUi;
use embassy_time::{Duration, Timer};

pub struct Blinky<UiType: IUi> {
    ui: UiType,
}

impl<UiType: IUi> Blinky<UiType> {
    pub fn new(ui: UiType) -> Self {
        Blinky { ui }
    }

    pub async fn run(&mut self) {
        self.ui.set().await;
        Timer::after(Duration::from_millis(500)).await;

        self.ui.reset().await;
        Timer::after(Duration::from_millis(500)).await;
    }
}
