mod system;
mod types;
pub use types::*;
pub use system::ui_event_system;

#[derive(PartialEq,Eq,Hash,Clone,Debug)]
pub enum EventType {
    TouchStart = 0,
    TouchEnd = 1,
    Click = 2,
    MouseMove = 3,
    MouseEnter = 4,
    MouseLeave = 5,
    KeyBoard = 6,
    RecvChar = 7,
}
