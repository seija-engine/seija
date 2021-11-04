use std::{ops::{Deref, DerefMut}};

use bevy_ecs::prelude::{Changed, Entity, Query, QuerySet, With, Without};
use glam::{Mat4, Quat, Vec2, Vec3};

use crate::hierarchy::{Children, Parent};

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


#[derive(Default,PartialEq,Clone,Debug)]
pub struct Transform {
    pub local:TransformMatrix,
    pub(crate) global:TransformMatrix
}

impl Transform {
    pub fn global(&self) -> &TransformMatrix {
        &self.global
    }

    pub fn from_matrix(matrix:Mat4) -> Transform {
        let (scale, rotation, translation) = matrix.to_scale_rotation_translation();
        Transform::new(translation, rotation, scale)
    }

    pub fn new(position:Vec3,rotation:Quat,scale:Vec3) -> Transform {
        Transform {
            local: TransformMatrix {scale,rotation,position },
            global:TransformMatrix::default()
        }
    }
}




pub(crate) fn update_transform_system(
                                children_query: Query<Option<&Children>, (With<Parent>, With<Transform>)>,
                                query_set:QuerySet<(
                                                    Query<(Entity, Option<&Children>, &mut Transform),Without<Parent>>,
                                                    Query<&mut Transform, With<Parent>>,
                                                    Query<Entity, Changed<Transform>>)>
                               ) {
    unsafe {
        for (entity, children, mut transform) in query_set.q0().iter_unsafe() {
            let mut changed = false;
            if query_set.q2().get_unchecked(entity).is_ok() {
                transform.global = transform.local.clone();
                changed = true;
            }
            if let Some(children) = children {
                for child in children.0.iter() {
                    update_transform(transform.global(),&children_query,&query_set,*child,changed);
                }
            }
        }
    }    
}

unsafe fn update_transform(parent:&TransformMatrix,children_query: &Query<Option<&Children>, (With<Parent>, With<Transform>)>,
                        query_set:&QuerySet<(
                        Query<(Entity, Option<&Children>, &mut Transform),Without<Parent>>,
                        Query<&mut Transform, With<Parent>>,
                        Query<Entity, Changed<Transform>>
                       )>,entity: Entity,mut changed: bool) {
    changed |= query_set.q2().get(entity).is_ok();
    let global_matrix = {
        if let Ok(mut transform) = query_set.q1().get_unchecked(entity) {
            if changed {
                transform.global = parent.mul_transform(&transform.local);
            }
            transform.global().clone()
        } else {
            return;
        }
    };
    if let Ok(Some(children)) = children_query.get(entity) {
        for child in children.0.iter() {
            update_transform(&global_matrix,&children_query,query_set,*child,changed);
        }
    }
}