use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email {
    pub email: String,
}

impl Email {
    pub fn new(email: String) -> Self {
        Self { email }
    }
}
