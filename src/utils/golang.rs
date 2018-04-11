use std::env::consts::{ARCH, OS};

pub fn arch() -> &'static str {
    if ARCH == "x86_64" { "amd64" } else { ARCH }
}

pub fn os() -> &'static str {
    if OS == "macos" { "darwin" } else { OS }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arch_is_not_x86_64() {
        assert_ne!(arch(), "x86_64");
    }

    #[test]
    fn os_is_not_macos() {
        assert_ne!(os(), "macos");
    }
}