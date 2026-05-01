#[derive(Clone, Copy)]
pub enum BlinkyEvent {
    Toggle,
    Timeout { timer_id: u8, generation: u32 },
}

#[derive(Clone, Copy)]
pub enum ButtonEvent {
    Press,
    Timeout { timer_id: u8, generation: u32 },
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TimerRecipient {
    Blinky,
    Button,
}

#[derive(Clone, Copy)]
pub enum TimerCommand {
    Arm {
        recipient: TimerRecipient,
        timer_id: u8,
        generation: u32,
        delay_ms: u32,
    },
    Disarm {
        recipient: TimerRecipient,
        timer_id: u8,
    },
}