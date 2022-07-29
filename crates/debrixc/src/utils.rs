pub fn str_val(string: &str) -> &str {
    let mut chars = string.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}
