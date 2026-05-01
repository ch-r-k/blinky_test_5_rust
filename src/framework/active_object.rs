pub trait ActiveObject<Event, Effect> {
    fn handle_event(&mut self, event: Event) -> Option<Effect>;
}