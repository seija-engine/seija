pub struct GraphSetting {
   pub msaa_samples:u32,
   pub is_hdr:bool,
   pub hdr_format:Option<wgpu::TextureFormat>
}

impl Default for GraphSetting {
    fn default() -> Self {
        Self { 
            msaa_samples: 1,
            is_hdr: true,
            hdr_format:Some(wgpu::TextureFormat::Rgba16Float)
        }
    }
}

impl GraphSetting {
   
}