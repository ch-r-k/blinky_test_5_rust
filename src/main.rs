#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use panic_probe as _;

mod application_layer;
mod device_layer;
mod device_layer_abstraction;
mod hardware_layer;
mod hardware_layer_abstraction;
mod system_manager;

use crate::system_manager::SystemManager;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    SystemManager::run(p);
}
