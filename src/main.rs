#![no_std]
#![no_main]

mod application_layer;
mod device_layer;
mod device_layer_abstraction;
mod hardware_layer;
mod hardware_layer_abstraction;
mod system_manager;

use crate::system_manager::SystemManager;
use panic_halt as _;
use rp_pico::entry;

#[entry]
fn main() -> ! {
    let mut system_manager = SystemManager::new();
    system_manager.run();

    loop {
        system_manager.run();
        cortex_m::asm::nop();
    }
}
