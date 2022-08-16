use std::collections::HashMap;
use smol_str::SmolStr;
#[derive(Default,Debug)]
pub struct TEntity {
    pub name:Option<SmolStr>,
    pub layer:u32,
    pub tag:Option<SmolStr>,
    pub components:Vec<TComponent>,
    pub children:Vec<TEntity>
}
#[derive(Default,Debug)]
pub struct TComponent {
    pub typ:SmolStr,
    pub attrs:HashMap<SmolStr,SmolStr> 
}

impl TComponent {
    pub fn new(typ:SmolStr) -> Self {
        TComponent { typ, attrs:HashMap::default() }
    }
}