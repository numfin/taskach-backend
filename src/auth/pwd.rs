use argon2::{hash_encoded, verify_encoded, Config};

pub fn create_pwd_hash(pwd: String) -> Result<String, String> {
    let salt: String = std::env::var("PWD_SALT").unwrap();
    let salt_bytes = salt.as_bytes();
    hash_encoded(pwd.as_bytes(), salt_bytes, &Config::default())
        .map_err(|_| "Cannot create password".to_string())
}

pub fn verify_pwd_hash(hash: &String, pwd: &String) -> bool {
    match verify_encoded(hash, pwd.as_bytes()) {
        Ok(result) => result,
        Err(_) => false,
    }
}
