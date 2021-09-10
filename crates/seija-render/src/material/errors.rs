
use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum MaterialDefReadError {
    #[error("MaterialDef language error")]
    LanguageError,
    #[error("MaterialDef formatError error")]
    FormatError,
    #[error("MaterialDef name does not exist")]
    InvalidName,
    #[error("MaterialDef pass does not exist")]
    InvalidPass,
    #[error("MaterialDef order error")]
    InvalidOrder(String),
}