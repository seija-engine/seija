use super::RenderOrder;
use seija_core::TypeUuid;
use uuid::Uuid;

#[derive(Debug,TypeUuid)]
#[uuid = "9fb83fbe-b850-42e0-a58c-53da87bace04"]
pub struct Material {
    pub order:RenderOrder,
}