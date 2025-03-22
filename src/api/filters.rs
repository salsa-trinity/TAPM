pub fn numbers(hash: &str) -> String {
    hash.chars().filter(|c| c.is_numeric()).collect()
}
