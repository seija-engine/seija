use std::convert::TryInto;
use bevy_render::texture::TextureFormat;
use bevy_render::pipeline::*;
use serde_json::Value;

pub struct MValue(pub Value);

impl TryInto<FrontFace> for MValue {
    type Error = ();
    fn try_into(self) -> Result<FrontFace, ()> {
        if let Value::String(ref str) = self.0 {
            match str.as_str() {
                "Ccw" => Ok(FrontFace::Ccw),
                "Cw" => Ok(FrontFace::Cw),
                _ => Err(())
            }
        } else {
            Err(())
        }
    }
}

impl TryInto<CullMode> for MValue {
    type Error = ();
    fn try_into(self) -> Result<CullMode, ()> {
        if let Value::String(ref str) = self.0 {
            match str.as_str() {
                "None" => Ok(CullMode::None),
                "Front" => Ok(CullMode::Front),
                "Back" => Ok(CullMode::Back),
                _ => Err(())
            }
        } else {
            Err(())
        }
    }
}

impl TryInto<PrimitiveState> for MValue {
    type Error = ();
    fn try_into(mut self) -> Result<PrimitiveState, ()> {
        let object = self.0.as_object_mut().ok_or(())?;
        let mut ret = PrimitiveState::default();
      
        for (k,v) in object.iter_mut() {
            match k.as_str() {
              ":topology" => ret.topology = serde_json::value::from_value(v.take()).map_err(|_|())?,
              ":strip-index-format" => {
                  ret.strip_index_format = Some(serde_json::value::from_value(v.take()).map_err(|_|())?);
              },
              ":front-face" => ret.front_face = MValue(v.take()).try_into()?,
              ":cull-mode" => ret.cull_mode = MValue(v.take()).try_into()?,
              ":polygon-mode" => ret.polygon_mode = MValue(v.take()).try_into()?,
               _ => ()
            }
        }
        Ok(ret)
    }
}

impl TryInto<PolygonMode> for MValue {
    type Error = ();
    fn try_into(self) -> Result<PolygonMode, ()> {
        if let Value::String(ref str) = self.0 {
            match str.as_str() {
                "Fill" => Ok(PolygonMode::Fill),
                "Line" => Ok(PolygonMode::Line),
                "Point" => Ok(PolygonMode::Point),
                _ => Err(())
            }
        } else {
            Err(())
        }
    }
}

