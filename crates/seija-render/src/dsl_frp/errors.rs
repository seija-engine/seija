use thiserror::Error;
#[derive(Debug,Error)]

pub(crate) enum Errors {
    #[error("not found info set")]
    NotFoundInfoSet,
    #[error("type cast error {0}")]
    TypeCastError(&'static str),
    #[error("not found userdata {0}")]
    NotFoundUserData(&'static str)
}