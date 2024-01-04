#[allow(dead_code)]
pub fn is_palindrome(s: &str) -> bool {
    let str: String = s
        .chars()
        .filter(|&x| x.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();
    str.eq(&str.chars().rev().collect::<String>())
}
