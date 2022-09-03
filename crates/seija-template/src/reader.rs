use seija_core::anyhow::{Result,bail};
use quick_xml::events::{BytesStart, Event};
use smol_str::SmolStr;
use crate::{TEntity, types::{TEntityChildren, TTemplateEntity}, errors::TemplateError, TComponent};

pub fn read_tmpl_entity(xml_string: &str) -> Result<TEntity> {
    let mut xml_reader = quick_xml::Reader::from_str(xml_string);
    xml_reader.trim_text(true);
    let mut entity_stack: Vec<TEntityChildren> = vec![];
    let mut in_components = false;
    let mut cur_component: Option<Vec<u8>> = None;
    let mut buf = Vec::new();
    loop {
        match xml_reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
                b"Entity" => {
                    let mut now_entity = TEntity::default();
                    for attr in e.attributes() {
                        if let Ok(item) = attr {
                            match item.key {
                                b"name" => {
                                    now_entity.name = Some(std::str::from_utf8(&item.value)?.into())
                                }
                                b"layer" => {
                                    now_entity.layer = u32::from_str_radix(std::str::from_utf8(&item.value)?,10)?;
                                }
                                b"tag" => {
                                    now_entity.tag = Some(std::str::from_utf8(&item.value)?.into())
                                }
                                _ => {}
                            }
                        }
                    }
                    entity_stack.push(TEntityChildren::TEntity(now_entity));
                },
                b"Template" => {
                    let template = read_template(&e)?;
                    entity_stack.push(TEntityChildren::Template(template));
                },
                b"Components" => {
                    in_components = true;
                }
                b"Children" => {}
                _ if in_components => {
                    cur_component = Some(e.name().to_vec());
                    let t = read_tmpl_component(&e)?;
                    if let Some(TEntityChildren::TEntity(e)) = entity_stack.last_mut() {
                        e.components.push(t);
                    }
                }
                _ => {},
            },
            Ok(Event::Empty(e))  => {
                if e.name() == b"Template" {
                    let template = read_template(&e)?;
                    if let Some(TEntityChildren::TEntity(e)) = entity_stack.last_mut() {
                        e.children.push(TEntityChildren::Template(template));
                    }
                } else {
                    if in_components {
                        let t = read_tmpl_component(&e)?;
                        if let Some(TEntityChildren::TEntity(e)) = entity_stack.last_mut() {
                            e.components.push(t);
                        }
                    } else {
                        if let Some(TEntityChildren::Template(template)) = entity_stack.last_mut() {
                            let t = read_tmpl_component(&e)?;
                            template.components.push(t);
                        }
                    }
                }
            }
            Ok(Event::Text(txt)) => {
                if cur_component.is_some() {
                    let inner_string = txt.unescape_and_decode(&xml_reader)?;
                    if let Some(TEntityChildren::TEntity(e)) = entity_stack.last_mut() {
                        if let Some(c) = e.components.last_mut() {
                            c.attrs.insert("innerText".into(), inner_string.into());
                        }
                    }
                }
            }
            Ok(Event::End(ref e)) => match e.name() {
                b"Components" => in_components = false,
                b"Entity" | b"Template" => {
                    if entity_stack.len() > 1 {
                        let pop = entity_stack.pop().unwrap();
                        if let Some(TEntityChildren::TEntity(e)) = entity_stack.last_mut() {
                            e.children.push(pop);
                        }
                    }
                }
                name => {
                    if Some(name) == cur_component.as_ref().map(|v| v.as_slice()) {
                        cur_component = None;
                    }
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => bail!(e),
            _ => (),
        }
    }
    if let Some(TEntityChildren::TEntity(e)) = entity_stack.pop() {
        return Ok(e);
    } else {
        bail!("top entity error")
    }
}

fn read_template<'a>(e: &BytesStart<'a>) -> Result<TTemplateEntity> {
    let mut template = TTemplateEntity::default();
    
    for attr in e.attributes() {
        if let Ok(item) = attr {
            match item.key {
                b"name" => { template.name = Some(std::str::from_utf8(&item.value)?.into()) },
                b"layer" => { 
                    template.layer = u32::from_str_radix(std::str::from_utf8(&item.value)?,10)?;
                },
                b"tag" => { template.tag = Some(std::str::from_utf8(&item.value)?.into()) },
                b"res" => { template.res = std::str::from_utf8(&item.value)?.into()},
                _ => {}
            }
        }
    }
    if template.res.is_empty() {
        return Err(TemplateError::TemplateMissRes.into());   
    }
    Ok(template)
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

