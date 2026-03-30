pub trait IGpioOutput {
    async fn set(&mut self);
    async fn reset(&mut self);
}
