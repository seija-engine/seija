use bevy_ecs::prelude::World;
use lite_clojure_eval::EvalRT;
use seija_asset::Assets;
use seija_render::{graph::INode, RenderContext, resource::{RenderResourceId, shape::{Quad, Cube, Sphere}, Mesh}, material::{read_material_def, MaterialStorage}};
use seija_transform::Transform;

static LIGHT_PASS_MAT_STRING:&str = include_str!("light_pass.mat.clj");

pub struct DeferredLightPass {
    tex_count:usize
}

impl DeferredLightPass {
    pub fn new(tex_count:usize) -> Self {
        DeferredLightPass {
            tex_count
        }
    }
}

impl INode for DeferredLightPass {
    fn input_count(&self) -> usize { self.tex_count }
    fn init(&mut self, world: &mut World, ctx:&mut RenderContext) {
        // {
        //     let mut vm = EvalRT::new();
        //     let mat_def = read_material_def(&mut vm, LIGHT_PASS_MAT_STRING).unwrap();
        //     let mat_storages = world.get_resource::<MaterialStorage>().unwrap();
        //     mat_storages.add_def(mat_def);
        //     let h_mat = mat_storages.create_material("DeferredLightPass").unwrap();

        //     let quad_mesh:Mesh = Sphere::new(1f32).into();
        //     let mut meshs = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        //     let h_quad = meshs.add(quad_mesh);

        //     let mut t = Transform::default();
        //     t.local.position.z = -0.1f32;
           
        //     world.spawn().insert(h_quad).insert(t).insert(h_mat);
        // };
    }

    fn prepare(&mut self, _world: &mut World, ctx:&mut RenderContext) {
        
    }

    fn update(&mut self,world: &mut World,ctx:&mut RenderContext,
              inputs:&Vec<Option<RenderResourceId>>,
              outputs:&mut Vec<Option<RenderResourceId>>) {
       
    }
}