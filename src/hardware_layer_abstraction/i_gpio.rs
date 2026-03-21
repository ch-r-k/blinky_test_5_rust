pub trait IGpio {
    fn set(&mut self);
    fn reset(&mut self);
}
