use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use crate::command::check_in::CheckinCommand;
use crate::event::{Event, EventQueue};
use crate::plate::Plate;

/// 处理停车 Command

pub struct Parking {
    id: Plate,
    check_in_time: Option<SystemTime>,
    last_pay_time: Option<SystemTime>,
    total_paid: u32,
}

impl Parking {
    pub fn is_parked(&self) -> bool {
        self.check_in_time.is_none()
    }

    /// 入场逻辑具体实现
    /// 1. 检查是否已经入场
    /// 2. 如果已经入场，则发送入场失败事件
    /// 3. 否则发送入场成功事件
    /// 4. 如果事件发送失败也算入场失败
    pub fn check_in(&self, event_queue: Arc<Mutex<dyn EventQueue>>, command: CheckinCommand) -> Result<(), String> {
        let (id, time) = (command.plate, command.time);

        if self.is_parked() {
            event_queue.lock().unwrap().enqueue(
                Event::CheckInFailed { id, time }
            )?;
            return Err(String::from("车辆已入场！"));
        }

        event_queue.lock().unwrap().enqueue(
            Event::CheckedIn { id, time }
        )?;
        Ok(())
    }

    pub fn charge() -> Result<u32, String> {
        Ok(0)
    }
}


/// Parking 仓储
pub trait ParkingRepository {
    fn find_by_id(&self, plate: &Plate) -> Parking;

    fn save(&self, parking: Parking);
}
