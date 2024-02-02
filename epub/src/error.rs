use thiserror::Error;
use zip::result::ZipError;

#[derive(Debug, Error)]
pub enum EpubError {
    #[error("ZipError: `{0}`")]
    ZipError(#[from] ZipError),
    #[error("ReaderError: `{0}`")]
    ReaderError(String),
    #[error("ContainerError: `{0}`")]
    ContainerError(String),
    #[error("XMLError: `{0}`")]
    XmlError(String),
    #[error("ParseError: `{0}`")]
    ParseError(String),
    #[error("ForatError: `{0}`")]
    FormatError(String),
    #[error("UrlError: `{0}`")]
    UrlError(String),
    #[error("CfiError:`{0}`")]
    CfiError(String),
}

pub type Result<T> = std::result::Result<T, EpubError>;
