
use bevy_ecs::prelude::Component;
use glam::{Mat4, Quat, Vec3};
use lazy_static::{lazy_static};

lazy_static! {
    pub static ref TRANSFORM_MAT_ID:TransformMatrix = TransformMatrix::default(); 
}

#[derive(Debug,PartialEq,Clone)]
pub struct TransformMatrix {
    pub scale:Vec3,
    pub rotation:Quat,
    pub position:Vec3
}

impl TransformMatrix {
    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale,self.rotation, self.position)
    }
}

impl Into<TransformMatrix> for Mat4 {
    fn into(self) -> TransformMatrix {
        let (scale, rotation, translation) = self.to_scale_rotation_translation();
        TransformMatrix {scale,rotation,position:translation }
    }
}

impl TransformMatrix {
    
    #[inline]
    pub fn mul_vec3(&self, mut value: Vec3) -> Vec3 {
        value = self.rotation * value;
        value = self.scale * value;
        value += self.position;
        value
    }

    pub fn mul_transform(&self, transform: &TransformMatrix) -> TransformMatrix {
        let position = self.mul_vec3(transform.position);
        let rotation = self.rotation * transform.rotation;
        let scale = self.scale * transform.scale;
        TransformMatrix {
            position,
            rotation,
            scale,
        }
    }

}

impl Default for TransformMatrix {
    fn default() -> TransformMatrix {
        TransformMatrix {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}


#[derive(Default,PartialEq,Clone,Debug,Component)]
pub struct Transform {
    pub local:TransformMatrix,
    pub(crate) global:TransformMatrix
}

impl Transform {
    pub fn global(&self) -> &TransformMatrix {
        &self.global
    }

    pub fn set_global(&mut self,mat:TransformMatrix) {
        self.global = mat
    }

    pub fn from_matrix(matrix:Mat4) -> Transform {
        let (scale, rotation, translation) = matrix.to_scale_rotation_translation();
        Transform::new(translation, rotation, scale)
    }

    pub fn from_t_matrix(t:TransformMatrix) -> Transform {
        Transform {
            local:t,
            global:TransformMatrix::default()
        }
    }

    pub fn new(position:Vec3,rotation:Quat,scale:Vec3) -> Transform {
        Transform {
            local: TransformMatrix {scale,rotation,position },
            global:TransformMatrix::default()
        }
    }
}



