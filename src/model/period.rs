#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Period {
    day: String,
    hour: usize,
    room: String,
}

impl Period {
    pub fn new(day: impl Into<String>, hour: usize, room: impl Into<String>) -> Self {
        Self {
            day: day.into(),
            hour,
            room: room.into(),
        }
    }
    /// This function suppose that the two periods are sorted in order before.
    pub fn is_overlapping(&self, other: &Period) -> bool {
        (self.day == other.day) && (self.hour == other.hour)
    }
}
