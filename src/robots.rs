use url::Url;

#[derive(Default)]
pub struct Robots {
    disallowed: Vec<String>,
}

impl Robots {
    pub fn parse(body: &str) -> Self {
        let disallowed = body
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                let rest = line
                    .strip_prefix("Disallow:")
                    .or_else(|| line.strip_prefix("disallow:"))?;
                Some(rest.trim().to_string())
            })
            .filter(|path| !path.is_empty())
            .collect();

        Robots { disallowed }
    }

    pub fn is_allowed(&self, path: &str) -> bool {
        !self
            .disallowed
            .iter()
            .any(|prefix| path.starts_with(prefix.as_str()))
    }
}

pub fn fetch(client: &reqwest::blocking::Client, start_url: &Url) -> Robots {
    let Ok(robots_url) = start_url.join("/robots.txt") else {
        return Robots::default();
    };

    match client.get(robots_url).send() {
        Ok(response) if response.status().is_success() => {
            Robots::parse(&response.text().unwrap_or_default())
        }
        _ => Robots::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowes_everything_with_no_disallow_lines() {
        let robots = Robots::parse("User-agent: *\nAllow: /\n");
        assert!(robots.is_allowed("/anything"));
    }

    #[test]
    fn disallowed_path_blocks_only_that_prefix() {
        let robots = Robots::parse("User-agent: *\nDisallow: /private\n");
        assert!(!robots.is_allowed("/private/page"));
        assert!(robots.is_allowed("/public"));
    }

    #[test]
    fn missing_robots_txt_allows_everything() {
        let robots = Robots::default();
        assert!(robots.is_allowed("/anything"));
    }
}
