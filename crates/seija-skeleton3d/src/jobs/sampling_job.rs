use seija_transform::TransformMatrix;

use crate::Animation;

pub struct SamplingJob<'a,'b> {
    animation:&'a Animation,
    ratio:f32,
    output:&'b mut Vec<TransformMatrix>
}

pub struct  SamplingJobContext {
    
}