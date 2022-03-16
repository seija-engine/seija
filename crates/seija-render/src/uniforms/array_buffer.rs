use crate::resource::BufferId;


struct ArrayItem {

}

pub struct ArrayBuffer {
    item_size:u32,
    cap:usize,
    len:usize,
    cache:Option<BufferId>,
    buffer:Option<BufferId>,
    items:Vec<ArrayItem>
}

impl ArrayBuffer {
    pub fn new(item_size:u32) -> Self {
        ArrayBuffer { 
            item_size : item_size,
            cap : 0, 
            len : 0, 
            cache : None, 
            buffer: None, items: vec![] 
        }
    }
}