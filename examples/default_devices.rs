extern crate pulsectl;

use pulsectl::controllers::{DeviceControl, SinkController, SourceController};

fn main() {
    let mut sinks = SinkController::create().expect("could not connect to the sound server");
    match sinks.get_default_device() {
        Ok(dev) => println!("SinkController::get_default_device -> {:?}", dev.name),
        Err(err) => println!("SinkController::get_default_device -> error: {:?}", err),
    }
    let mut sources = SourceController::create().expect("could not connect to the sound server");
    match sources.get_default_device() {
        Ok(dev) => println!("SourceController::get_default_device -> {:?}", dev.name),
        Err(err) => println!("SourceController::get_default_device -> error: {:?}", err),
    }
}
