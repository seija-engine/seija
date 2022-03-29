use glam::{Vec4};

pub struct LightEnv {
    pub is_dirty: bool,
    pub(crate) ambient_color: Vec4
}

impl Default for LightEnv {
    fn default() -> Self {
        Self {
            is_dirty: true,
            ambient_color: Vec4::ONE
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


pub struct Light {
   type_light:LightType,
   color:Vec4,
   intensity:f32,
   angle:f32,
   range:f32
}

impl Light {
    pub fn directional(color:Vec4,intensity:f32) -> Self {
        Light {
            type_light:LightType::Directional,
            color,
            intensity,
            angle:0f32,
            range:0f32
        }
    }
}