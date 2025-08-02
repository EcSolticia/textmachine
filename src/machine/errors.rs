use thiserror;
use super::ui::{note::Note, note::Issuer};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    SoftwareIllogic(String),
    #[error("{0}")]
    InvalidUnicode(String),
    #[error("{0}")]
    UnknownError(String),
    #[error("{0}")]
    InvalidInput(String),
    #[error("{0}")]
    PandocError(#[from] pandoc::PandocError),
    #[error("{0}")]
    IoError(#[from] std::io::Error)
}
impl Issuer for Error {
    fn issue(&self) -> Note {
        match self {
            Self::SoftwareIllogic(s) => Note::from("software-illogic", s.to_string()),
            Self::InvalidUnicode(s) => Note::from("invalid-unicode", s.to_string()),
            Self::UnknownError(s) => Note::from("unknown-error", s.to_string()),
            Self::InvalidInput(s) => Note::from("invalid-input", s.to_string()),
            Self::PandocError(s) => Note::from("pandoc-error", s.to_string()),
            Self::IoError(s) => Note::from("io-error", s.to_string())
        }
    }
}
