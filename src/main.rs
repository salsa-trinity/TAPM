fn main() {
    // TODO: change to use bcrypt by default
    println!("TAPM");
    loop {
        let mut seed = String::new();
        let mut salt = String::new();
        std::io::stdin().read_line(&mut seed).unwrap();
        std::io::stdin().read_line(&mut salt).unwrap();
        tapm::api::tapm::hash(&seed, &salt);
    }
}
