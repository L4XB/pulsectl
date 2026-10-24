extern crate pulsectl;

use pulsectl::controllers::{DeviceControl, SourceController};

fn main() {
    let mut sources = SourceController::create().expect("could not connect to the sound server");
    match sources.get_default_device() {
        Ok(dev) => println!("SourceController::get_default_device -> {:?}", dev.name),
        Err(err) => println!("SourceController::get_default_device -> error: {:?}", err),
    }
}
