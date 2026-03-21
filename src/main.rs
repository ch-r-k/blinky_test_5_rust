mod application_layer;
mod device_layer;
mod device_layer_abstraction;
mod hardware_layer;
mod hardware_layer_abstraction;
mod system_manager;

use crate::system_manager::SystemManager;

fn main() {
    let system_manager = SystemManager::new();

    system_manager.run();
}
