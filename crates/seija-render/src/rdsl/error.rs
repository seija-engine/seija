use thiserror::{Error};
#[derive(Error,Debug)]
pub enum ScriptError {
    #[error("error declare unifrom")]
    DeclareUnifromErr,
}