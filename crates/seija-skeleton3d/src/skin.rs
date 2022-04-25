use glam::Mat4;
use seija_core::{TypeUuid,uuid::{Uuid}};

#[derive(Debug,Default,TypeUuid)]
#[uuid = "bd7c7dbf-18f9-4917-954e-bc5a1d6b6b69"]
pub struct Skin {
    inverse_matrices:Vec<Mat4>
}

impl Skin {
    pub fn new(inverse_matrices:Vec<Mat4>) -> Self {
        Skin {inverse_matrices}
    }

    pub fn mats(&self) -> &Vec<Mat4> {
        &self.inverse_matrices
    }
}