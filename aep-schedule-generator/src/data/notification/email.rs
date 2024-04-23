use super::auth_token::AuthToken;

pub struct Email {
    email: String,
    identification_hash: AuthToken,
}