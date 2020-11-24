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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::users::User;
    use chrono::prelude::*;
    #[test]
    fn it_creates_valid_token() {
        std::env::set_var("SESSION_KEY", "test");
        let user = User {
            id: juniper::ID::from("a".to_string()),
            email: "b".to_string(),
            first_name: "c".to_string(),
            last_name: "d".to_string(),
            phone: "e".to_string(),
            password_hash: "".to_string(),
            created_at: DateTime::<Utc>::from(chrono::Local::now()),
            updated_at: DateTime::<Utc>::from(chrono::Local::now()),
            active: true,
        };
        let token = token_from(user.clone()).unwrap();
        let verified_token = verify_token(&token).unwrap();
        assert_eq!(
            verified_token,
            Claims {
                id: user.id.to_string(),
                email: user.email,
                first_name: user.first_name,
                last_name: user.last_name,
                exp: MAX,
                phone: user.phone
            }
        )
    }
}
