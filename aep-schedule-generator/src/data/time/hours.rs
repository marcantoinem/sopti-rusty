// Hour start at 8 and stop and include 22
// Each 1 represent an occupied 15min
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
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
