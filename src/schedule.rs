use crate::schedule_entry::ScheduleEntry;

pub struct Schedule {
    next_appiontment: usize,
    schedule: Vec<ScheduleEntry>,
}

impl Schedule {
    pub fn new() -> Schedule {
        Schedule {
            next_appiontment: 0,
            schedule: Vec::new(),
        }
    }

    pub fn push(&mut self, schedule_entry: ScheduleEntry) {
        self.schedule.push(schedule_entry);
    }

    pub fn update(&mut self, dt: f64, time: f64) -> Option<usize> {
        if time - dt < 0.0 {
            self.next_appiontment = 0;
        }
        if self.next_appiontment < self.schedule.len()
            && time > self.schedule[self.next_appiontment].time
        {
            let building_id = self.schedule[self.next_appiontment].building_id;
            self.next_appiontment += 1;
            return Some(building_id);
        } else {
            return None;
        }
    }
}
