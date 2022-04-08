use glam::{f32, Vec2, Vec4};

#[derive(PartialEq, Eq)]
pub enum PBRLightType {
    Directional,
    Point,
    Spot,
    FocusedSpot,
}

impl Default for PBRLightType {
    fn default() -> Self { PBRLightType::Directional }
}

#[derive(Default)]
pub struct PBRLight {
    typ: PBRLightType,
    pub color: Vec4,
    //点光源聚光灯是辐射通量/光通量,平行光是辐射照度/光照度，
    intensity: f32,
    //发光强度，坎德拉
    falloff_radius: f32,
    inner_angle: f32,
    outer_angle: f32,

    _cos_outer_squared: f32,
    _luminous_intensity: f32,
    _scale_offset: Vec2,
    _squared_fall_offinv: f32,
}

impl PBRLight {
    pub fn directional(color: Vec4, intensity: f32) -> Self {
        let mut light = PBRLight::default();
        light.typ = PBRLightType::Directional;
        light.color = color;
        light.set_intensity(intensity);
        light
    }

    pub fn point(color: Vec4, intensity: f32, falloff: f32) -> Self {
        let mut light = PBRLight::default();
        light.typ = PBRLightType::Point;
        light.color = color;
        light.set_falloff(falloff);
        light.set_intensity(intensity);
        light
    }

    pub fn spot(color: Vec4, intensity: f32, falloff: f32, inner: f32, outer: f32,is_focused:bool) -> Self {
        let mut light = PBRLight::default();
        light.typ = if is_focused { PBRLightType::FocusedSpot } else { PBRLightType::Spot };
        light.color = color;
        light.set_falloff(falloff);
        light.set_spot_cone(inner, outer);
        light.set_intensity(intensity);
        light
    }

    //intensity光通量，或者光照度lux
    pub fn set_intensity(&mut self, intensity: f32) {
        let luminous_power = intensity;
        self.intensity = luminous_power;
        self.calc_intensity();
    }

    fn calc_intensity(&mut self) {
        match self.typ {
            PBRLightType::Directional => {
                self._luminous_intensity = self.intensity;
            }
            PBRLightType::Point => {
                // li = lp / (4 * pi)
                self._luminous_intensity = self.intensity * std::f32::consts::FRAC_1_PI * 0.25f32;
            }
            PBRLightType::Spot => {
                // li = lp / pi
                self._luminous_intensity = self.intensity * std::f32::consts::FRAC_1_PI;
            }
            PBRLightType::FocusedSpot => {
                // li = lp / (2 * pi * (1 - cos(cone_outer / 2)))
                let cos_outer = self._cos_outer_squared.sqrt();
                self._luminous_intensity =
                    self.intensity / ((1f32 - cos_outer) * std::f32::consts::TAU);
            }
        }
    }

    pub fn set_falloff(&mut self, falloff: f32) {
        self.falloff_radius = falloff;
        self.calc_falloff();
    }

    fn calc_falloff(&mut self) {
        let sq_falloff = self.falloff_radius * self.falloff_radius;
        self._squared_fall_offinv = 0f32;
        if sq_falloff > 0f32 {
            self._squared_fall_offinv = 1f32 / sq_falloff;
        }
    }

    pub fn set_spot_cone(&mut self, inner: f32, outer: f32) {
        self.inner_angle = inner;
        self.outer_angle = outer;
        self.calc_spot();
    }

    fn calc_spot(&mut self) {
        let mut inner_clamped = self
            .inner_angle
            .abs()
            .clamp(0.5f32.to_radians(), std::f32::consts::PI * 2f32);
        let outer_clamped = self
            .outer_angle
            .abs()
            .clamp(0.5f32.to_radians(), std::f32::consts::PI * 2f32);
        inner_clamped = inner_clamped.min(outer_clamped);
        let cos_outer = outer_clamped.cos();
        let cos_inner = inner_clamped.cos();
        let scale = 1.0f32 / (1.0f32 / 1024.0f32).max(cos_inner - cos_outer);
        let offset = -cos_outer * scale;
        self._scale_offset.x = scale;
        self._scale_offset.y = offset;
        self._cos_outer_squared = cos_outer * cos_outer;
        if self.typ == PBRLightType::FocusedSpot {
            self._luminous_intensity =
                self.intensity / ((1f32 - cos_outer) * std::f32::consts::TAU);
        }
    }
}
