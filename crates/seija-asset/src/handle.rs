use std::marker::PhantomData;

use uuid::Uuid;

use crate::asset::Asset;

#[derive(Debug,Clone, Copy,PartialEq, Eq,Hash)]
pub enum HandleId {
    Id(Uuid, u64)
}

impl HandleId {
    #[inline]
    pub fn random<T: Asset>() -> Self {
        HandleId::Id(T::TYPE_UUID, rand::random())
    }

    #[inline]
    pub const fn new(type_uuid: Uuid, id: u64) -> Self {
        HandleId::Id(type_uuid, id)
    }
}

#[derive(Debug)]
pub struct Handle<T> where T:Asset {
    pub id:HandleId,
    marker: PhantomData<T>
}

impl<T: Asset> Handle<T> {
    pub fn weak(id: HandleId) -> Handle<T> {
        Handle { id, marker:PhantomData }
    }

    pub fn clone_weak(&self) -> Handle<T> {
        Handle::weak(self.id)
    }

    pub fn clone_weak_untyped(&self) -> HandleUntyped {
        HandleUntyped::weak(self.id)
    }
}


#[derive(Debug)]
pub struct HandleUntyped {
    pub id: HandleId
}

impl HandleUntyped {
    pub fn weak(id: HandleId) -> Self {
        Self {
            id
        }
    }
}