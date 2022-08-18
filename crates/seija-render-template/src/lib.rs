use seija_asset::AssetServer;
use seija_core::{bevy_ecs::{system::EntityCommands, world::World}, anyhow::{Result,anyhow, bail}};
use seija_render::{camera::camera::{Camera, Perspective, Projection}, resource::Mesh};
use seija_template::{IFromTComponent, TComponent,TComponentCreator};


pub fn add_render_templates(world:&mut World) -> Result<()> {
    let mut creator = world.get_resource_mut::<TComponentCreator>()
                                               .ok_or(anyhow!("TComponentCreator"))?;
    creator.add("Camera", CameraCreator);
    creator.add("Mesh", MeshCreator);
    creator.add("Material", MaterialCreator);
    Ok(())
}

struct CameraCreator;

impl IFromTComponent for CameraCreator {
    fn from<'w,'s,'a>(&self,component:&TComponent,_:&AssetServer,commands:&mut EntityCommands<'w,'s,'a>) -> Result<()> {
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
        commands.insert(camera);
        Ok(())
    }
}


struct MeshCreator;

impl IFromTComponent for MeshCreator {
    fn from<'w,'s,'a>(&self,component:&TComponent,assets:&AssetServer,commands:&mut EntityCommands<'w,'s,'a>) -> Result<()> {
        if let Some(res_path) = component.attrs.get("res") {
           let real_path = res_path.strip_prefix("res://").ok_or(anyhow!("mesh res path err"))?;
           let handle = assets.get_asset_handle(real_path).ok_or(anyhow!("not found mesh res:{}",real_path))?;
           let h_mesh = handle.typed::<Mesh>();
           commands.insert(h_mesh);
           return Ok(());
        }
        Err(anyhow!("Mesh need res"))
    }
}


struct MaterialCreator;

impl IFromTComponent for MaterialCreator {
    fn from<'w,'s,'a>(&self,component:&TComponent,assets:&AssetServer,commands:&mut EntityCommands<'w,'s,'a>) -> Result<()> {
        
        Ok(())
    }
}