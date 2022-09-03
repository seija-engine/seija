pub trait Contains<RHS> {
    
    fn contains(&self, _: &RHS) -> bool;
}