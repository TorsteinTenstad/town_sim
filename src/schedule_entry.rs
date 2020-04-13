pub struct ScheduleEntry {
    pub time: f64,
    pub building_id: usize,
}

impl ScheduleEntry {
    pub fn new(time: f64, building_id: usize) -> ScheduleEntry {
        ScheduleEntry { time, building_id }
    }
}
