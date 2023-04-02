use std::path::{Path, PathBuf};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    User(String),
    Reportable { message: String, info: ErrorInfo },
    Fatal(String),
    Other(Box<dyn std::error::Error>),
}

#[derive(Debug)]
pub enum ErrorInfo {
    CouldNotInferIsopyDir,
    FileNotFound {
        path: PathBuf,
        inner: Option<std::io::Error>,
    },
    YamlError {
        path: PathBuf,
        inner: Option<serde_yaml::Error>,
    },
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

pub fn could_not_infer_isopy_dir() -> Error {
    Error::Reportable {
        message: String::from(
            "Could not infer isopy cache directory location: please specify using --dir option",
        ),
        info: ErrorInfo::CouldNotInferIsopyDir,
    }
}

pub fn file_not_found<P>(path: P, inner: Option<std::io::Error>) -> Error
where
    P: Into<PathBuf>,
{
    let p = path.into();
    let m = format!("Could not find file {}", p.display());
    Error::Reportable {
        message: m,
        info: ErrorInfo::FileNotFound {
            path: p,
            inner: inner,
        },
    }
}

pub fn fatal<M>(msg: M) -> Error
where
    M: Into<String>,
{
    Error::Fatal(msg.into())
}

/// Attach file path to error
pub fn translate_io_error<P>(e: std::io::Error, path: P) -> Error
where
    P: AsRef<Path>,
{
    if e.kind() == std::io::ErrorKind::NotFound {
        file_not_found(path.as_ref(), Some(e))
    } else {
        e.into()
    }
}

/// Attach file path to error
pub fn translate_yaml_error<P>(e: serde_yaml::Error, path: P) -> Error
where
    P: AsRef<Path>,
{
    let p = PathBuf::from(path.as_ref());
    let m = format!("{} in {}", e.to_string(), p.display());
    Error::Reportable {
        message: m,
        info: crate::result::ErrorInfo::YamlError {
            path: p,
            inner: Some(e),
        },
    }
}