impl TryInto<TextureFormat> for MValue {
    type Error = ();
    fn try_into(self) -> Result<TextureFormat, ()> {
        let str = self.0.as_str().ok_or(())?;
        match str {
            "R8Unorm" => Ok(TextureFormat::R8Unorm),
            "R8Snorm" => Ok(TextureFormat::R8Snorm), //1
            "R8Uint" => Ok(TextureFormat::R8Uint),   //2
            "R8Sint" => Ok(TextureFormat::R8Sint),   //3

            "R16Uint" => Ok(TextureFormat::R16Uint),  //4
            "R16Sint" => Ok(TextureFormat::R16Sint),  //5
            "R16Float" => Ok(TextureFormat::R16Float), //6
            "Rg8Unorm" => Ok(TextureFormat::Rg8Unorm), //7
            "Rg8Snorm" => Ok(TextureFormat::Rg8Snorm), //8
            "Rg8Uint" => Ok(TextureFormat::Rg8Uint),  //9
            "Rg8Sint" => Ok(TextureFormat::Rg8Sint),  //10

            "R32Uint" => Ok(TextureFormat::R32Uint), //11
            "R32Sint" => Ok(TextureFormat::R32Sint), //12
            "R32Float" => Ok(TextureFormat::R32Float), //13
            "Rg16Uint" => Ok(TextureFormat::Rg16Uint),  //14
            "Rg16Sint" => Ok(TextureFormat::Rg16Sint),  //15
            "Rg16Float" => Ok(TextureFormat::Rg16Float), //16
            "Rgba8Unorm" => Ok(TextureFormat::Rgba8Unorm), //17
            "Rgba8UnormSrgb" => Ok(TextureFormat::Rgba8UnormSrgb), //18
            "Rgba8Snorm" => Ok(TextureFormat::Rgba8Snorm),  //19
            "Rgba8Uint" => Ok(TextureFormat::Rgba8Uint),  //20
            "Rgba8Sint" => Ok(TextureFormat::Rgba8Sint), //21
            "Bgra8Unorm" => Ok(TextureFormat::Bgra8Unorm),  //22
            "Bgra8UnormSrgb" => Ok(TextureFormat::Bgra8UnormSrgb),  //23
            "Rgb10a2Unorm" => Ok(TextureFormat::Rgb10a2Unorm),  //24
            "Rg11b10Float" => Ok(TextureFormat::Rg11b10Float),  //25

            "Rg32Uint" => Ok(TextureFormat::Rg32Uint),  //26
            "Rg32Sint" => Ok(TextureFormat::Rg32Sint), //27
            "Rg32Float" => Ok(TextureFormat::Rg32Float),  //28
            "Rgba16Uint" => Ok(TextureFormat::Rgba16Uint),  //29
            "Rgba16Sint" => Ok(TextureFormat::Rgba16Sint),  //30
            "Rgba16Float" => Ok(TextureFormat::Rgba16Float),  //31
            "Rgba32Uint" => Ok(TextureFormat::Rgba32Uint),  //32
            "Rgba32Sint" => Ok(TextureFormat::Rgba32Sint),  //33
            "Rgba32Float" => Ok(TextureFormat::Rgba32Float),  //34

            "Depth32Float" => Ok(TextureFormat::Depth32Float),  //35
            "Depth24Plus" => Ok(TextureFormat::Depth24Plus),  //36
            "Depth24PlusStencil8" => Ok(TextureFormat::Depth24PlusStencil8),  //37
           _ => Err(())
        }
    }
}

impl TryInto<CompareFunction> for MValue {
    type Error = ();
    fn try_into(self) -> Result<CompareFunction, ()> {
        let str = self.0.as_str().ok_or(())?;
        match str {
            "Never" => Ok(CompareFunction::Never),
            "Less" => Ok(CompareFunction::Less),
            "Equal" => Ok(CompareFunction::Equal),
            "LessEqual" => Ok(CompareFunction::LessEqual),
            "Greater" => Ok(CompareFunction::Greater),
            "NotEqual" => Ok(CompareFunction::NotEqual),
            "GreaterEqual" => Ok(CompareFunction::GreaterEqual),
            "Always" => Ok(CompareFunction::Always),
            _ => Err(())
        }
    }
}

impl TryInto<StencilOperation> for MValue {
    type Error = ();
    fn try_into(self) -> Result<StencilOperation, ()> {
        let str = self.0.as_str().ok_or(())?;
        match str {
            "Keep" => Ok(StencilOperation::Keep),
            "Zero" => Ok(StencilOperation::Zero),
            "Replace" => Ok(StencilOperation::Replace),
            "Invert" => Ok(StencilOperation::Invert),
            "IncrementClamp" => Ok(StencilOperation::IncrementClamp),
            "DecrementClamp" => Ok(StencilOperation::DecrementClamp),
            "IncrementWrap" => Ok(StencilOperation::IncrementWrap),
            "DecrementWrap" => Ok(StencilOperation::DecrementWrap),
            _ => Err(())
        }
    }
}

impl TryInto<StencilFaceState> for MValue {
    type Error = ();
    fn try_into(mut self) -> Result<StencilFaceState, ()> {
        let mut ret = StencilFaceState::IGNORE.clone();
        let map = self.0.as_object_mut().ok_or(())?;
        for (k,v) in map {
            match k.as_str() {
                ":compare" => ret.compare = MValue(v.take()).try_into()?,
                ":fail-op" => ret.fail_op = MValue(v.take()).try_into()?,
                ":depth-fail-op" => ret.fail_op = MValue(v.take()).try_into()?,
                ":pass-op" => ret.fail_op = MValue(v.take()).try_into()?,
                _ => ()
            }
        }
        Ok(ret)
    }
}

