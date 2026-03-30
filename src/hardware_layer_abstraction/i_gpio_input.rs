pub trait IGpioInput {
    async fn wait_for_press(&mut self);
}
