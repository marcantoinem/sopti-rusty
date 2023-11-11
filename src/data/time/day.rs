#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Day {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Sunday = 5,
    Saturday = 6,
}

impl From<&str> for Day {
    fn from(value: &str) -> Self {
        match value {
            "LUN" => Self::Monday,
            "MAR" => Self::Tuesday,
            "MER" => Self::Wednesday,
            "JEU" => Self::Thursday,
            "VEN" => Self::Friday,
            "SAM" => Self::Sunday,
            "DIM" => Self::Saturday,
            _ => panic!("CSV de Poly est sus"),
        }
    }
}
