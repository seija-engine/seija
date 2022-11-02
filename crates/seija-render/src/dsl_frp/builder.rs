use super::frp_comp::FRPComponent;
use anyhow::{Result,anyhow};

#[derive(Debug)]
pub enum BuilderCommand {
    StartComp(String),
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

    pub fn build(mut self) -> Result<()> {
        for command in self.command_list.drain(..) {
            dbg!(&command);
            match command {
                BuilderCommand::StartComp(name) => {
                    self.comp_stack.push(FRPComponent::new(name));
                },
                BuilderCommand::Uniform(name) => {
                    let cur_comp = self.comp_stack.last().ok_or(anyhow!("stack comp is nil"))?;
                },
                _ => {}
            }
        }
        Ok(())
    }
}
