use std::convert::{TryFrom};
use serde_json::Value;
use smol_str::SmolStr;
use wgpu::{CompareFunction, Face, FrontFace, PolygonMode};
use num_enum::{IntoPrimitive,TryFromPrimitive};

#[derive(IntoPrimitive,Debug,Clone, Copy,Eq,PartialEq,TryFromPrimitive)]
#[repr(usize)]
pub enum RenderPath {
    Forward,
    Deferred,
    ForwardPlus
}

impl TryFrom<&str> for RenderPath {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, String> {
        match value {
            "Forward" => Ok(RenderPath::Forward),
            "Deferred" => Ok(RenderPath::Deferred),
            "ForwardPlus" => Ok(RenderPath::ForwardPlus),
            _ =>  Err(value.to_string())
        }
    }
}

#[derive(IntoPrimitive,Debug,Clone, Copy,Eq,PartialEq,TryFromPrimitive)]
#[repr(usize)]
pub enum RenderOrder {
    BeforeOpaque,
    Opaque,
    AfterOpaque,
    BeforeTransparent,
    Transparent,
    AfterTransparent,
    LinearPostEffects,
    ToneMap,
    DisplayPostEffects,
    Overlay,
    MAX
}

impl TryFrom<&str> for RenderOrder {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, String> {
        match value {
            "BeforeOpaque" => Ok(RenderOrder::BeforeOpaque),
            "Opaque" => Ok(RenderOrder::Opaque),
            "AfterOpaque" => Ok(RenderOrder::AfterOpaque),
            "BeforeTransparent" => Ok(RenderOrder::BeforeTransparent),
            "Transparent" => Ok(RenderOrder::Transparent),
            "AfterTransparent" => Ok(RenderOrder::AfterTransparent),
            "LinearPostEffects" => Ok(RenderOrder::LinearPostEffects),
            "ToneMap" => Ok(RenderOrder::ToneMap),
            "DisplayPostEffects" => Ok(RenderOrder::DisplayPostEffects),
            "Overlay" => Ok(RenderOrder::Overlay),
            _ =>  Err(value.to_string())
        }
    }
}

#[derive(Debug)]
pub enum ZTest {
    Less,
    LEqual,
    Equal,
    GEqual,
    Greater,
    Always,
    Never
}

impl Into<CompareFunction> for &ZTest {
    fn into(self) -> CompareFunction {
        match self {
            ZTest::Less => CompareFunction::Less,
            ZTest::LEqual => CompareFunction::LessEqual,
            ZTest::Equal => CompareFunction::Equal,
            ZTest::GEqual => CompareFunction::GreaterEqual,
            ZTest::Greater => CompareFunction::Greater,
            ZTest::Always => CompareFunction::Always,
            ZTest::Never => CompareFunction::Never,
        }
    }
}

impl TryFrom<&str> for ZTest {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, String> {
        match value {
            "<" => Ok(ZTest::Less),
            "<=" => Ok(ZTest::LEqual),
            "=" => Ok(ZTest::Equal),
            ">=" => Ok(ZTest::GEqual),
            ">" => Ok(ZTest::Greater),
            "always" => Ok(ZTest::Always),
            "never" => Ok(ZTest::Never),
            _ => Err(value.to_string())
        }
    }
}

impl Default for ZTest {
    fn default() -> Self { ZTest::Less }
}
#[derive(Debug)]
pub enum Cull {
    Back,
    Front,
    Off
}

impl Into<Option<Face>> for &Cull {
    fn into(self) -> Option<Face> {
        match self {
            Cull::Off => None,
            Cull::Back => Some(Face::Back),
            Cull::Front => Some(Face::Front),
        }
    }
}

impl TryFrom<&str> for Cull {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, String> {
        match value {
            "Back" => Ok(Cull::Back),
            "Front" => Ok(Cull::Front),
            "Off" => Ok(Cull::Off),
            _ => Err(value.to_string())
        }
    }
}

#[derive(Debug)]
pub struct SFrontFace(pub FrontFace);

