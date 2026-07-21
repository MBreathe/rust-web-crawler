use url::Url;

/// Resolves `href` against `base` into an absolute URL with any fragment stripped.
/// Returns `None` if `href` doesn't parse as a URL at all.
pub fn resolve_link(base: &Url, href: &str) -> Option<Url> {
    let mut resolved = base.join(href).ok()?;
    resolved.set_fragment(None);
    Some(resolved)
}

/// True if `a` and `b` have the same host.
pub fn same_domain(a: &Url, b: &Url) -> bool {
    a.host_str() == b.host_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_path_resolution() {
        assert_eq!(
            resolve_link(&Url::parse("https://example.com/a/b").unwrap(), "c"),
            Some(Url::parse("https://example.com/a/c").unwrap())
        );
    }

    #[test]
    fn fragment_stripped() {
        assert_eq!(
            resolve_link(&Url::parse("https://example.com/a#fragment").unwrap(), "c"),
            Some(Url::parse("https://example.com/c").unwrap())
        );
    }

    #[test]
    fn same_domain_returns_correctly() {
        assert!(same_domain(
            &Url::parse("https://example.com/a/b").unwrap(),
            &Url::parse("https://example.com/a/b").unwrap()
        ));
        assert!(!same_domain(
            &Url::parse("https://example.com/a/b").unwrap(),
            &Url::parse("https://google.com/a/b").unwrap()
        ))
    }
}
