pub struct Exposure {
    //光圈
    pub aperture:f32,
    //快门
    pub shutter_speed:f32,
    //ISO
    pub sensitivity:f32,
}

impl Default for Exposure {
    fn default() -> Self {
        Self { aperture: 16f32, shutter_speed: 1.0f32 / 125f32, sensitivity: 100f32 }
    }
}

impl Exposure {
    pub fn exposure(ev100:f32) -> f32 {
        // H = (q * t / (N^2)) * L
        // q = 0.65, S = 100 and EV100 = log2(N^2 / t)
        //Lmax = 1.2 * 2^EV100
        1.0f32 / (1.2f32 * ev100.powf(2f32))
    }

    pub fn exposure_self(&self) -> f32 {
        //exposure(ev100(N, t, S))
        let e = (self.aperture * self.aperture) / self.shutter_speed * 100.0f32 / self.sensitivity;
        1.0f32 / (1.2f32 * e)
    }

    pub fn ev100(&self) -> f32 {
        //N = aperture, t = shutter speed and S = sensitivity
        // EV100 = log2((N^2 / t) * (100 / S))
        //
        ((self.aperture * self.aperture) / self.shutter_speed * 100f32 / self.sensitivity).log2()
    }

    //辐射亮度/光亮度
    pub fn luminance(ev100:f32) -> f32 {
         // EV = log2(L * S / K)
         // L = 2^EV100 * K / 100
         // K = 12.5
         // L = 2^EV100 * 12.5 / 100 = 2^EV100 * 0.125
         //std::pow(2.0f, ev100 - 3.0f);
         (ev100 - 3.0f32).powf(2.0f32)
    }

    //辐射亮度/光亮度
    pub fn luminance_self(&self) -> f32 {
        //luminance(ev100(N, t, S))
        let e = (self.aperture * self.aperture) / self.shutter_speed * 100.0f32 / self.sensitivity;
        e * 0.125f32    
    }

    //辐射照度 / 光照度
    pub fn illuminance(ev100:f32) -> f32 {
        // EV100 = log2(E * 100 / C)
        // C = 250
        // EV100 = log2(E * 100 / 250)
        // E = 2^EV100 / (100 / 250)
        // E = 2.5 * 2^EV100
        2.5f32 * ev100.powf(2f32)
    }

    //辐射照度 / 光照度
    pub fn illuminance_self(&self) -> f32 {
        //luminance(ev100(N, t, S))
        let e =  (self.aperture * self.aperture) / self.shutter_speed * 100.0f32 / self.sensitivity;
        e * 0.125f32    
    }
}