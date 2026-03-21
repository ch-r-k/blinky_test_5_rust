use crate::device_layer_abstraction::i_ui::IUi;

pub struct Blinky<UiType: IUi> {
    ui: UiType,
}

impl<UiType: IUi> Blinky<UiType> {
    pub fn new(ui: UiType) -> Self {
        Blinky { ui }
    }

    pub fn run(&self) {
        for _ in 0..9 {
            self.ui.set();
            self.ui.reset();
        }
    }
}
