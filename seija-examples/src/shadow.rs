use bevy_ecs::prelude::{Commands, Entity, IntoSystem, Query, Res, ResMut};
use glam::{Quat, Vec3};
use seija_asset::{Assets, Handle};
use seija_core::{window::AppWindow, CoreStage, StartupStage};
use seija_examples::{load_texture, IExamples};

use seija_render::{
    light::LightEnv,
    material::MaterialStorage,
    resource::{
        shape::{Cube, Sphere},
        Mesh, Texture,
    },
    wgpu,
};
use seija_transform::Transform;

pub struct ShadowTest;

impl IExamples for ShadowTest {
    fn run(app: &mut seija_app::App) {
        app.add_system2(CoreStage::Startup, StartupStage::Startup, on_start.system());
        app.add_system(CoreStage::Update, on_update.system());
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
    mut light: ResMut<LightEnv>,
    mut query: Query<(Entity, &Handle<Mesh>, &mut Transform)>,
) {
}
