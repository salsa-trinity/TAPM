pub fn hash(seed: &str, salt: &str) -> String {
    // make a unique number per user for how many times it
    // recursively sha256, and therefore get the benefits of
    // bcrypt's timely function, while also providing extra security
    // for people how dont know that users sha number

    let hash = sha256::digest(seed.to_owned() + salt).to_string();
    println!("hash: {}", hash);
    hash
}
