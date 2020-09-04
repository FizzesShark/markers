use argon2::{self, Config};
use std::env;

pub fn hash(str: &str) -> String {
    let pwd = str.as_bytes();
    let salt_owner = env::var("SALT").unwrap();
    let salt = salt_owner.as_bytes();
    let config = Config::default();

    argon2::hash_encoded(pwd, salt, &config).unwrap()
}
