use regex::Regex;

#[inline]
pub fn is_username_legit(username: &str) -> bool {
    /*
        1. No blank characters
        2. No line breaks, tabs or other special characters
        3. Between 4 and 512 characters
     */
    let regex = Regex::new(r"^[^\s\t\r\n]{4,512}$").unwrap();
    regex.is_match(username)
}

#[inline]
pub fn is_email_legit(email: &str) -> bool {
    let regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    regex.is_match(email)
}