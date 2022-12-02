use regex::Regex;

const BCRYPT_COST: u32 = 6;

pub fn is_email(str_to_check: &str) -> bool {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();

    email_regex.is_match(&str_to_check)
}

pub fn hash_password(password: &str) -> String {
    bcrypt::hash(password, BCRYPT_COST).unwrap()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap_or_else(|_| false)
}
