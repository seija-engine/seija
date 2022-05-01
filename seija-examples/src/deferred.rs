use seija_asset::{Assets, Handle};
use seija_core::{CoreStage, StartupStage, window::AppWindow, time::Time};
use seija_examples::{IExamples, add_render_mesh, load_material};
use bevy_ecs::{prelude::{Commands, Entity, Query, Res, ResMut}, system::{IntoSystem,SystemParam}};
use seija_render::{camera::camera::{Camera, Perspective}, material::{Material, MaterialStorage}, resource::{CubeMapBuilder, Mesh, Texture}};
use seija_skeleton3d::{Skeleton, AnimationSet, RuntimeSkeleton, Skin};

use crate::lib::{add_pbr_camera};
pub struct Deferred;

impl IExamples for Deferred {
    fn run(app:&mut seija_app::App) {
       app.add_system2(CoreStage::Startup,StartupStage::Startup, on_start.system());
       app.add_system(CoreStage::Update ,on_update.system());
    }
}





fn on_start(mut commands:Commands,
            mut meshs:ResMut<Assets<Mesh>>,
            mut textures:ResMut<Assets<Texture>>,
            mut skeletons:ResMut<Assets<Skeleton>>,
            mut animations:ResMut<Assets<AnimationSet>>,
            mut rtskeletons:ResMut<Assets<RuntimeSkeleton>>,
            mut skins:ResMut<Assets<Skin>>,
            window:Res<AppWindow>,
            materials:Res<MaterialStorage>) {
    add_pbr_camera(&window, &mut commands);
    load_material("res/materials/deferredPBR.mat.clj", &materials);
}


fn on_update(mut commands:Commands,time:Res<Time>,query:Query<(Entity,&Handle<RuntimeSkeleton>,&Handle<Mesh>)>) {
    //if time.frame() < 200 { return; }
    //for (e,_,_) in query.iter() {
    //    commands.entity(e).remove::<Handle<RuntimeSkeleton>>();
    //    commands.entity(e).remove::<Handle<Mesh>>();
    //}
}