use super::schedule::Schedule;
use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, PartialOrd, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Score {
    pub global: f64,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq)]
pub struct EvaluationOption {
    pub day_off: u8,
    pub morning: i8,
    pub finish_early: u8,
}

impl Score {
    pub fn evaluate(schedule: &Schedule, options: EvaluationOption) -> Self {
        let mut score = Score::default();
        let sum =
            (2 * options.day_off) as f64 + options.morning as f64 + options.finish_early as f64;
        score.global += (2 * options.day_off) as f64 * schedule.day_off() / sum;
        score.global += options.morning as f64 * schedule.more_morning() / sum;
        score.global += options.finish_early as f64 * schedule.finish_early() / sum;
        score
    }
}
