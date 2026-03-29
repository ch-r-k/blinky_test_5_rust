pub trait IUi {
    async fn set(&mut self);
    async fn reset(&mut self);
}
