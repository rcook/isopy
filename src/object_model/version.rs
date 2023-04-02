#[derive(Clone, Debug, PartialEq)]
pub struct Version {
    major: i32,
    minor: i32,
    build: i32,
    value: String,
}

impl Version {
    pub fn new(major: i32, minor: i32, build: i32) -> Self {
        Self {
            major: major,
            minor: minor,
            build: build,
            value: format!("{}.{}.{}", major, minor, build),
        }
    }

    pub fn parse<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        let parts = s.as_ref().split(".").collect::<Vec<_>>();
        if parts.len() != 3 {
            return None;
        }

        let major = parts[0].parse().ok()?;
        let minor = parts[1].parse().ok()?;
        let build = parts[2].parse().ok()?;

        Some(Version::new(major, minor, build))
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::Version;

    #[test]
    fn test_parse() {
        assert_eq!(Some(Version::new(1, 2, 3)), Version::parse("1.2.3"))
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.build)
    }
}
