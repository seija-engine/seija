#[derive(Debug, Clone,Copy)]
pub struct WindowResized {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone,Copy)]
pub struct WindowCreated;