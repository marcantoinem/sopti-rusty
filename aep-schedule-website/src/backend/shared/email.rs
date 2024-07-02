#[derive(PartialEq, Eq, Hash)]
pub struct Email {
    pub email: String,
}

impl Email {
    pub fn new(email: String) -> Self {
        Self { email }
    }
}