impl TryInto<StencilState> for MValue {
    type Error = ();
    fn try_into(mut self) -> Result<StencilState, ()> {
        let mut ret = StencilState {
            front: StencilFaceState::IGNORE,
            back: StencilFaceState::IGNORE,
            read_mask: 0,
            write_mask: 0,
        };
        let map = self.0.as_object_mut().ok_or(())?;
        for (k,v) in map {
            match k.as_str() {
                ":front" => ret.front = MValue(v.take()).try_into()?,
                ":back-op" => ret.back = MValue(v.take()).try_into()?,
                ":read-mask" => ret.read_mask = v.take().as_u64().ok_or(())? as u32,
                ":write-mask" => ret.write_mask = v.take().as_u64().ok_or(())? as u32,
                _ => ()
            }
        }
        Ok(ret)
    }
}

impl TryInto<DepthBiasState> for MValue {
    type Error = ();
    fn try_into(mut self) -> Result<DepthBiasState, ()> {
        let mut ret =  DepthBiasState {constant: 0, slope_scale: 0.0, clamp: 0.0};
        let map = self.0.as_object_mut().ok_or(())?;
        for (k,v) in map {
            match k.as_str() {
                ":constant" => ret.constant = v.as_i64().ok_or(())? as i32,
                ":slope-scale" => ret.slope_scale = v.as_f64().ok_or(())? as f32,
                ":clamp" => ret.clamp = v.as_f64().ok_or(())? as f32,
                _ => ()
            }
        }
        Ok(ret)
    }
}

impl TryInto<MultisampleState> for MValue {
    type Error = ();
    fn try_into(mut self) -> Result<MultisampleState, ()> {
        let mut ret =  MultisampleState {count: 1, mask: !0, alpha_to_coverage_enabled: false};
        let map = self.0.as_object_mut().ok_or(())?;
        for (k,v) in map {
            match k.as_str() {
                ":constant" => ret.count = v.as_u64().ok_or(())? as u32,
                ":mask" => ret.mask = v.as_u64().ok_or(())?,
                ":alpha-to-coverage-enabled" => ret.alpha_to_coverage_enabled = v.as_bool().ok_or(())?,
                _ => ()
            }
        }
        Ok(ret)
    }

}

impl TryInto<DepthStencilState> for MValue {
    type Error = ();
    fn try_into(mut self) -> Result<DepthStencilState, ()> {
        let mut ret = DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::Less,
            stencil: StencilState {
                front: StencilFaceState::IGNORE,
                back: StencilFaceState::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
            bias: DepthBiasState {
                constant: 0,
                slope_scale: 0.0,
                clamp: 0.0,
            },
            clamp_depth:false
        };
        let map = self.0.as_object_mut().ok_or(())?;
        for (k,v) in map {
            match k.as_str() {
                ":format" => ret.format = MValue(v.take()).try_into()?,
                ":depth-write-enabled" => ret.depth_write_enabled = v.as_bool().ok_or(())?,
                ":depth-compare" => ret.depth_compare = MValue(v.take()).try_into()?,
                ":stencil" => ret.stencil = MValue(v.take()).try_into()?,
                ":bias" => ret.bias = MValue(v.take()).try_into()?,
                ":clamp-depth" => ret.clamp_depth = v.as_bool().ok_or(())?,
                _ => ()
            }
        }
        Ok(ret)
    }
}

