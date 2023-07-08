#[derive(Debug, Clone)]
pub enum WeekDay {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

#[derive(Debug, Clone)]
pub struct Period {
    day: WeekDay,
    time_end: u32,
    time_start: u32,
    room: String,
}
