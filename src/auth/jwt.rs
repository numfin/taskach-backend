use super::Claims;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::usize::MAX;

fn get_jwt_key() -> String {
    std::env::var("SESSION_KEY").unwrap()
}

pub fn token_from<'a>(user: crate::users::User) -> Result<String, String> {
    encode(
        &Header::default(),
        &Claims {
            exp: MAX,
            id: user.id.to_string(),
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            phone: user.phone,
        },
        &EncodingKey::from_secret(get_jwt_key().as_bytes()),
    )
    .map_err(|err| err.to_string())
}

pub fn verify_token<'a>(token_str: &'a str) -> Result<Claims, String> {
    Ok(decode::<Claims>(
        &token_str,
        &DecodingKey::from_secret(get_jwt_key().as_bytes()),
        &Validation::default(),
    )
    .map_err(|err| err.to_string())?
    .claims)
}
