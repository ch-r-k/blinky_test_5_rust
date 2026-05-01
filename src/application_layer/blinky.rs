use crate::device_layer_abstraction::i_ui::IUi;
use crate::framework::active_object::ActiveObject;
use rust_fsm::state_machine;

use crate::framework::event::{BlinkyEvent, TimerCommand, TimerRecipient};

state_machine! {
    blinky_fsm(LedOff)

    LedOff(Timeout) => LedOn [TurnLedOn],
    LedOn(Timeout) => LedOff [TurnLedOff],

    LedOff(Toggle) => Stopped [EnterStopped],
    LedOn(Toggle) => Stopped [EnterStopped],
    Stopped(Toggle) => LedOff [EnterRunning],

    Stopped(Timeout) => Stopped
}

const BLINK_PERIOD_MS: u32 = 500;
const BLINK_TIMER_ID: u8 = 0;

/// Blinky owns the UI and exposes simple led_on/led_off primitives.
/// Timing is driven by external timeout events provided by the timer service.
pub struct Blinky<UiG: IUi> {
    ui: UiG,
    fsm: blinky_fsm::StateMachine,
    blink_generation: u32,
}

impl<UiG: IUi> Blinky<UiG> {
    pub fn new(ui: UiG) -> Self {
        Self {
            ui,
            fsm: blinky_fsm::StateMachine::new(),
            blink_generation: 0,
        }
    }

    pub fn arm_initial_timeout(&mut self) -> TimerCommand {
        self.arm_blink_timeout(0)
    }

    pub fn is_running(&self) -> bool {
        !matches!(self.fsm.state(), blinky_fsm::State::Stopped)
    }

    pub fn led_on(&mut self) {
        self.ui.set();
    }

    pub fn led_off(&mut self) {
        self.ui.reset();
    }

    pub fn on_event(&mut self, event: BlinkyEvent) -> Option<TimerCommand> {
        match event {
            BlinkyEvent::Toggle => self.dispatch(blinky_fsm::Input::Toggle),
            BlinkyEvent::Timeout {
                timer_id,
                generation,
            } if timer_id == BLINK_TIMER_ID && generation == self.blink_generation => {
                self.dispatch(blinky_fsm::Input::Timeout)
            }
            BlinkyEvent::Timeout { .. } => None,
        }
    }

    fn arm_blink_timeout(&mut self, delay_ms: u32) -> TimerCommand {
        self.blink_generation = self.blink_generation.wrapping_add(1);

        TimerCommand::Arm {
            recipient: TimerRecipient::Blinky,
            timer_id: BLINK_TIMER_ID,
            generation: self.blink_generation,
            delay_ms,
        }
    }

    fn disarm_blink_timeout(&mut self) -> TimerCommand {
        self.blink_generation = self.blink_generation.wrapping_add(1);

        TimerCommand::Disarm {
            recipient: TimerRecipient::Blinky,
            timer_id: BLINK_TIMER_ID,
        }
    }

    fn dispatch(&mut self, input: blinky_fsm::Input) -> Option<TimerCommand> {
        if let Ok(output) = self.fsm.consume(&input) {
            if let Some(output) = output {
                return self.apply_output(output);
            }
        }

        None
    }

    fn apply_output(&mut self, output: blinky_fsm::Output) -> Option<TimerCommand> {
        match output {
            blinky_fsm::Output::TurnLedOn => {
                self.led_on();
                Some(self.arm_blink_timeout(BLINK_PERIOD_MS))
            }
            blinky_fsm::Output::TurnLedOff => {
                self.led_off();
                Some(self.arm_blink_timeout(BLINK_PERIOD_MS))
            }
            blinky_fsm::Output::EnterStopped => {
                self.led_off();
                Some(self.disarm_blink_timeout())
            }
            blinky_fsm::Output::EnterRunning => {
                self.led_off();
                Some(self.arm_blink_timeout(0))
            }
        }
    }
}

impl<UiG: IUi> ActiveObject<BlinkyEvent, TimerCommand> for Blinky<UiG> {
    fn handle_event(&mut self, event: BlinkyEvent) -> Option<TimerCommand> {
        self.on_event(event)
    }
}
