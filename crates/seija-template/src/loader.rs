use crate::component::TComponentManager;
use crate::errors::TemplateError;
use crate::{TComponent, TEntity, Template,};
use seija_app::ecs::world::World;
use seija_asset::downcast_rs::DowncastSync;
use seija_asset::{AssetServer, AssetLoaderParams, AssetDynamic, AsyncLoadMode};
use seija_asset::{IAssetLoader,async_trait::async_trait};
use seija_core::anyhow::{bail,Result,anyhow};
use seija_core::TypeUuid;
use seija_core::smol;
use quick_xml::events::{BytesStart, Event};
use smol_str::SmolStr;

#[derive(Default)]
pub(crate) struct TemplateLoader;

#[async_trait]
impl IAssetLoader for TemplateLoader {
    fn typ(&self) -> seija_core::uuid::Uuid { Template::TYPE_UUID }
    
    fn mode(&self) -> AsyncLoadMode { AsyncLoadMode::Perpare }

    fn sync_load(&self,_:&mut World,path:&str,server:&AssetServer,_:Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>> {
        let full_path = server.full_path(path)?;
        let xml_string = std::fs::read_to_string(full_path)?;
        let template = Template::from_str(&xml_string)?;
        Ok(Box::new(template))
    }

    fn perpare(&self,world:&mut World,_:Option<Box<dyn DowncastSync>>) -> Option<Box<dyn DowncastSync>> {
        let mgr = world.get_resource::<TComponentManager>().unwrap().clone();
        Some(Box::new(mgr)) 
    }

    async fn async_load(&self,server:AssetServer,path:SmolStr,
                        mut touch_data:Option<Box<dyn DowncastSync>>,
                        _:Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>> {
        if let Some(touch_data) = touch_data.take() {
            let mgr = touch_data.into_any().downcast::<TComponentManager>().map_err(|_| TemplateError::TypeCastError)?;
            
            let full_path = server.full_path(path.as_str())?;
            
            let xml_string = smol::fs::read_to_string(full_path).await?;
            let mut template = Template::from_str(&xml_string)?;
            for (asset_typ,asset_path) in mgr.search_assets(&template.entity)? {
             
               let req = server.load_async_untyped(&asset_typ, asset_path.as_str(), None)?;
              
               let handle = req.wait_handle().await.ok_or(TemplateError::LoadAssetError)?;
              
               template.assets.push(handle);
            }
            return Ok(Box::new(template));
        }
        
        Err(anyhow!("TComponentManager"))
    }
}

pub fn read_tmpl_entity(xml_string: &str) -> Result<TEntity> {
    let mut xml_reader = quick_xml::Reader::from_str(xml_string);
    xml_reader.trim_text(true);
    let mut entity_stack:Vec<TEntity> = vec![];
    let mut in_components = false;
    let mut cur_component:Option<Vec<u8>> = None;
   
    let mut buf = Vec::new();
    loop {
        match xml_reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"Entity" => {
                        let mut now_entity = TEntity::default();
                        for attr in e.attributes() {
                            if let Ok(item) = attr {
                                match item.key {
                                    b"name" => {
                                        now_entity.name = Some(std::str::from_utf8(&item.value)?.into())
                                    }
                                    b"layer" => {
                                        let value = unsafe {
                                            u32::from_str_radix(
                                                std::str::from_utf8_unchecked(&item.value),
                                                10,
                                            )?
                                        };
                                        now_entity.layer = value;
                                    }
                                    b"tag" => {
                                        now_entity.name =
                                            Some(std::str::from_utf8(&item.value)?.into())
                                    }
                                    _ => {}
                                }
                            }
                        }
                        entity_stack.push(now_entity);
                    },
                    b"Components" => { in_components = true; }
                    b"Children" => {  }
                    _ if in_components => {
                        cur_component = Some(e.name().to_vec());
                        let t = read_tmpl_component(&e)?;
                        entity_stack.last_mut().map(|v| v.components.push(t)); 
                    },
                    _ => (),
                }
            }
            Ok(Event::Empty(e)) if in_components => {
                let t = read_tmpl_component(&e)?;
                entity_stack.last_mut().map(|v| v.components.push(t)); 
            },
            Ok(Event::Text(txt)) => {
                if cur_component.is_some() {
                    let inner_string = txt.unescape_and_decode(&xml_reader)?;
                    entity_stack.last_mut().and_then(|v| v.components.last_mut()).map(|c| {
                        c.attrs.insert("innerText".into(), inner_string.into());
                    });
                }
            },
            Ok(Event::End(ref e)) => match e.name() {
                b"Components" => in_components = false,
                b"Entity" => {
                   if entity_stack.len() > 1 {
                      let pop = entity_stack.pop().unwrap();
                      entity_stack.last_mut().unwrap().children.push(pop);
                   }
                },
                name => if Some(name) == cur_component.as_ref().map(|v| v.as_slice()) {  
                   cur_component = None;
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => bail!(e),
            _ => (),
        }
    }
    entity_stack.pop().ok_or(anyhow!("zero"))
}

fn read_tmpl_component<'a>(e: &BytesStart<'a>) -> Result<TComponent> {
    let name: SmolStr = std::str::from_utf8(e.name())?.into();
    let mut component = TComponent::new(name);
    for attr in e.attributes() {
        if let Ok(item) = attr {
            component.attrs.insert(
                std::str::from_utf8(&item.key)?.into(),
                std::str::from_utf8(&item.value)?.into(),
            );
        }
    }
    Ok(component)
}
