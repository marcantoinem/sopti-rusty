#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WeekDay {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Period {
    day: WeekDay,
    start: u32,
    end: u32,
    room: String,
}

impl Period {
    /// This function suppose that the two period are sorted in order before.
    pub fn is_overlapping(&self, other: &Period) -> bool {
        (self.day == other.day) && (self.end >= other.start)
    }
}