impl TryFrom<&str> for SFrontFace {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, String> {
        match value {
            "Ccw" => Ok(SFrontFace(FrontFace::Ccw)),
            "Cw" =>  Ok(SFrontFace(FrontFace::Cw)),
            _ => Err(value.to_string())
        }
    }
}

#[derive(Debug)]
pub struct SPolygonMode(pub PolygonMode);

impl TryFrom<&str> for SPolygonMode {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Fill" => Ok(SPolygonMode(PolygonMode::Fill)),
            "Line" => Ok(SPolygonMode(PolygonMode::Line)),
            "Point" => Ok(SPolygonMode(PolygonMode::Point)),
            _ => Err(value.to_string())
        }
    }
}

impl Default for Cull {
    fn default() -> Self { Cull::Back }
}

pub struct STextureFormat(pub wgpu::TextureFormat);

impl TryFrom<&str> for STextureFormat {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let format = match value {
            "R8Unorm" => wgpu::TextureFormat::R8Unorm,
            "R8Snorm" => wgpu::TextureFormat::R8Snorm,
            "R8Uint" => wgpu::TextureFormat::R8Uint,
            "R8Sint" => wgpu::TextureFormat::R8Sint,

            "R16Uint" => wgpu::TextureFormat::R16Uint,
            "R16Sint" => wgpu::TextureFormat::R16Sint,
            "R16Float" => wgpu::TextureFormat::R16Float,
            "Rg8Unorm" => wgpu::TextureFormat::Rg8Unorm,
            "Rg8Snorm" => wgpu::TextureFormat::Rg8Snorm,
            "Rg8Uint" => wgpu::TextureFormat::Rg8Uint,
            "Rg8Sint" => wgpu::TextureFormat::Rg8Sint,

            "R32Uint" => wgpu::TextureFormat::R32Uint,
            "R32Sint" => wgpu::TextureFormat::R32Sint,
            "R32Float" => wgpu::TextureFormat::R32Float,
            "Rg16Uint" => wgpu::TextureFormat::Rg16Uint,
            "Rg16Sint" => wgpu::TextureFormat::Rg16Sint,
            "Rg16Float" => wgpu::TextureFormat::Rg16Float,
            "Rgba8Unorm" => wgpu::TextureFormat::Rgba8Unorm,
            "Rgba8UnormSrgb" => wgpu::TextureFormat::Rgba8UnormSrgb,
            "Rgba8Snorm" => wgpu::TextureFormat::Rgba8Snorm,
            "Rgba8Uint" => wgpu::TextureFormat::Rgba8Uint,
            "Rgba8Sint" => wgpu::TextureFormat::Rgba8Sint,
            "Bgra8Unorm" => wgpu::TextureFormat::Bgra8Unorm,
            "Bgra8UnormSrgb" => wgpu::TextureFormat::Bgra8UnormSrgb,
            "Rgb10a2Unorm" => wgpu::TextureFormat::Rgb10a2Unorm,
            "Rg11b10Float" => wgpu::TextureFormat::Rg11b10Float,
            
            "Rg32Uint" => wgpu::TextureFormat::Rg32Uint,
            "Rg32Sint" => wgpu::TextureFormat::Rg32Sint,
            "Rg32Float" => wgpu::TextureFormat::Rg32Float,
            "Rgba16Uint" => wgpu::TextureFormat::Rgba16Uint,
            "Rgba16Sint" => wgpu::TextureFormat::Rgba16Sint,
            "Rgba16Float" => wgpu::TextureFormat::Rgba16Float,

            "Rgba32Uint" => wgpu::TextureFormat::Rgba32Uint,
            "Rgba32Sint" => wgpu::TextureFormat::Rgba32Sint,
            "Rgba32Float" => wgpu::TextureFormat::Rgba32Float,

            "Depth32Float" => wgpu::TextureFormat::Depth32Float,
            "Depth24Plus" => wgpu::TextureFormat::Depth24Plus,
            "Depth24PlusStencil8" => wgpu::TextureFormat::Depth24PlusStencil8,

            _ => return Err(())
        };
        Ok(STextureFormat(format))
    }
}

