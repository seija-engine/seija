use thiserror::{Error};
#[derive(Debug,Error)]
pub enum RenderErrors {
    #[error("not found material storage")]
    NotFoundMaterialStorage,
    #[error("not found Assets<Mesh>")]
    NotFoundAssetsMesh
}