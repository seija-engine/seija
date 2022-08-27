use thiserror::Error;

#[derive(Debug,Error)]
pub enum AssetError {
    #[error("not found loader")]
    NotFoundLoader,
    #[error("type cast error")]
    TypeCastError,
    #[error("type cast error")]
    NotFoundAssets
}
