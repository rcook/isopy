use anyhow::{bail, Error, Result};
use sanitise_file_name::{sanitise_with_options, Options};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug)]
pub struct Url {
    value: String,
    sanitized: String,
}

impl Url {
    pub fn make_path<P>(&self, dir: P) -> Result<PathBuf>
    where
        P: AsRef<Path>,
    {
        let d = dir.as_ref();
        if !d.is_absolute() {
            bail!("Path {} is not absolute", d.display())
        }

        Ok(d.join(&self.sanitized))
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl FromStr for Url {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        fn sanitize(s: &str) -> Result<String> {
            let (prefix, suffix) = match s.rsplit_once(".") {
                None => (s, ""),
                Some(result) => result,
            };

            if prefix.is_empty() {
                bail!("Cannot sanitize URL {s}")
            }

            let mut options = Options::default();
            options.collapse_replacements = true;

            let mut output = sanitise_with_options(&prefix.replace(".", "_"), &options);
            let suffix = sanitise_with_options(suffix, &options);
            if !output.is_empty() {
                output.push('.');
                output.push_str(&suffix)
            }

            Ok(output)
        }

        let value = String::from(s);
        let sanitized = sanitize(s)?;
        Ok(Self { value, sanitized })
    }
}

impl Display for Url {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.value)
    }
}
