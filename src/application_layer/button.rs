use crate::device_layer_abstraction::icb_user_input_press::IcbUserInput;
use crate::framework::active_object::ActiveObject;
use crate::framework::event::{BlinkyEvent, ButtonEvent, TimerCommand, TimerRecipient};
use crate::framework::message_bus::{try_publish, BusSender};

const BUTTON_DEBOUNCE_MS: u32 = 30;
const BUTTON_TIMER_ID: u8 = 0;

#[derive(Clone, Copy)]
enum ButtonState {
    Idle,
    Debouncing,
}

pub enum ButtonEffect {
    TimerCommand(TimerCommand),
    BlinkyEvent(BlinkyEvent),
}

/// Button active object handling debouncing and semantic event generation.
pub struct Button {
    state: ButtonState,
    generation: u32,
}

impl Button {
    pub fn new() -> Self {
        Self {
            state: ButtonState::Idle,
            generation: 0,
        }
    }

    fn arm_debounce(&mut self) -> TimerCommand {
        self.generation = self.generation.wrapping_add(1);

        TimerCommand::Arm {
            recipient: TimerRecipient::Button,
            timer_id: BUTTON_TIMER_ID,
            generation: self.generation,
            delay_ms: BUTTON_DEBOUNCE_MS,
        }
    }

    pub fn on_event(&mut self, event: ButtonEvent) -> Option<ButtonEffect> {
        match event {
            ButtonEvent::Press => match self.state {
                ButtonState::Idle => {
                    self.state = ButtonState::Debouncing;
                    Some(ButtonEffect::TimerCommand(self.arm_debounce()))
                }
                ButtonState::Debouncing => None,
            },
            ButtonEvent::Timeout {
                timer_id,
                generation,
            } if timer_id == BUTTON_TIMER_ID && generation == self.generation => {
                self.state = ButtonState::Idle;
                Some(ButtonEffect::BlinkyEvent(BlinkyEvent::Toggle))
            }
            ButtonEvent::Timeout { .. } => None,
        }
    }
}

impl ActiveObject<ButtonEvent, ButtonEffect> for Button {
    fn handle_event(&mut self, event: ButtonEvent) -> Option<ButtonEffect> {
        self.on_event(event)
    }
}

/// IRQ callback publisher: converts hardware button press to Button AO input events.
pub struct ButtonIrqCallback {
    event_sender: BusSender<'static, ButtonEvent, 8>,
}

impl ButtonIrqCallback {
    pub fn new(event_sender: BusSender<'static, ButtonEvent, 8>) -> Self {
        Self { event_sender }
    }

    pub fn on_user_input_press(&mut self) {
        let _ = try_publish(&mut self.event_sender, ButtonEvent::Press);
    }
}

impl IcbUserInput for ButtonIrqCallback {
    fn on_user_input_press(&mut self) {
        ButtonIrqCallback::on_user_input_press(self);
    }
}