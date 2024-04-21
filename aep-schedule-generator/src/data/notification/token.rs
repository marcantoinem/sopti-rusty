use rand::{distributions::Alphanumeric, rngs::StdRng, Rng, SeedableRng};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AuthToken(String);

impl AuthToken {
    pub fn new() -> Self {
        Self(StdRng::from_entropy().sample_iter(&Alphanumeric).take(32).map(char::from).collect())
    }
}