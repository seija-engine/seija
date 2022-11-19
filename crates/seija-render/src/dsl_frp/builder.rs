use crate::{dsl_frp::{elems::{UniformElement, TextureElement, if_comp::IfCompElement, posteffect_item::PostEffectItem}}, resource::TextureDescInfo};
use super::{frp_comp::FRPComponent, system::ElementCreator};
use anyhow::{Result,anyhow};
use seija_app::ecs::entity::Entity;
use lite_clojure_eval::{Variable, EvalRT};
use lite_clojure_frp::DynamicID;
use smol_str::SmolStr;

#[derive(Debug)]
pub enum BuilderCommand {
    StartComp(String),
    EndComp,
    Uniform(String),
    Node(i64,Vec<Variable>),
    Texture(TextureDescInfo,DynamicID),
    IfComp(DynamicID,Variable,Option<Variable>),
    PostEffectItem(Entity,SmolStr,u32)
}

pub struct FRPCompBuilder {
    command_list:Vec<BuilderCommand>,
    comp_stack:Vec<FRPComponent>
}

impl FRPCompBuilder {
    pub fn new() -> Self {
        FRPCompBuilder { command_list:vec![],comp_stack:vec![] }
    }

    pub fn push_command(&mut self,command:BuilderCommand) {
        self.command_list.push(command);
    }

    pub fn build(mut self,creator:&ElementCreator,_:&mut EvalRT) -> Result<FRPComponent> {
        for command in self.command_list.drain(..) {
           log::info!("Exec FRPCompBuilder:{:?}",&command);
            match command {
                BuilderCommand::StartComp(name) => {
                    self.comp_stack.push(FRPComponent::new(name));
                },
                BuilderCommand::EndComp => {
                   let pop_comp = self.comp_stack.pop().ok_or(anyhow!("comp stack is nil"))?;
                   if let Some(parent_comp) = self.comp_stack.last_mut() {
                        parent_comp.add_element(Box::new(pop_comp));
                   } else {
                      return Ok(pop_comp);
                   }
                },
                BuilderCommand::Uniform(name) => {
                    let cur_comp = self.comp_stack.last_mut().ok_or(anyhow!("stack comp is nil"))?;
                    cur_comp.add_element(Box::new(UniformElement::new(name)));
                },
                BuilderCommand::Node(index, args) => {
                    let update_node = creator.create_node(index as usize, args)?;
                    let node = Box::new(update_node);
                    let cur_comp = self.comp_stack.last_mut().ok_or(anyhow!("stack comp is nil"))?;
                    cur_comp.add_element(node);
                },
                BuilderCommand::Texture(desc_info,dyn_id) => {
                    let element = Box::new(TextureElement::new(desc_info,dyn_id));
                    let cur_comp = self.comp_stack.last_mut().ok_or(anyhow!("stack comp is nil"))?;
                    cur_comp.add_element(element);
                },
                BuilderCommand::IfComp(dynamic_id, true_comp_var,else_comp_var) => {
                    let if_comp = IfCompElement::new(dynamic_id, true_comp_var, else_comp_var)?;
                    let element = Box::new(if_comp);
                    let cur_comp = self.comp_stack.last_mut().ok_or(anyhow!("stack comp is nil"))?;
                 
                    cur_comp.add_element(element);
                },
                BuilderCommand::PostEffectItem(camera_entity, material_path, sort_order) => {
                    let item_comp = PostEffectItem::new(camera_entity, material_path, sort_order)?;
                    let element = Box::new(item_comp);
                    let cur_comp = self.comp_stack.last_mut().ok_or(anyhow!("stack comp is nil"))?;
                    cur_comp.add_element(element);
                }
            }
        }
        Err(anyhow!("error eof"))
    }
}
