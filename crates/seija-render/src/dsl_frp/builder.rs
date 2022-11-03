use crate::dsl_frp::{elems::UniformElem, frp_comp::CompElement};

use super::frp_comp::FRPComponent;
use anyhow::{Result,anyhow};

#[derive(Debug)]
pub enum BuilderCommand {
    StartComp(String),
    EndComp,
    Uniform(String)
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

    pub fn build(mut self) -> Result<FRPComponent> {
        for command in self.command_list.drain(..) {
           log::info!("Exec FRPCompBuilder:{:?}",&command);
            match command {
                BuilderCommand::StartComp(name) => {
                    self.comp_stack.push(FRPComponent::new(name));
                },
                BuilderCommand::EndComp => {
                   let pop_comp = self.comp_stack.pop().ok_or(anyhow!("comp stack is nil"))?;
                   if let Some(parent_comp) = self.comp_stack.last_mut() {
                        parent_comp.add_element(CompElement::Component(pop_comp));
                   } else {
                      return Ok(pop_comp);
                   }
                },
                BuilderCommand::Uniform(name) => {
                    let cur_comp = self.comp_stack.last_mut().ok_or(anyhow!("stack comp is nil"))?;
                    cur_comp.add_element(CompElement::Unifrom(UniformElem::new(name)));
                },
            }
        }
        Err(anyhow!("error eof"))
    }
}
