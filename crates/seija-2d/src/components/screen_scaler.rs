use bevy_ecs::{event::EventReader, system::{Query, Res}, entity::Entity, query::Changed, component::Component};
use seija_core::{math::{Vec2, Vec3}, window::AppWindow};
use seija_render::camera::camera::{Camera, Projection, Orthographic};
use seija_transform::Transform;
use seija_winit::event::WindowResized;
#[repr(u8)]
pub enum ScalerMode {
    ConstantPixelSize,
    ScreenSizeMatchWH(ScreenSizeMatchWHInfo)
}

pub struct ScreenSizeMatchWHInfo {
    pub design_size:Vec2,
    pub wh_rate:f32
}

#[derive(Component)]
pub struct ScreenScaler {
    pub mode:ScalerMode,
    pub camera_entity:Option<Entity>
}

 
pub(crate) fn screen_scaler_system(mut scaler_list:Query<(Entity,&mut Transform,&ScreenScaler)>,cameras:Query<&Camera>,
                                    resize_event:EventReader<WindowResized>,window:Res<AppWindow>,
                                    scaler_changed:Query<Entity,Changed<ScreenScaler>>) {
    let has_scaler_changed = !scaler_changed.is_empty();
    let has_win_changed = !resize_event.is_empty();
    if has_scaler_changed || has_win_changed {
        for (eitity,mut t,scaler) in scaler_list.iter_mut() {
            let camera_entity = match scaler.camera_entity {
                Some(e) => e,
                None => eitity
            };
            if let Ok(camera) = cameras.get(camera_entity) {
                if let Projection::Ortho(ref ortho) = camera.projection {
                    match &scaler.mode {
                        ScalerMode::ScreenSizeMatchWH(info) => {
                            update_screen_match_width_or_height(info,&mut t,ortho,&window);
                        }
                        _ => {}
                    } 
                }   
            }
        }
    }
}

const K_LOG_BASE:f32 = 2f32;

fn lerp(a:f32,b:f32,t:f32) -> f32 {
    a + (b - a) * t
}

fn update_screen_match_width_or_height(info:&ScreenSizeMatchWHInfo,t:&mut Transform,ortho:&Orthographic,window:&AppWindow) {
    let ortho_w = ortho.left.abs() + ortho.right.abs();
    let ortho_h = ortho.top.abs() + ortho.bottom.abs();
    let window_width   = window.width() as f32;
    let window_height  = window.height() as f32;
    let log_width = (window_width as f32  / info.design_size.x).log2();
    let log_height = (window_height as f32 / info.design_size.y).log2();
   
    let lerp = lerp(log_width, log_height, info.wh_rate);
    //design : window
    let scale_factor = K_LOG_BASE.powf(lerp);

    let w = window_width / scale_factor;
    let h = window_height / scale_factor; 
    let x_scale =  ortho_w / w;
    let y_scale = ortho_h / h;
    t.local.scale = Vec3::new(x_scale, y_scale, 1f32);
}