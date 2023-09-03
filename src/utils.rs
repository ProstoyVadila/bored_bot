pub fn capitalze(s: String) -> String {
    let s = s.to_lowercase();
    let mut chars = s.as_str().chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
