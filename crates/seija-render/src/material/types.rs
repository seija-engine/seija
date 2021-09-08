use std::convert::{TryFrom, TryInto};

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