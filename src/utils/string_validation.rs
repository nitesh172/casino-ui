pub fn validate_contains_number(password: &str) -> bool {
    password.chars().any(|c| c.is_ascii_digit())
}

pub fn validate_contains_uppercase(password: &str) -> bool {
    password.chars().any(|c| c.is_ascii_uppercase())
}

pub fn validate_contains_special(password: &str) -> bool {
    password.chars().any(|c| !c.is_alphanumeric())
}

pub fn validate_length(password: &str, min_length: usize) -> bool {
    password.len() >= min_length
}
