pub trait IGpioOutput {
    fn set(&mut self);
    fn reset(&mut self);
}
