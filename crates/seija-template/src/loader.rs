use crate::{TComponent, TEntity,};
use seija_core::anyhow::{bail,Result,anyhow};

use quick_xml::events::{BytesStart, Event};
use smol_str::SmolStr;

/*
pub struct TemplateLoader;

#[async_trait]
impl AssetLoader for TemplateLoader {
    async fn load(&self,server:AssetServer,_:Option<LoadingTrack>,path:&str,_:Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>>  {
        let full_path = server.full_path(path)?;
        let xml_string = smol::fs::read_to_string(full_path).await?;
        let template = Template::from_str(&xml_string)?;
        Ok(Box::new(template))
    }
}

*/
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
