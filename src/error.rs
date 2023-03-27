pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    User(String),
    Reportable(String, ReportableError),
    IO(std::io::Error),
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    Template(indicatif::style::TemplateError),
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

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJson(e)
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
