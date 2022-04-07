use glam::{Vec4, f32};

pub enum PBRLightType {
    Directional,
    Point,
    Spot,
    FocusedSpot
}



pub struct PBRLight {
    typ:PBRLightType,
    color:Vec4,
    //平行光是辐射照度/光照度，点光源聚光灯是辐射通量/光通量
    intensity:f32,
    //发光强度，发光强度，坎德拉
    luminous_intensity:f32,
    falloff_radius:f32,
    inner_angle:f32,
    outer_angle:f32,

    _cos_outer_squared:f32
}


impl PBRLight {
   //intensity光通量，或者光照度lux
   pub fn set_intensity(&mut self,intensity:f32) {
       let luminous_power = intensity;
       match self.typ {
           PBRLightType::Directional => { 
               self.luminous_intensity = luminous_power;
           },
           PBRLightType::Point => {
               // li = lp / (4 * pi) = 
               self.luminous_intensity = luminous_power * std::f32::consts::FRAC_1_PI * 0.25f32;
           },
           PBRLightType::Spot => {
                // li = lp / pi
               self.luminous_intensity = luminous_power * std::f32::consts::FRAC_1_PI;
           },
           PBRLightType::FocusedSpot => {
                // li = lp / (2 * pi * (1 - cos(cone_outer / 2)))
           },
       }
   }
}