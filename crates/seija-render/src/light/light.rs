use bevy_ecs::system::Resource;
use glam::{Vec4, Vec3};
#[derive(Resource)]
pub struct LightEnv {
    pub is_dirty: bool,
    pub(crate) ambient_color: Vec4
}

impl Default for LightEnv {
    fn default() -> Self {
        Self {
            is_dirty: true,
            ambient_color: Vec4::new(0.05f32, 0.05f32, 0.05f32, 1f32)
        }
    }
}

impl LightEnv {
    pub fn set_ambient_color(&mut self, color: Vec4) {
        self.ambient_color = color;
        self.is_dirty = true;
    }

    pub fn clear_dirty(&mut self) {
        self.is_dirty = false;
    }
}


pub enum LightType {
    Directional,
    Spot,
    Point
}

impl LightType {
    pub fn type_id(&self) -> usize {
        match self {
            LightType::Directional => 0,
            LightType::Spot        => 1,
            LightType::Point       => 2,
        }
    }
}

pub struct Light {
   pub typ:LightType,
   pub color:Vec3,
   pub intensity:f32,
   pub angle:f32,
   pub outer_angle:f32,
   pub range:f32
}

impl Light {
    pub fn directional(color:Vec3,intensity:f32) -> Self {
        Light {
            typ:LightType::Directional,
            color,
            intensity,
            angle:0f32,
            range:0f32,
            outer_angle:0f32,
        }
    }

    pub fn spot(color:Vec3,intensity:f32,range:f32,angle:f32) -> Self {
        Light {
            typ:LightType::Spot,
            color,
            intensity,
            angle,
            range,
            outer_angle:0f32,
        }
    }

    pub fn point(color:Vec3,intensity:f32,range:f32) -> Self {
        Light {
            typ:LightType::Point,
            color,
            intensity,
            angle:0f32,
            outer_angle:0f32,
            range
        }
    }
}