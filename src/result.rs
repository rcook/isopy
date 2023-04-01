pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Fatal(String),
    Reportable(String, ReportableError),
    User(String),
    Other(Box<dyn std::error::Error>),
}

#[derive(Debug)]
pub enum ReportableError {
    CouldNotGetIsopyDir,
}

impl<E> From<E> for Error
where
    E: std::error::Error + 'static,
{
    fn from(e: E) -> Self {
        Self::Other(Box::new(e))
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
