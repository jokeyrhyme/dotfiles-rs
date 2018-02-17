use std::option;

pub fn unoption(s: option::Option<&str>) -> &str {
    return match s {
        Some(s) => s,
        None => "nil",
    };
}
