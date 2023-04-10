use std::path::{Path, PathBuf};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    User { message: String },
    Reportable { message: String, info: ErrorInfo },
    Fatal { message: String },
    Other { inner: Box<dyn std::error::Error> },
}

#[derive(Debug)]
pub enum ErrorInfo {
    CouldNotInferIsopyDir,
    FileAlreadyExists {
        path: PathBuf,
        inner: Option<std::io::Error>,
    },
    FileNotFound {
        path: PathBuf,
        inner: Option<std::io::Error>,
    },
    Json {
        path: PathBuf,
        inner: Option<serde_json::Error>,
    },
    Yaml {
        path: PathBuf,
        inner: Option<serde_yaml::Error>,
    },
}

impl<E> From<E> for Error
where
    E: std::error::Error + 'static,
{
    fn from(e: E) -> Self {
        other(Box::new(e))
    }
}

pub fn other(inner: Box<dyn std::error::Error>) -> Error {
    Error::Other { inner }
}

pub fn user<M>(message: M) -> Error
where
    M: Into<String>,
{
    Error::User {
        message: message.into(),
    }
}

pub fn could_not_infer_isopy_dir() -> Error {
    Error::Reportable {
        message: String::from(
            "Could not infer isopy cache directory location: please specify using --dir option",
        ),
        info: ErrorInfo::CouldNotInferIsopyDir,
    }
}

pub fn file_already_exists<P>(path: P, inner: Option<std::io::Error>) -> Error
where
    P: Into<PathBuf>,
{
    let p = path.into();
    let m = format!("File {} already exists", p.display());
    Error::Reportable {
        message: m,
        info: ErrorInfo::FileAlreadyExists { path: p, inner },
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
        info: ErrorInfo::FileNotFound { path: p, inner },
    }
}

pub fn fatal<M>(message: M) -> Error
where
    M: Into<String>,
{
    Error::Fatal {
        message: message.into(),
    }
}

/// Attach file path to error
pub fn translate_io_error<P>(e: std::io::Error, path: P) -> Error
where
    P: AsRef<Path>,
{
    use std::io::ErrorKind::*;

    match e.kind() {
        AlreadyExists => file_already_exists(path.as_ref(), Some(e)),
        NotFound => file_not_found(path.as_ref(), Some(e)),
        _ => e.into(),
    }
}

/// Attach file path to error
pub fn translate_json_error<P>(e: serde_json::Error, path: P) -> Error
where
    P: AsRef<Path>,
{
    let p = PathBuf::from(path.as_ref());
    let m = format!("{} in {}", e, p.display());
    Error::Reportable {
        message: m,
        info: crate::result::ErrorInfo::Json {
            path: p,
            inner: Some(e),
        },
    }
}

/// Attach file path to error
pub fn translate_yaml_error<P>(e: serde_yaml::Error, path: P) -> Error
where
    P: AsRef<Path>,
{
    let p = PathBuf::from(path.as_ref());
    let m = format!("{} in {}", e, p.display());
    Error::Reportable {
        message: m,
        info: crate::result::ErrorInfo::Yaml {
            path: p,
            inner: Some(e),
        },
    }
}
