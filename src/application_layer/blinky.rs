use crate::device_layer_abstraction::i_ui::IUi;
use cortex_m::asm::delay;

pub struct Blinky<UiType: IUi> {
    ui: UiType,
}

impl<UiType: IUi> Blinky<UiType> {
    pub fn new(ui: UiType) -> Self {
        Blinky { ui }
    }

    pub fn run(&mut self) {
        self.ui.set();
        delay(24_000_000);
        self.ui.reset();
        delay(24_000_000);
    }
}