#[derive(Debug)]
pub struct STextureDescriptor(pub wgpu::TextureDescriptor<'static>);

impl TryFrom<&Value> for STextureDescriptor {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let map_value = value.as_object().ok_or(())?;
        let json_format = map_value.get(":format").and_then(Value::as_str).ok_or(())?;
        let format = STextureFormat::try_from(json_format)?;
        let mut default_value = wgpu::TextureDescriptor { 
            label: None,
            size: wgpu::Extent3d::default(),
            mip_level_count: 1,
            sample_count: 1, 
            dimension: wgpu::TextureDimension::D2, 
            format: format.0, 
            view_formats:&[],
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING
        };
        if let Some(sample_count) = map_value.get(":sample-count").and_then(Value::as_i64) {
            default_value.sample_count = sample_count as u32;
        }
        if let Some(w) = map_value.get(":width").and_then(Value::as_i64) {
            default_value.size.width = w as u32;
        }
        if let Some(h) = map_value.get(":height").and_then(Value::as_i64) {
            default_value.size.height = h as u32;
        }
        Ok(STextureDescriptor(default_value))
    }
}

pub struct SBlendFactor(pub wgpu::BlendFactor);

impl TryFrom<&Value> for SBlendFactor {
    type Error = ();
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let factor = match value.as_str().ok_or(())? {
            "Zero" => wgpu::BlendFactor::Zero,
            "One" => wgpu::BlendFactor::One,
            "Src" => wgpu::BlendFactor::Src,
            "OneMinusSrc" => wgpu::BlendFactor::OneMinusSrc,
            "SrcAlpha" => wgpu::BlendFactor::SrcAlpha,
            "OneMinusSrcAlpha" => wgpu::BlendFactor::OneMinusSrcAlpha,
            "Dst" => wgpu::BlendFactor::Dst,
            "OneMinusDst" => wgpu::BlendFactor::OneMinusDst,
            "DstAlpha" => wgpu::BlendFactor::DstAlpha,
            "OneMinusDstAlpha" => wgpu::BlendFactor::OneMinusDstAlpha,
            "SrcAlphaSaturated" => wgpu::BlendFactor::SrcAlphaSaturated,
            "Constant" => wgpu::BlendFactor::Constant,
            "OneMinusConstant" => wgpu::BlendFactor::OneMinusConstant,
            _ => { return Err(()); },
        };
        Ok(SBlendFactor(factor))
    }
}


pub struct SBlendComponent(pub wgpu::BlendComponent);

impl TryFrom<&Value> for SBlendComponent {
    type Error = ();
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let mut arr = value.as_array().ok_or(())?.iter();
        let e1 = arr.next().ok_or(())?;
        let e2 = arr.next().ok_or(())?;
        let e3 = arr.next().ok_or(())?;
        let fa = SBlendFactor::try_from(e1)?;
        let fb = SBlendFactor::try_from(e3)?;
        let o = SBlendOperation::try_from(e2)?;
        let blend = wgpu::BlendComponent {src_factor:fa.0,dst_factor:fb.0,operation:o.0 };
        Ok(SBlendComponent(blend))
    }
}

pub struct SBlendOperation(pub wgpu::BlendOperation);

impl TryFrom<&Value> for SBlendOperation {
    type Error = ();
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let operation = match value.as_str().ok_or(())? {
            "+" => wgpu::BlendOperation::Add,
            "-" => wgpu::BlendOperation::Subtract,
            "r-" => wgpu::BlendOperation::ReverseSubtract,
            "min" => wgpu::BlendOperation::Min,
            "max" => wgpu::BlendOperation::Max,
            _ => return Err(())
        };
        Ok(SBlendOperation(operation))
    }
}

pub struct SBlendState(pub wgpu::BlendState);

impl TryFrom<&Value> for SBlendState {
    type Error = ();
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let json_map = value.as_object().ok_or(())?;
        let color = json_map.get(":color").ok_or(())?;
        let alpha = json_map.get(":alpha").ok_or(())?;
        let blend_color = SBlendComponent::try_from(color)?;
        let blend_alpha = SBlendComponent::try_from(alpha)?;
        Ok(SBlendState(wgpu::BlendState {
            color:blend_color.0,
            alpha:blend_alpha.0
        }))
    }
}
