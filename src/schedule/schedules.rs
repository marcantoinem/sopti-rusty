use super::schedule::Schedule;
use std::{cmp::Ordering, collections::BinaryHeap};

/// It reverse the order of the value to make the heap a min heap
#[derive(Debug, PartialEq)]
struct ScheduleValue {
    value: f64,
    schedule: Schedule,
}

impl ScheduleValue {
    fn new(value: f64, schedule: Schedule) -> ScheduleValue {
        Self { value, schedule }
    }
}

impl PartialOrd for ScheduleValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.value.partial_cmp(&self.value)
    }
}

impl Eq for ScheduleValue {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Ord for ScheduleValue {
    fn cmp(&self, other: &Self) -> Ordering {
        other.value.partial_cmp(&self.value).unwrap()
    }
}

#[derive(Debug)]
pub struct Schedules {
    schedules: BinaryHeap<ScheduleValue>,
    max_size: usize,
}

impl Schedules {
    pub fn new(max_size: usize) -> Self {
        Self {
            schedules: BinaryHeap::new(),
            max_size,
        }
    }

    pub fn push(&mut self, schedule: Schedule, evaluation: impl Fn(&Schedule) -> f64) {
        let evaluation = evaluation(&schedule);
        if let Some(schedule) = self.schedules.peek() {
            if evaluation < schedule.value {
                return;
            }
        }
        if self.schedules.len() >= self.max_size {
            self.schedules.pop();
        }
        let schedule_value = ScheduleValue::new(evaluation, schedule);
        self.schedules.push(schedule_value);
    }
}
