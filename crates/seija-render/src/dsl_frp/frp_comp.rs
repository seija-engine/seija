pub enum CompElement {
    Unifrom,
    Component(FRPComponent)
}

pub struct FRPComponent {
    name:String,
    elems:Vec<CompElement>
}

impl FRPComponent {
    pub fn new(name:String) -> Self {
        FRPComponent { name,elems:vec![] }
    }
}