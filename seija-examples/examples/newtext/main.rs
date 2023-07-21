use bevy_ecs::prelude::*;
use seija_core::{time::Time, CoreStage, StartupStage, info::EStateInfo};
use seija_examples::init_core_app;
use seija_text::{FontSystem, text::Text};
use seija_transform::Transform;
use seija_ui::{
    components::{canvas::Canvas, rect2d::Rect2D, sprite::Sprite, ui_canvas::UICanvas},
    types::Thickness,
     update_ui_render, 
     event::{EventNode, UIEventType, UIEvent, UIEventSystem}, 
     layout::{types::{LayoutElement, LayoutAlignment, SizeValue, UISize}, comps::Orientation},
};

fn main() {
    let mut app = init_core_app("FRPRender.clj", vec![update_ui_render],None);
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);
    app.add_system(CoreStage::Update, on_update);

    app.run();
}

fn start(world: &mut World) {
    let t = Transform::default();
    let rect2d = Rect2D::default();
    let mut font_system = world.get_resource_mut::<FontSystem>().unwrap();
    font_system.db.load_system_fonts();
    let face_id = font_system.query_family("微软雅黑").unwrap();
    let mut text_2d = Text::default();
    text_2d.text = "我.".into();
    text_2d.font_id = Some(face_id);
    world.spawn_empty().insert((t,rect2d,text_2d));
    
}


fn on_update(mut commands: Commands,state_infos:Query<&EStateInfo>,time:Res<Time>,mut render:EventReader<UIEvent>) {

}
