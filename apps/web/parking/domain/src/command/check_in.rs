use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use crate::event::EventQueue;
use crate::parking::ParkingRepository;
use crate::plate::Plate;

/// 车辆入场 Command
pub struct CheckinCommand {
    pub plate: Plate,
    pub time: SystemTime,
}

pub struct CheckInCommandHandler {
    parking_repository: Arc<dyn ParkingRepository>,
}

impl CheckInCommandHandler {
    pub fn handle(&self, event_queue: Arc<Mutex<dyn EventQueue>>, command: CheckinCommand) -> Result<(), String> {
        let parking = self.parking_repository.find_by_id(&command.plate);

        let result = parking.check_in(event_queue, command);

        self.parking_repository.save(parking);

        result
    }
}


