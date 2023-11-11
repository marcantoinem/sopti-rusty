use super::schedule::Schedule;
use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct Schedules {
    schedules: BinaryHeap<Schedule>,
    max_size: usize,
}

impl Schedules {
    pub fn new(max_size: usize) -> Self {
        Self {
            schedules: BinaryHeap::new(),
            max_size,
        }
    }

    pub fn push(&mut self, mut schedule: Schedule, evaluation: impl Fn(&Schedule) -> f64) {
        let evaluation = evaluation(&schedule);
        if let Some(schedule) = self.schedules.peek() {
            if evaluation < schedule.value {
                return;
            }
        }
        if self.schedules.len() >= self.max_size {
            self.schedules.pop();
        }
        schedule.value = evaluation;
        self.schedules.push(schedule);
    }
}
