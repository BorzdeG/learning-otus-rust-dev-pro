#![allow(unused)]
use std::error::Error;

type Watt = f32;

enum SmartSocketStatus {
    On,
    Off,
}

struct SmartSocketInfo {
    status: SmartSocketStatus,
    power_consumption: Watt,
}

trait SmartSocket {
    fn description() -> String;
    fn on() -> Result<SmartSocketInfo, Box<dyn Error>>;
    fn off();
    fn current() -> SmartSocketInfo;
}

type Degree = f32;
trait Thermometer {
    fn current() -> Degree;
}

fn main() {}
