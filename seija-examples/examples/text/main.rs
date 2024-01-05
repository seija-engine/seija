use bevy_ecs::world::World;
use glam::{Vec3};
use seija_asset::AssetServer;
use seija_core::{CoreStage, StartupStage};
use seija_examples::init_core_app;
use seija_render::{camera::camera::{Orthographic, Camera, SortType}};
use seija_transform::{Transform, events::{EntityMutEx}};
use seija_ui::{update_ui_render,Rect2D, text::{Font, Text, LineMode}, event::UIEventSystem, components::{ui_canvas::UICanvas, sprite::Sprite, canvas::Canvas}, types::{Thickness, AnchorAlign}};

const show_text:&'static str = "体验诗仙李白的豪放飘逸，诗圣杜甫的沉郁顿挫，诗佛王维的诗中有画，诗魔白居易的晓畅通俗，诗豪刘禹锡的旷达沉雄，七绝圣手王昌龄的清畅雄浑，五言长城刘长卿的清旷淡雅，孟浩然的淡泊清逸，高适的慷慨悲壮，岑参的雄奇弘阔，李商隐的精工典丽，杜牧的俊爽清妙";

fn main() {
    let mut app = init_core_app("FRPRender.clj", vec![update_ui_render],None);
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);

    app.run();
}

fn start(world: &mut World) {
    let server = world.get_resource::<AssetServer>().unwrap().clone();
    let mut ui_camera = Camera::from_2d(Orthographic::default());
    ui_camera.sort_type = SortType::Z;
    let event_system = UIEventSystem::default();
    let canvas_id = world.spawn_empty().insert(Transform::default())
                         .insert(ui_camera)
                         .insert(event_system).insert(UICanvas::default()).id();
    
    let server = world.get_resource::<AssetServer>().unwrap().clone();
    let h_font = server.load_sync::<Font>(world, "ui/WenQuanYiMicroHei.ttf", None).unwrap();
    

    let text_id = {
        let mut t = Transform::default();
        t.local.position = Vec3::new(0f32, 0f32, -2f32);
        t.local.position.x = 0f32;
        let rect2d = Rect2D::new(100f32, 100f32);
        let mut text = Text::new(h_font.clone(),show_text.into());
        text.line_mode = LineMode::Wrap;
        text.font_size = 20;
        text.anchor = AnchorAlign::Center;
        let canvas = Canvas::default();
        let e_text = world.spawn((text,rect2d,t,canvas)).set_parent(Some(canvas_id)).id();
        log::error!("text:{:?}",e_text);
        e_text
    };
}



