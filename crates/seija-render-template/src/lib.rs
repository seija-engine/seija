use relative_path::RelativePath;
use seija_asset::AssetServer;
use seija_core::{bevy_ecs::{system::{CommandQueue, Insert}, prelude::Entity}, anyhow::{Result,anyhow, bail}, smol_str::SmolStr, uuid::Uuid, TypeUuid, math::Vec3};
use seija_gltf::asset::GltfAsset;
use seija_pbr::{PBRCameraInfo, lights::{PBRLight, PBRLightType}};
use seija_render::{camera::camera::{Camera, Perspective, Projection}, resource::Mesh, material::Material};
use seija_template::{TComponent,ITComponentOpt,AddTComponent};
use seija_app::App;

pub fn add_render_templates(app:&mut App) {
    app.add_tcomponent_opt("Camera", TComponentCameraOpt);
    app.add_tcomponent_opt("Mesh", TComponentMeshOpt);
    app.add_tcomponent_opt("PBRCameraInfo", TComponentPBRCameraInfoOpt);
    app.add_tcomponent_opt("Material", TComponentMaterialOpt);
    app.add_tcomponent_opt("PBRLight", TComponentLightOpt);
}

pub(crate) struct TComponentCameraOpt;

impl ITComponentOpt for TComponentCameraOpt {
    fn search_assets(&self, _: &mut TComponent,_:&RelativePath) -> Result<Vec<(Uuid,SmolStr)>> {
       Ok(vec![])
    }

    fn create_component(&self,_:&AssetServer, component: &TComponent,queue:&mut CommandQueue,entity:Entity)-> Result<()> {
        let mut camera = Camera::default();
        let camera_type = component.attrs.get("type").map(|v| v.as_str()).unwrap_or("Perspective");
        if let Some(cull_str) = component.attrs.get("cull").map(|v| v.as_str()) {
            camera.cull_type = cull_str.parse()?;
        }
        if let Some(is_hdr_str) = component.attrs.get("isHDR").map(|v| v.as_str()) {
            camera.is_hdr = is_hdr_str.parse()?;
        }
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
        let insert = Insert {entity,bundle:camera};
        queue.push(insert);
        Ok(())
    }
}

pub(crate) struct TComponentMeshOpt;

impl ITComponentOpt for TComponentMeshOpt {
    fn search_assets(&self, component: &mut TComponent,template_path:&RelativePath) -> Result<Vec<(Uuid,SmolStr)>> {
        if let Some(res_path) = component.attrs.get("res") {
            if res_path.contains(".gltf") {
                let split_names = res_path.split(".gltf").collect::<Vec<_>>();
                let path = split_names[0];
                let file_path = format!("{}.gltf",path);
                let asset_path = template_path.join_normalized(file_path.as_str());
                let mesh_path = format!("{}{}",asset_path.as_str(),split_names[1]) ;
                component.rt_attrs.insert("res".into(),mesh_path.into());
                return Ok(vec![(GltfAsset::TYPE_UUID,asset_path.as_str().into())]);
            } else {
                component.rt_attrs.insert("res".into(),res_path.clone());
            }
        }
        Ok(vec![])
    }

    fn create_component(&self,server:&AssetServer,component: &TComponent,queue:&mut CommandQueue,entity:Entity)-> Result<()> {
        if let Some(res_path) = component.rt_attrs.get("res") {
            
            let handle = server.get_asset(res_path).ok_or(anyhow!("not found mesh res:{}",res_path))?;
            let h_mesh = handle.make_handle().typed::<Mesh>();
            queue.push(Insert {entity,bundle:h_mesh });
            return Ok(());
        }
        Err(anyhow!("Mesh need res"))
    }
}

pub(crate) struct TComponentPBRCameraInfoOpt;

impl ITComponentOpt for TComponentPBRCameraInfoOpt {
    fn create_component(&self,_:&AssetServer, _: &TComponent,queue:&mut CommandQueue,entity:Entity)-> Result<()> {
        let info = PBRCameraInfo::default();
        queue.push(Insert {entity,bundle:info });
        Ok(())
    }
}

pub(crate) struct TComponentMaterialOpt;

impl ITComponentOpt for TComponentMaterialOpt {
    fn search_assets(&self, component: &mut TComponent,template_path:&RelativePath) -> Result<Vec<(Uuid,SmolStr)>> {
        if let Some(res_path) = component.attrs.get("res") {
            if res_path.starts_with('/') {
                let asset_path = res_path.trim_start_matches('/');
                component.rt_attrs.insert("res".into(), asset_path.into());
                return Ok(vec![(Material::TYPE_UUID,asset_path.into())]);
            } else {
                let asset_path = template_path.join_normalized(res_path.as_str());
                component.rt_attrs.insert("res".into(), asset_path.as_str().into());
                return Ok(vec![(Material::TYPE_UUID,asset_path.as_str().into())]);
            }
            
        }
        Ok(vec![])
    }

    fn create_component(&self,server:&AssetServer, component: &TComponent,queue:&mut CommandQueue,entity:Entity)-> Result<()> {
        if let Some(res_path) = component.rt_attrs.get("res") {
           
            let info = server.get_asset(res_path).ok_or(anyhow!("not found material res:{}",res_path))?;
            let h_material = info.make_handle().typed::<Material>();
            queue.push(Insert {entity,bundle:h_material });
        }
        Ok(())
    }
}


pub(crate) struct TComponentLightOpt;

impl ITComponentOpt for TComponentLightOpt {
    fn create_component(&self,_:&AssetServer, component: &TComponent,queue:&mut CommandQueue,entity:Entity)-> Result<()> {
        let light_type = component.attrs.get("type").map(|v| v.as_str()).unwrap_or("Directional");
        let typ = PBRLightType::try_from(light_type).map_err(|_| anyhow!("PBRLight Type"))?;
        let intensity:f32 = component.attrs.get("intensity").map(|v| v.as_str()).unwrap_or("100000").parse()?;
        let color = component.read_v3("color").unwrap_or(Vec3::ONE);
        
        let light = match typ {
            PBRLightType::Directional => { PBRLight::directional(color, intensity) },
            PBRLightType::Point => {
                let falloff = component.read_float("falloff", 10f32);
                PBRLight::point(color, intensity, falloff)
            },
            PBRLightType::Spot | PBRLightType::FocusedSpot => {
                let falloff = component.read_float("falloff", 10f32);
                let inner = component.read_float("inner", 45f32);
                let outer = component.read_float("outer", 50f32);
                PBRLight::spot(color, intensity, falloff, inner, outer, typ == PBRLightType::Spot)
            }
        };
        queue.push(Insert {entity,bundle:light });
        Ok(())
    }
}