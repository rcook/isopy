pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Env(std::env::VarError),
    Fatal(String),
    IO(std::io::Error),
    Json(serde_json::Error),
    Regex(regex::Error),
    Reportable(String, ReportableError),
    Reqwest(reqwest::Error),
    Template(indicatif::style::TemplateError),
    User(String),
    Yaml(serde_yaml::Error),
}

#[derive(Debug)]
pub enum ReportableError {
    CouldNotGetIsopyDir,
}

impl From<indicatif::style::TemplateError> for Error {
    fn from(e: indicatif::style::TemplateError) -> Self {
        Self::Template(e)
    }
}

impl From<regex::Error> for Error {
    fn from(e: regex::Error) -> Self {
        Self::Regex(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        Self::Yaml(e)
    }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Self::Env(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

pub fn user<M>(msg: M) -> Error
where
    M: Into<String>,
{
    Error::User(msg.into())
}

pub fn could_not_get_isopy_dir<M>(msg: M) -> Error
where
    M: Into<String>,
{
    Error::Reportable(msg.into(), ReportableError::CouldNotGetIsopyDir)
}

pub fn fatal<M>(msg: M) -> Error
where
    M: Into<String>,
{
    Error::Fatal(msg.into())
}
