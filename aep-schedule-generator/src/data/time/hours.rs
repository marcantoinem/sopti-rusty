// Hour start at 8 and stop and include 22
// Each 1 represent an occupied 15min
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Deref};

pub const NO_HOUR: Hours = Hours(0);

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Hours(pub u64);

impl Debug for Hours {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016X}", self.0)
    }
}

impl Deref for Hours {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Hours {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} à {}", self.starting_text(), self.end_text())
    }
}

impl Hours {
    pub fn start(self) -> u8 {
        self.0.trailing_zeros() as u8
    }

    pub fn end(self) -> u8 {
        (self.0.ilog2() + 1) as u8
    }

    pub fn start_minutes(self) -> u8 {
        (self.start() & 0b11) * 15
    }

    pub fn last_minutes(self) -> u8 {
        (self.end() & 0b11) * 15
    }

    pub fn starting_hour(self) -> u8 {
        (self.start() >> 2) + 8
    }

    pub fn last_hour(self) -> u8 {
        (self.end() >> 2) + 8
    }

    pub fn starting_text(self) -> String {
        let mut hour = self.starting_hour().to_string();
        let minute = self.start_minutes();
        hour.push_str(&format!("h{:0>2}", minute));
        hour
    }

    pub fn end_text(self) -> String {
        let mut hour = self.last_hour().to_string();
        let minute = self.last_minutes();
        hour.push_str(&format!("h{:0>2}", minute));
        hour
    }

    /// Only use on single block of time!
    pub fn len_hour(self) -> u8 {
        self.last_hour() - self.starting_hour()
    }
}

impl From<&str> for Hours {
    fn from(value: &str) -> Self {
        let value = value.parse::<u64>().unwrap();
        let hour = value / 100;
        let min = value % 100;
        let hour = hour - 8;
        let min = min / 15;
        let bit = hour * 4 + min;
        let mut bits = 0;
        for i in 0..4 {
            bits |= 1 << (bit + i);
        }
        Hours(bits)
    }
}

impl From<u64> for Hours {
    fn from(value: u64) -> Self {
        Hours(value)
    }
}

impl BitAnd for Hours {
    type Output = Hours;
    fn bitand(self, rhs: Self) -> Self::Output {
        Hours(self.0 & rhs.0)
    }
}

impl BitOr for Hours {
    type Output = Hours;
    fn bitor(self, rhs: Self) -> Self::Output {
        Hours(self.0 | rhs.0)
    }
}

impl BitOrAssign for Hours {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAndAssign for Hours {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}
