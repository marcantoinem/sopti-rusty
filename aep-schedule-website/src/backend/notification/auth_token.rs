use rand::distr::Alphanumeric;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AuthToken(String);

impl AuthToken {
    pub fn new() -> Self {
        Self(
            StdRng::from_os_rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .map(char::from)
                .collect(),
        )
    }
}
