use super::time::period::Period;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Group {
    pub number: u8,
    pub closed: bool,
    pub periods: Vec<Period>,
}

impl Group {
    pub fn new(number: &str, period: Period) -> Self {
        let number = number.parse().unwrap_or(0);
        Self {
            number,
            closed: false,
            periods: vec![period],
        }
    }
}
