use std::time::SystemTime;
use crate::plate::Plate;

pub enum Event {
    CheckInFailed {
        id: Plate,
        time: SystemTime,
    },

    CheckedIn {
        id: Plate,
        time: SystemTime,
    },
}

pub trait EventQueue {
    fn enqueue(&mut self, event: Event) -> Result<(), String>;
}