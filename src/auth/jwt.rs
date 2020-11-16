use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

pub fn token_from<'a>(fields: Vec<(&'a str, String)>) -> Result<String, String> {
    let mut claims: BTreeMap<&str, &str> = BTreeMap::new();
    fields.iter().for_each(|(key, value)| {
        claims.insert(key, value);
    });
    let key = std::env::var("SESSION_KEY").unwrap();
    let key = key.as_bytes();
    let key = <Hmac<Sha256>>::new_varkey(key).map_err(|_| "Unable to create token".to_string())?;

    claims
        .clone()
        .sign_with_key(&key)
        .map_err(|_| "Unable to sign token".to_string())
}
