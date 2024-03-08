use std::time::SystemTime;
use crate::plate::Plate;

pub struct ChargeCommand {
    id: Plate,
    time: SystemTime,
}

