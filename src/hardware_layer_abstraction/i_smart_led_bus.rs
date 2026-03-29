#[derive(Copy, Clone, Default)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub trait ISmartLedBus {
    async fn write(&mut self, data: &[Rgb]);
}
