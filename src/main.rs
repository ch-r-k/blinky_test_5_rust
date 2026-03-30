#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use panic_probe as _;

mod application_layer;
mod application_manager;
mod device_layer;
mod device_layer_abstraction;
mod device_manager;
mod hardware_layer;
mod hardware_layer_abstraction;
mod hardware_manager;

use crate::application_manager::ApplicationManager;
use crate::device_manager::DeviceManager;
use crate::hardware_manager::HardwareManager;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Initialize hardware layer
    let hardware = HardwareManager::init(p);

    // Initialize device layer
    let device = DeviceManager::init(hardware);

    // Initialize and run application layer
    ApplicationManager::init_and_run(device, spawner).await;
}
