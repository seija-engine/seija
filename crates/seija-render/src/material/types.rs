use std::convert::{TryFrom};
use wgpu::{CompareFunction, Face, FrontFace, PolygonMode};
use num_enum::{IntoPrimitive,TryFromPrimitive};

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
    Always
}

impl Into<CompareFunction> for &ZTest {
    fn into(self) -> CompareFunction {
        match self {
            ZTest::Less => CompareFunction::Less,
            ZTest::LEqual => CompareFunction::LessEqual,
            ZTest::Equal => CompareFunction::Equal,
            ZTest::GEqual => CompareFunction::GreaterEqual,
            ZTest::Greater => CompareFunction::Greater,
            ZTest::Always => CompareFunction::Always
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