impl TryInto<BlendFactor> for MValue {
    type Error = ();
    fn try_into(self) -> Result<BlendFactor, ()> {
        let str = self.0.as_str().ok_or(())?;
        match str {
         "Zero" => Ok(BlendFactor::Zero),
         "One" => Ok(BlendFactor::One),
         "SrcColor" => Ok(BlendFactor::SrcColor),
         "OneMinusSrcColor" => Ok(BlendFactor::OneMinusSrcColor),
         "SrcAlpha" => Ok(BlendFactor::SrcAlpha),
         "OneMinusSrcAlpha" => Ok(BlendFactor::OneMinusSrcAlpha),
         "DstColor" => Ok(BlendFactor::DstColor),
         "OneMinusDstColor" => Ok(BlendFactor::OneMinusDstColor),
         "DstAlpha" => Ok(BlendFactor::DstAlpha),
         "OneMinusDstAlpha" => Ok(BlendFactor::OneMinusDstAlpha),
         "SrcAlphaSaturated" => Ok(BlendFactor::SrcAlphaSaturated),
         "BlendColor" => Ok(BlendFactor::BlendColor),
         "OneMinusBlendColor" => Ok(BlendFactor::OneMinusBlendColor),
          _ => Err(())
        }
    }
}

impl TryInto<BlendOperation> for MValue {
    type Error = ();
    fn try_into(self) -> Result<BlendOperation, ()> {
        let str = self.0.as_str().ok_or(())?;
        match str {
         "Add" => Ok(BlendOperation::Add),
         "Subtract" => Ok(BlendOperation::Subtract),
         "ReverseSubtract" => Ok(BlendOperation::ReverseSubtract),
         "Min" => Ok(BlendOperation::Min),
         "Max" => Ok(BlendOperation::Max),
          _ => Err(())
        }
    }
}

impl TryInto<ColorTargetState> for MValue {
    type Error = ();
    fn try_into(mut self) -> Result<ColorTargetState, ()> {
        let mut ret = ColorTargetState { 
            format:TextureFormat::default(),
            alpha_blend:BlendState::REPLACE,
            color_blend:BlendState::REPLACE,
            write_mask:ColorWrite::ALL,
        };
        let map = self.0.as_object_mut().ok_or(())?;
        for (k,v) in map  {
            match k.as_str() {
                ":format" => ret.format = MValue(v.take()).try_into()?,
                ":alpha-blend" => ret.alpha_blend = MValue(v.take()).try_into()?,
                ":color-blend" => ret.color_blend = MValue(v.take()).try_into()?,
                ":write-mask" => ret.write_mask = MValue(v.take()).try_into()?,
                 _ => ()
            }
        }
        Err(())
    }
}

impl TryInto<BlendState> for MValue {
    type Error = ();
    fn try_into(mut self) -> Result<BlendState, ()> {
        let mut ret = BlendState::REPLACE;
        let map = self.0.as_object_mut().ok_or(())?;
        for (k,v) in map  {
            match k.as_str() {
                ":src-factor" => ret.src_factor = MValue(v.take()).try_into()?,
                ":dst-factor" => ret.dst_factor = MValue(v.take()).try_into()?,
                ":operation" => ret.operation = MValue(v.take()).try_into()?,
                _ => ()
            }
        }
        Ok(ret)
    }
}

impl TryInto<ColorWrite> for MValue {
    type Error = ();
    fn try_into(self) -> Result<ColorWrite, ()> {
        let str = self.0.as_str().ok_or(())?;
        match str {
            "RED" => Ok(ColorWrite::RED),
            "GREEN" => Ok(ColorWrite::GREEN),
            "BLUE" => Ok(ColorWrite::BLUE),
            "ALPHA" => Ok(ColorWrite::ALPHA),
            "COLOR" => Ok(ColorWrite::COLOR),
            "ALL" => Ok(ColorWrite::ALL),
            _ => Err(())
        }
    }
}

#[test]
fn test_der() {
   let json = r##"
        {
            ":topology" : "LineStrip"
        }
   "##;

   let v:Value = serde_json::from_str(json).unwrap();
   let s:PrimitiveState = MValue(v).try_into().unwrap();
   dbg!(s);

}