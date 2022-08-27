use seija_asset::AssetServer;
use seija_core::{bevy_ecs::{system::{CommandQueue, Insert}, world::World, prelude::Entity}, anyhow::{Result,anyhow, bail}};
use seija_pbr::PBRCameraInfo;
use seija_render::{camera::camera::{Camera, Perspective, Projection}, resource::Mesh, material::Material};
use seija_template::{TComponent,TComponentCreator};


pub fn add_render_templates(world:&mut World) -> Result<()> {
    let mut creator = world.get_resource_mut::<TComponentCreator>()
                                               .ok_or(anyhow!("TComponentCreator"))?;
    creator.add("Camera", tcomponent_camera);
    creator.add("Mesh", t_component_mesh);
    creator.add("Material", t_component_material);
    creator.add("PBRCameraInfo", t_component_pbr_camera_info);
    Ok(())
}

fn tcomponent_camera(_:&mut World,entity:Entity,component:&TComponent,queue:&mut CommandQueue) -> Result<()> {
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
    queue.push(Insert {entity,component:camera});
    Ok(())
}


fn t_component_mesh<'w,'s,'a>(world:&mut World,entity:Entity,component:&TComponent,queue:&mut CommandQueue) -> Result<()> {
    let assets = world.get_resource::<AssetServer>().ok_or(anyhow!("not found asset server"))?;
    if let Some(res_path) = component.attrs.get("res") {
       let real_path = res_path.strip_prefix("res://").ok_or(anyhow!("mesh res path err"))?;
       let handle = assets.get_asset(real_path).ok_or(anyhow!("not found mesh res:{}",real_path))?;
       let h_mesh = handle.make_handle().typed::<Mesh>();
       queue.push(Insert {entity,component:h_mesh });
       return Ok(());
    }
    Err(anyhow!("Mesh need res"))
}

fn t_component_material<'w,'s,'a>(world:&mut World,entity:Entity,component:&TComponent,queue:&mut CommandQueue) -> Result<()> {
    let server = world.get_resource::<AssetServer>().ok_or(anyhow!("asset server"))?.clone();
    if let Some(res_path) = component.attrs.get("res") {
        let real_path = res_path.strip_prefix("res://").ok_or(anyhow!("mesh res path err"))?;
        let h_mat = server.load_sync::<Material>(world, real_path)?;
        queue.push(Insert {entity,component:h_mat });
    }
    Ok(())
}

fn t_component_pbr_camera_info<'w,'s,'a>(_:&mut World,entity:Entity,_:&TComponent,queue:&mut CommandQueue) -> Result<()> {
    let info = PBRCameraInfo::default();
    queue.push(Insert {entity,component:info });
    Ok(())
}