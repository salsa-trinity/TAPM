pub fn hash(seed: &String) -> String {
    // argon2 bcrybt scrypt pbkdf2 sha256
    let mut password = String::new();
    let mut salt = String::new();
    std::io::stdin().read_line(&mut password).unwrap();
    std::io::stdin().read_line(&mut salt).unwrap(); // TODO: add salt when using a username
    let config = argon2::Config::default();
    let hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap();
    println!("hash: {}", hash);
    hash
}
