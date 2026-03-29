pub trait IGpio {
    async fn set(&mut self);
    async fn reset(&mut self);
}
