use smol_str::SmolStr;
use thiserror::Error;

#[derive(Debug,Error)]
pub enum TemplateError {
    #[error("not found template opt {0}")]
    NotFoundOpt(SmolStr),
    #[error("load asset error")]
    LoadAssetError,
    #[error("type cast error")]
    TypeCastError,
    #[error("template miss Res")]
    TemplateMissRes,
    #[error("load children err {0}")]
    LoadChildenError(SmolStr),
    #[error("not found template opt {0}")]
    NotFoundChild(SmolStr),
}