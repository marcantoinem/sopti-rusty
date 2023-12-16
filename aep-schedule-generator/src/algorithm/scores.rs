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
        let day_off = 2.0 * options.day_off as f64;
        let morning = options.morning as f64;
        let finish_early = options.finish_early as f64;
        let sum = day_off + morning.abs() + finish_early;
        score.global += day_off * schedule.day_off() / sum;
        if morning.is_sign_positive() {
            score.global += morning * schedule.more_morning() / sum;
        } else {
            score.global += -morning * (1.0 - schedule.more_morning()) / sum;
        }
        score.global += finish_early * schedule.finish_early() / sum;
        score
    }
}
