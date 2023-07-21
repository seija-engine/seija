use bevy_ecs::prelude::Component;
use seija_core::math::Vec4;

#[derive(Component)]
pub struct Text {
    pub color:Vec4,
    pub font_size:u32,
    pub need_calc_size:bool,
    pub x_size:f32,
    pub font_id:Option<fontdb::ID>,
    pub text:String,
}

impl Default for Text {
    fn default() -> Self {
        Text { 
            color: Vec4::ONE, 
            font_size: 24, 
            font_id: None,
            need_calc_size:false,
            x_size:0f32,
            text: String::default() 
        }
    }
}