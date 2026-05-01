use crate::app::Mono;
use crate::framework::event::{BlinkyEvent, ButtonEvent, TimerCommand, TimerRecipient};
use crate::framework::timer::TimerTable;
use rtic_monotonics::fugit::ExtU64;
use rtic_monotonics::Monotonic;
use rtic_sync::channel::Receiver;
use rtic_sync::channel::Sender;

type MonoInstant = <Mono as Monotonic>::Instant;

pub fn apply_command<const N: usize, Instant>(
    timers: &mut TimerTable<N, Instant>,
    timer_command: TimerCommand,
    mut deadline_for_delay: impl FnMut(u32) -> Instant,
) where
    Instant: Copy + Ord,
{
    timers.apply_command(timer_command, |delay_ms| deadline_for_delay(delay_ms));
}

pub async fn publish_due_timers<const N: usize, Instant>(
    timers: &mut TimerTable<N, Instant>,
    now: Instant,
    blinky_event_sender: &mut Sender<'static, BlinkyEvent, 8>,
    button_event_sender: &mut Sender<'static, ButtonEvent, 8>,
) where
    Instant: Copy + Ord,
{
    let due_timers = timers.collect_due(now);

    for timer in due_timers {
        match timer.recipient {
            TimerRecipient::Blinky => {
                let _ = blinky_event_sender
                    .send(BlinkyEvent::Timeout {
                        timer_id: timer.timer_id,
                        generation: timer.generation,
                    })
                    .await;
            }
            TimerRecipient::Button => {
                let _ = button_event_sender
                    .send(ButtonEvent::Timeout {
                        timer_id: timer.timer_id,
                        generation: timer.generation,
                    })
                    .await;
            }
        }
    }
}

pub async fn run_loop<const N: usize>(
    timers: &mut TimerTable<N, MonoInstant>,
    timer_command_receiver: &mut Receiver<'static, TimerCommand, 8>,
    blinky_event_sender: &mut Sender<'static, BlinkyEvent, 8>,
    button_event_sender: &mut Sender<'static, ButtonEvent, 8>,
) -> ! {
    loop {
        if let Some(next_deadline) = timers.next_deadline() {
            match Mono::timeout_at(next_deadline, timer_command_receiver.recv()).await {
                Ok(Ok(timer_command)) => {
                    let now = Mono::now();
                    apply_command(timers, timer_command, |delay_ms| {
                        now + u64::from(delay_ms).millis()
                    });
                }
                Ok(Err(_)) => {}
                Err(_) => {
                    publish_due_timers(
                        timers,
                        Mono::now(),
                        blinky_event_sender,
                        button_event_sender,
                    )
                    .await;
                }
            }
        } else if let Ok(timer_command) = timer_command_receiver.recv().await {
            let now = Mono::now();
            apply_command(timers, timer_command, |delay_ms| now + u64::from(delay_ms).millis());
        }
    }
}