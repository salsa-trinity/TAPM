pub fn numbers(hash: &str) -> String {
    let mut nums = String::new();
    for c in hash.chars() {
        if c.is_numeric() {
            nums += &c.to_string();
        }
    }
    nums
}
