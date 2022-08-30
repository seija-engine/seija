use seija_asset::AssetServer;
use seija_core::{bevy_ecs::{system::{CommandQueue, Insert}, prelude::Entity}, anyhow::{Result,anyhow, bail}, smol_str::SmolStr, uuid::Uuid, TypeUuid};
use seija_gltf::asset::GltfAsset;
use seija_pbr::PBRCameraInfo;
use seija_render::{camera::camera::{Camera, Perspective, Projection}, resource::Mesh, material::Material};
use seija_template::{TComponent,ITComponentOpt,AddTComponent};
use seija_app::App;

pub fn add_render_templates(app:&mut App) {
    app.add_tcomponent_opt("Camera", TComponentCameraOpt);
    app.add_tcomponent_opt("Mesh", TComponentMeshOpt);
    app.add_tcomponent_opt("PBRCameraInfo", TComponentPBRCameraInfoOpt);
    app.add_tcomponent_opt("Material", TComponentMaterialOpt);
}

pub(crate) struct TComponentCameraOpt;

impl ITComponentOpt for TComponentCameraOpt {
    fn search_assets(&self, _: &TComponent) -> Result<Vec<(Uuid,SmolStr)>> {
       Ok(vec![])
    }

    fn create_component(&self,_:&AssetServer, component: &TComponent,queue:&mut CommandQueue,entity:Entity)-> Result<()> {
        let mut camera = Camera::default();
        let camera_type = component.attrs.get("type").map(|v| v.as_str()).unwrap_or("Perspective");
        match camera_type {
            "Perspective" =>  {
                let mut per = Perspective::default();
                if let Some(fov) = component.attrs.get("fov") {
                    per.fov = fov.parse::<f32>()?.to_radians();
                }
                if let Some(near) = component.attrs.get("near") {
                    per.near = near.parse::<f32>()?;
                }
                if let Some(far) = component.attrs.get("far") {
                    per.far = far.parse::<f32>()?;
                }
                if let Some(aspect) = component.attrs.get("aspect") {
                    per.aspect_ratio = aspect.parse::<f32>()?;
                }
                camera.projection = Projection::Perspective(per);
            },
            "Orthographic" => {}
            _ => { bail!("error camera type:{}",camera_type) },
        }
        let insert = Insert {entity,component:camera};
        queue.push(insert);
        Ok(())
    }
}

pub(crate) struct TComponentMeshOpt;

impl ITComponentOpt for TComponentMeshOpt {
    fn search_assets(&self, component: &TComponent) -> Result<Vec<(Uuid,SmolStr)>> {
        if let Some(res_path) = component.attrs.get("res") {
            let real_path = res_path.strip_prefix("res://").ok_or(anyhow!("mesh res path err"))?;
            if real_path.contains(".json") {
                let path = real_path.split(".json").collect::<Vec<_>>()[0];
                let gltf_path = format!("{}.json",path); 
                return Ok(vec![(GltfAsset::TYPE_UUID,gltf_path.into())]);
            }
        }
        Ok(vec![])
    }

    fn create_component(&self,server:&AssetServer,component: &TComponent,queue:&mut CommandQueue,entity:Entity)-> Result<()> {
        if let Some(res_path) = component.attrs.get("res") {
            let real_path = res_path.strip_prefix("res://").ok_or(anyhow!("mesh res path err"))?;
            let handle = server.get_asset(real_path).ok_or(anyhow!("not found mesh res:{}",real_path))?;
            let h_mesh = handle.make_handle().typed::<Mesh>();
            queue.push(Insert {entity,component:h_mesh });
            return Ok(());
        }
        Err(anyhow!("Mesh need res"))
    }
}

pub(crate) struct TComponentPBRCameraInfoOpt;

impl ITComponentOpt for TComponentPBRCameraInfoOpt {
    fn create_component(&self,_:&AssetServer, _: &TComponent,queue:&mut CommandQueue,entity:Entity)-> Result<()> {
        let info = PBRCameraInfo::default();
        queue.push(Insert {entity,component:info });
        Ok(())
    }
}

pub(crate) struct TComponentMaterialOpt;

impl ITComponentOpt for TComponentMaterialOpt {
    fn search_assets(&self, component: &TComponent) -> Result<Vec<(Uuid,SmolStr)>> {
        if let Some(res_path) = component.attrs.get("res") {
            let real_path = res_path.strip_prefix("res://").ok_or(anyhow!("mesh res path err"))?;
            
            return Ok(vec![(Material::TYPE_UUID,real_path.into())]);
        }
        Ok(vec![])
    }

    fn create_component(&self,server:&AssetServer, component: &TComponent,queue:&mut CommandQueue,entity:Entity)-> Result<()> {
        if let Some(res_path) = component.attrs.get("res") {
            let real_path = res_path.strip_prefix("res://").ok_or(anyhow!("mesh res path err"))?;
            let info = server.get_asset(real_path).ok_or(anyhow!("not found material res:{}",real_path))?;
            let h_material = info.make_handle().typed::<Material>();
            queue.push(Insert {entity,component:h_material });
        }
        Ok(())
    }
}