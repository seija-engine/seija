use uuid::Uuid;
pub use core_derive::TypeUuid;
pub trait TypeUuid {
    const TYPE_UUID: Uuid;
}

pub trait TypeUuidDynamic {
    fn type_uuid(&self) -> Uuid;
    fn type_name(&self) -> &'static str;
}

impl<T> TypeUuidDynamic for T where T: TypeUuid {
    fn type_uuid(&self) -> Uuid {
        Self::TYPE_UUID
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}