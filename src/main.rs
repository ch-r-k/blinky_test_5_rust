#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

mod application_layer;
mod application_manager;
mod device_layer;
mod device_layer_abstraction;
mod device_manager;
mod framework;
mod hardware_layer;
mod hardware_layer_abstraction;
mod hardware_manager;

use crate::application_manager::{
    AppBlinky, AppButton, AppButtonIrqCallback, ApplicationManager,
};
use crate::device_manager::{DeviceGpioInput, DeviceManager};
use crate::device_layer::user_input::UserInputPressCallbackAdapter;
use crate::hardware_manager::HardwareManager;

// Boot2 section required by rp2040-hal / cortex-m-rt for RP2040.
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[rtic::app(device = rp2040_hal::pac, dispatchers = [TIMER_IRQ_1])]
mod app {
    use super::*;
    use crate::application_layer::button::ButtonEffect;
    use crate::framework::active_object::ActiveObject;
    use crate::framework::event::{BlinkyEvent, ButtonEvent, TimerCommand};
    use crate::framework::message_bus::{BusReceiver, BusSender};
    use crate::framework::timer::TimerTable;
    use crate::framework::timer_service as timer_runtime;
    use crate::hardware_layer_abstraction::i_gpio_input::IGpioInput;
    use rtic_monotonics::rp2040::prelude::*;
    use rtic_sync::channel::Channel;

    rp2040_timer_monotonic!(Mono);

    const MAX_TIMERS: usize = 8;

    type MonoInstant = <Mono as rtic_monotonics::Monotonic>::Instant;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        blinky: AppBlinky,
        button_ao: AppButton,
        gpio_input: DeviceGpioInput,
        user_input_callback: UserInputPressCallbackAdapter<AppButtonIrqCallback>,
        blinky_event_receiver: BusReceiver<'static, BlinkyEvent, 8>,
        blinky_event_sender_for_button: BusSender<'static, BlinkyEvent, 8>,
        blinky_event_sender_for_timer: BusSender<'static, BlinkyEvent, 8>,
        button_event_receiver: BusReceiver<'static, ButtonEvent, 8>,
        button_event_sender_for_timer: BusSender<'static, ButtonEvent, 8>,
        timer_command_receiver: BusReceiver<'static, TimerCommand, 8>,
        timer_command_sender_for_blinky: BusSender<'static, TimerCommand, 8>,
        timer_command_sender_for_button: BusSender<'static, TimerCommand, 8>,
        timers: TimerTable<MAX_TIMERS, MonoInstant>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        let pac = ctx.device;

        // The monotonic timer must be started first; it takes ownership of TIMER.
        Mono::start(pac.TIMER, &pac.RESETS);

        // Initialise the remaining hardware (clocks, GPIO).
        let hardware = HardwareManager::init(
            pac.WATCHDOG,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            pac.RESETS,
            pac.SIO,
            pac.IO_BANK0,
            pac.PADS_BANK0,
        );

        let device = DeviceManager::init(hardware);
        let crate::device_manager::DeviceResources {
            user_indication,
            user_input,
        } = device;
        let blinky_event_channel =
            cortex_m::singleton!(: Channel<BlinkyEvent, 8> = Channel::new()).unwrap();
        let (blinky_event_sender, blinky_event_receiver) = blinky_event_channel.split();
        let button_event_channel =
            cortex_m::singleton!(: Channel<ButtonEvent, 8> = Channel::new()).unwrap();
        let (button_event_sender, button_event_receiver) = button_event_channel.split();
        let timer_command_channel =
            cortex_m::singleton!(: Channel<TimerCommand, 8> = Channel::new()).unwrap();
        let (mut timer_command_sender, timer_command_receiver) = timer_command_channel.split();
        let blinky_event_sender_for_button = blinky_event_sender.clone();
        let blinky_event_sender_for_timer = blinky_event_sender.clone();
        let button_event_sender_for_timer = button_event_sender.clone();
        let timer_command_sender_for_blinky = timer_command_sender.clone();
        let timer_command_sender_for_button = timer_command_sender.clone();
        let mut application = ApplicationManager::init(user_indication, button_event_sender.clone());
        let gpio_input = user_input.into_inner();
        let user_input_callback = UserInputPressCallbackAdapter::new(application.button_irq_callback);

        let _ = timer_command_sender.try_send(application.blinky.arm_initial_timeout());

        // Spawn the active object and shared timer service.
        blink::spawn().ok();
        button::spawn().ok();
        timer_service::spawn().ok();

        (
            Shared {},
            Local {
                blinky: application.blinky,
                button_ao: application.button_ao,
                gpio_input,
                user_input_callback,
                blinky_event_receiver,
                blinky_event_sender_for_button,
                blinky_event_sender_for_timer,
                button_event_receiver,
                button_event_sender_for_timer,
                timer_command_receiver,
                timer_command_sender_for_blinky,
                timer_command_sender_for_button,
                timers: TimerTable::new(),
            },
        )
    }

    /// Active-object task for the Blinky state machine.
    #[task(local = [blinky, blinky_event_receiver, timer_command_sender_for_blinky], priority = 1)]
    async fn blink(ctx: blink::Context) -> ! {
        loop {
            if let Ok(event) = ctx.local.blinky_event_receiver.recv().await {
                if let Some(timer_command) = ActiveObject::handle_event(ctx.local.blinky, event) {
                    let _ = ctx.local.timer_command_sender_for_blinky.send(timer_command).await;
                }
            }
        }
    }

    /// Active-object task for button debouncing and semantic event generation.
    #[task(local = [button_ao, button_event_receiver, blinky_event_sender_for_button, timer_command_sender_for_button], priority = 1)]
    async fn button(ctx: button::Context) -> ! {
        loop {
            if let Ok(event) = ctx.local.button_event_receiver.recv().await {
                if let Some(effect) = ActiveObject::handle_event(ctx.local.button_ao, event) {
                    match effect {
                        ButtonEffect::TimerCommand(timer_command) => {
                            let _ = ctx.local.timer_command_sender_for_button.send(timer_command).await;
                        }
                        ButtonEffect::BlinkyEvent(blinky_event) => {
                            let _ = ctx.local.blinky_event_sender_for_button.send(blinky_event).await;
                        }
                    }
                }
            }
        }
    }

    /// Shared timer service for the whole application.
    #[task(local = [timer_command_receiver, blinky_event_sender_for_timer, button_event_sender_for_timer, timers], priority = 1)]
    async fn timer_service(ctx: timer_service::Context) -> ! {
        timer_runtime::run_loop(
            ctx.local.timers,
            ctx.local.timer_command_receiver,
            ctx.local.blinky_event_sender_for_timer,
            ctx.local.button_event_sender_for_timer,
        )
        .await
    }

    /// GPIO interrupt handler – calls hardware gpio input directly and forwards
    /// callback into the application button handler.
    #[task(binds = IO_IRQ_BANK0, local = [gpio_input, user_input_callback])]
    fn button_irq(ctx: button_irq::Context) {
        ctx.local.gpio_input.handle_irq(ctx.local.user_input_callback);
    }
}
