use super::{
    generation::SchedulesOptions,
    schedule::{Schedule, ScheduleBuilder},
    taken_course::TakenCourseBuilder,
};
use crate::data::group_index::GroupIndex;
use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug)]
pub struct Schedules<'a> {
    schedules: BinaryHeap<Reverse<ScheduleBuilder<'a>>>,
    options: &'a SchedulesOptions,
    pub number: u64,
}

impl<'a> Schedules<'a> {
    pub fn new(options: &'a SchedulesOptions) -> Self {
        Self {
            schedules: BinaryHeap::new(),
            options,
            number: 0,
        }
    }

    pub fn get_min(&self) -> f64 {
        if self.schedules.len() < self.options.max_size {
            return 0.0;
        }
        self.schedules.peek().unwrap().0.score.global
    }

    pub fn into_sorted_vec(self) -> Vec<Schedule> {
        self.schedules
            .into_sorted_vec()
            .into_iter()
            .map(|r| r.0.build())
            .rev()
            .collect()
    }

    pub(super) fn get_schedules_rec(&mut self, schedule: ScheduleBuilder<'a>, n: u8, i: u8) {
        let Some(course) = schedule.courses.get(i as usize) else {
            self.number += 1;
            self.push(schedule);
            return;
        };
        let min = self.get_min();
        let e = self.options.evaluation;
        let c = self.options.user_conflicts;
        match (course.theo_groups.is_empty(), course.lab_groups.is_empty()) {
            (false, false) => {
                for theo_group in course.theo_groups.iter().filter(|g| g.open) {
                    for lab_group in course.lab_groups.iter().filter(|g| g.open) {
                        let course =
                            TakenCourseBuilder::new(i, theo_group.into(), lab_group.into());
                        if let Some(schedule) = schedule.add_check_conflicts(n, min, &c, e, course)
                        {
                            self.get_schedules_rec(schedule, n, i + 1);
                        }
                    }
                }
            }
            (false, true) => {
                for theo_group in course.theo_groups.iter().filter(|g| g.open) {
                    let course = TakenCourseBuilder::new(i, theo_group.into(), GroupIndex::none());
                    if let Some(schedule) = schedule.add_check_conflicts(n, min, &c, e, course) {
                        self.get_schedules_rec(schedule, n, i + 1);
                    }
                }
            }
            (true, false) => {
                for lab_group in course.lab_groups.iter().filter(|g| g.open) {
                    let course = TakenCourseBuilder::new(i, GroupIndex::none(), lab_group.into());
                    if let Some(schedule) = schedule.add_check_conflicts(n, min, &c, e, course) {
                        self.get_schedules_rec(schedule, n, i + 1);
                    }
                }
            }
            _ => (),
        }
    }

    fn push(&mut self, schedule: ScheduleBuilder<'a>) {
        let evaluation = schedule.score;
        if let Some(Reverse(schedule)) = self.schedules.peek() {
            if evaluation < schedule.score && self.schedules.len() == self.options.max_size {
                return;
            }
        }
        if self.schedules.len() >= self.options.max_size {
            self.schedules.pop();
        }
        self.schedules.push(Reverse(schedule));
    }
}
