use crate::device_layer_abstraction::i_ui::IUi;
use embassy_time::{Duration, Timer};

pub struct Blinky<'a, UiG: IUi, UiL: IUi> {
    ui_gpio: &'a mut UiG,
    ui_led_bus: &'a mut UiL,
    running: bool,
}

impl<'a, UiG: IUi, UiL: IUi> Blinky<'a, UiG, UiL> {
    pub fn new(ui_gpio: &'a mut UiG, ui_led_bus: &'a mut UiL) -> Self {
        Self {
            ui_gpio,
            ui_led_bus,
            running: false,
        }
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn toggle(&mut self) {
        if self.running {
            self.stop();
        } else {
            self.start();
        }
    }

    pub async fn step(&mut self) {
        if self.running {
            // Turn all UIs ON

            self.ui_gpio.set().await;
            self.ui_led_bus.set().await;

            Timer::after(Duration::from_millis(500)).await;

            // Turn all UIs OFF
            self.ui_gpio.reset().await;
            self.ui_led_bus.reset().await;

            Timer::after(Duration::from_millis(500)).await;
        } else {
            // idle behavior (optional)
            Timer::after(Duration::from_millis(100)).await;
        }
    }
}
