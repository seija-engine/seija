use std::sync::{RwLock};
use bevy_ecs::{prelude::{World,Entity}};
use super::view_list::ViewList;

pub struct Query {
    typ:u32,
    key:u32,
    list:ViewList
}


#[derive(Default)]
pub struct QuerySystem {
    //这里可以考虑换成UnsafeCell
    pub querys:Vec<RwLock<Query>>
}

