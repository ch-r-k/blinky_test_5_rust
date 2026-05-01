use crate::framework::event::{TimerCommand, TimerRecipient};
use heapless::Vec;

#[derive(Clone, Copy)]
pub struct ArmedTimer<Instant> {
    pub recipient: TimerRecipient,
    pub timer_id: u8,
    pub generation: u32,
    pub deadline: Instant,
}

pub struct TimerTable<const N: usize, Instant> {
    slots: [Option<ArmedTimer<Instant>>; N],
}

impl<const N: usize, Instant> TimerTable<N, Instant>
where
    Instant: Copy + Ord,
{
    pub fn new() -> Self {
        Self { slots: [None; N] }
    }

    pub fn apply_command(
        &mut self,
        timer_command: TimerCommand,
        mut deadline_for_delay: impl FnMut(u32) -> Instant,
    ) {
        match timer_command {
            TimerCommand::Arm {
                recipient,
                timer_id,
                generation,
                delay_ms,
            } => {
                self.clear_timer(recipient, timer_id);

                if let Some(slot) = self.find_free_timer_slot() {
                    *slot = Some(ArmedTimer {
                        recipient,
                        timer_id,
                        generation,
                        deadline: deadline_for_delay(delay_ms),
                    });
                }
            }
            TimerCommand::Disarm { recipient, timer_id } => self.clear_timer(recipient, timer_id),
        }
    }

    pub fn next_deadline(&self) -> Option<Instant> {
        let mut next = None;

        for timer in self.slots.iter().flatten() {
            next = match next {
                Some(current) if current <= timer.deadline => Some(current),
                _ => Some(timer.deadline),
            };
        }

        next
    }

    pub fn collect_due(&mut self, now: Instant) -> Vec<ArmedTimer<Instant>, N> {
        let mut due = Vec::new();

        for slot in self.slots.iter_mut() {
            let due_timer = match slot {
                Some(timer) if timer.deadline <= now => Some(*timer),
                _ => None,
            };

            if let Some(timer) = due_timer {
                *slot = None;
                let _ = due.push(timer);
            }
        }

        due
    }

    fn clear_timer(&mut self, recipient: TimerRecipient, timer_id: u8) {
        for slot in self.slots.iter_mut() {
            if matches!(slot, Some(timer) if timer.recipient == recipient && timer.timer_id == timer_id) {
                *slot = None;
            }
        }
    }

    fn find_free_timer_slot(&mut self) -> Option<&mut Option<ArmedTimer<Instant>>> {
        for slot in self.slots.iter_mut() {
            if slot.is_none() {
                return Some(slot);
            }
        }

        None
    }
}