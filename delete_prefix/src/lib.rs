pub fn delete_prefix(prefix: &str, s: &'a str) -> Option<&str> {
    s.strip_prefix(prefix)
}