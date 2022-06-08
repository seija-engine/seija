use bevy_ecs::prelude::{Commands, Entity, IntoSystem, Query, Res, ResMut};
use glam::{Quat, Vec3};
use seija_asset::{Assets, Handle};
use seija_core::{window::AppWindow, CoreStage, StartupStage, time::Time};
use seija_examples::{load_texture, IExamples};

use seija_render::{
    light::LightEnv,
    material::MaterialStorage,
    resource::{
        shape::{Cube, Sphere},
        Mesh, Texture,
    },
    wgpu, graph::nodes::{ShadowMapNode, ShadowLight},
};
use seija_transform::Transform;

pub struct ShadowTest;

impl IExamples for ShadowTest {
    fn run(app: &mut seija_app::App) {
        app.add_system2(CoreStage::Startup, StartupStage::Startup, on_start.system());
        app.add_system(CoreStage::Update, on_update.system());

        app.world.insert_resource(ShadowLight::new(Vec3::new(0f32,-0.5f32, -0.5f32)));
    }
}

fn on_start(
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut textures: ResMut<Assets<Texture>>,
    window: Res<AppWindow>,
    materials: Res<MaterialStorage>,
) {
    
}

fn on_update(
    mut light: ResMut<LightEnv>
    ,time:Res<Time>,
) {
}
