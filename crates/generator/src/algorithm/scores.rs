use serde::{Deserialize, Serialize};

pub const BEST_MORNING: u8 = 4 * 5;
pub const BEST_AFTERNOON: u8 = 64 - BEST_MORNING;

#[derive(PartialEq, PartialOrd, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Score {
    pub global: f64,
    pub day_off: u8,
    pub morning_hours: u16,
    pub min_morning: u8,
    pub afternoon_hours: u16,
    pub min_afternoon: u8,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            global: 0.0,
            day_off: 14,
            morning_hours: 0,
            min_morning: u8::MAX,
            afternoon_hours: 0,
            min_afternoon: u8::MAX,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq)]
pub struct EvaluationOption {
    pub day_off: u8,
    pub morning: i8,
    pub finish_early: u8,
}

impl Score {
    pub fn evaluate(&mut self, options: EvaluationOption) -> f64 {
        let day_off = 2.0 * options.day_off as f64;
        let morning = options.morning as f64;
        let finish_early = options.finish_early as f64;
        let sum = day_off + morning.abs() + finish_early;
        self.global = 0.0;
        self.global += day_off * self.day_off as f64 / 14.0 / sum;
        if morning.is_sign_positive() {
            self.global += 0.5 * morning * (self.min_morning as f64 + self.morning_hours as f64)
                / (BEST_MORNING as f64 * 14.0)
                / sum;
        } else {
            self.global += -0.5
                * morning
                * (2.0
                    - (self.min_morning as f64 + self.morning_hours as f64)
                        / (BEST_MORNING as f64 * 14.0))
                / sum;
        }
        self.global +=
            0.5 * finish_early * (self.min_afternoon as f64 + self.afternoon_hours as f64)
                / (BEST_AFTERNOON as f64 * 14.0)
                / sum;
        self.global
    }
}
