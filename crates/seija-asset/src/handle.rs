use std::{hash::{Hash, Hasher}, marker::PhantomData};
use bevy_ecs::prelude::Component;
use seija_core::smol::channel::Sender;
use uuid::Uuid;

use crate::{asset::Asset, RefEvent};

#[derive(Debug,Clone, Copy,PartialEq, Eq,Hash)]
pub struct  HandleId {
    pub(crate) typ:Uuid,
    pub(crate) id:u64
}

impl HandleId {
    #[inline]
    pub fn random<T: Asset>() -> Self {
        HandleId {typ:T::TYPE_UUID,id:rand::random()} 
    }

    pub fn typ(&self) -> &Uuid {
        &self.typ
    }

    #[inline]
    pub const fn new(type_uuid: Uuid, id: u64) -> Self {  
        HandleId {typ:type_uuid,id} 
    }
}

#[derive(Debug,Component)]
pub struct Handle<T> where T:Asset {
    pub id:HandleId,
    sender:Option<Sender<RefEvent>>,
    marker: PhantomData<T>
}

impl<T: Asset> Hash for Handle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.id, state);
    }
}

impl<T: Asset> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T: Asset> Eq for Handle<T> {}


impl<T: Asset> Handle<T> {
    pub fn weak(id: HandleId) -> Handle<T> {
        Handle { id, marker:PhantomData,sender:None }
    }

    pub fn strong(id:HandleId,ref_sender:Sender<RefEvent>) -> Handle<T> {
        ref_sender.try_send(RefEvent::Increment(id)).unwrap();
        Handle {
            id,
            sender:Some(ref_sender),
            marker:PhantomData
        }
    }

    pub fn clone_weak(&self) -> Handle<T> {
        Handle::weak(self.id)
    }

    pub fn clone_weak_untyped(&self) -> HandleUntyped {
        HandleUntyped::weak(self.id)
    }

    pub fn untyped(mut self) -> HandleUntyped {
        let sender = self.sender.clone();
        self.sender = None;
        if let Some(sender) = sender {
            HandleUntyped::strong(self.id, sender)
        } else {
            HandleUntyped::weak(self.id)
        }
    }
    pub fn forget(&mut self) {
        self.sender = None;
    }
}

impl<T: Asset> Drop for Handle<T> {
    fn drop(&mut self) {
        if let Some(ref_sender) = &self.sender {
            let _ = ref_sender.try_send(RefEvent::Decrement(self.id));
        }
    }
}

impl<T:Asset> Clone for Handle<T> {
    fn clone(&self) -> Self {
        self.sender.as_ref().map(|sender| {
            sender.try_send(RefEvent::Increment(self.id)).unwrap();
        });

        Self { id: self.id.clone(), sender: self.sender.clone(), marker: PhantomData }
    }
}


#[derive(Debug)]
pub struct HandleUntyped {
    pub id: HandleId,
    sender:Option<Sender<RefEvent>>,
}

impl HandleUntyped {
    
    pub fn typed<T:Asset>(mut self) -> Handle<T> {
        let sender = self.sender.clone();
        self.sender = None;
        Handle { id:self.id,sender,marker:PhantomData }
    }

    pub fn weak(id: HandleId) -> Self {
        Self {
            id,
            sender:None
        }
    }

    pub fn strong(id:HandleId,sender:Sender<RefEvent>) -> Self {
        sender.try_send(RefEvent::Increment(id)).unwrap();
        HandleUntyped { id, sender:Some(sender) }
    }

    pub fn forget(&mut self) {
        self.sender = None;
    }
}

impl Hash for HandleUntyped {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.id, state);
    }
}

impl PartialEq for HandleUntyped {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for HandleUntyped {}

impl Drop for HandleUntyped {
    fn drop(&mut self) {
        if let Some(ref_sender) = &self.sender {
           let _ = ref_sender.try_send(RefEvent::Decrement(self.id));
        }
    }
}

impl Clone for HandleUntyped {
    fn clone(&self) -> Self {
        self.sender.as_ref().map(|sender| {
            sender.try_send(RefEvent::Increment(self.id)).unwrap();
        });

        Self { id: self.id.clone(), sender: self.sender.clone() }
    }
}