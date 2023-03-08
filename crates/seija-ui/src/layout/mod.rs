pub mod types;
pub mod comps;
pub mod system;
mod measure;
mod arrange;
use lazy_static::lazy_static;
use crate::components::rect2d::Rect2D;

use self::types::LayoutElement;

lazy_static! { static ref VIEW_ID:LayoutElement = LayoutElement::create_view(); }
lazy_static! { static ref RECT2D_ID:Rect2D = Rect2D::default(); }