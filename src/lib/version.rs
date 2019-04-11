use regex;

const UNSTABLE: &[&str] = &["alpha", "beta", "canary", "dev", "preview", "rc"];

pub fn is_stable<S>(version: S) -> bool
where
    S: AsRef<str>,
{
    let re = regex::Regex::new(&format!(
        "(\\b|[[:^alpha:]])({})(\\b|[[:^alpha:]])",
        UNSTABLE.join("|")
    ))
    .unwrap();
    !re.is_match(&version.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_stable_versions() {
        assert!(is_stable("1.0.0"));
        assert!(is_stable("1.0.0-arch.x86")); // check "\brc\b" no match

        assert!(!is_stable("1.0.0-alpha.1"));
        assert!(!is_stable("1.0.0-beta.1"));
        assert!(!is_stable("go1.12beta1"));
    }
}
