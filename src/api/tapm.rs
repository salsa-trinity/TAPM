pub fn hash(seed: &str, salt: &str) -> String {
    // argon2 bcrybt scrypt pbkdf2 sha256
    let config = argon2::Config::default();
    let hash = argon2::hash_encoded(seed.as_bytes(), salt.as_bytes(), &config).unwrap();
    let hash = &hash[31..];
    println!("hash: {}", hash);
    println!("numb: {}", crate::api::filters::numbers(&hash));
    hash.to_string()
}
