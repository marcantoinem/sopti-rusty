use super::period::Period;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Group {
    pub name: String,
    pub closed: bool,
    pub periods: Vec<Period>,
}

impl Group {
    pub fn new(name: impl Into<String>, period: Period) -> Self {
        Self {
            name: name.into(),
            closed: false,
            periods: vec![period],
        }
    }
}
