use super::schedule::Schedule;
use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug)]
pub struct Schedules {
    schedules: BinaryHeap<Reverse<Schedule>>,
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
        if let Some(Reverse(schedule)) = self.schedules.peek() {
            if evaluation < schedule.value {
                return;
            }
        }
        if self.schedules.len() >= self.max_size {
            self.schedules.pop();
        }
        schedule.value = evaluation;
        self.schedules.push(Reverse(schedule));
    }

    pub fn into_sorted_vec(self) -> Vec<Schedule> {
        self.schedules
            .into_sorted_vec()
            .into_iter()
            .map(|r| r.0)
            .collect()
    }
}
