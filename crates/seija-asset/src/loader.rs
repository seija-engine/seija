use std::sync::{Arc, atomic::{AtomicU8, Ordering}};
use seija_core::smol::channel::Sender;

use crate::{HandleUntyped,HandleId, RefEvent};
/*
pub enum LoadState {
    None,
    Loading,
    Loaded,
    Failed
}*/
#[repr(u8)]
pub enum TrackState {
    None    = 0,
    Loading = 1,
    Success = 2,
    Fail    = 3
}

impl Into<TrackState> for u8 {
    fn into(self) -> TrackState {
        match self {
            1 => TrackState::Loading,
            2 => TrackState::Success,
            3 => TrackState::Fail,
            _ => TrackState::None,
        }
    }
}

#[derive(Clone)]
pub struct LoadingTrack {
    inner:Arc<LoadingTrackInner>
}

struct LoadingTrackInner {
    sender:Sender<RefEvent>,
    handle:HandleId,
    progress:AtomicU8,
    state:AtomicU8
}

impl LoadingTrack {
    pub fn new(handle:HandleId,sender:Sender<RefEvent>) -> Self {
        LoadingTrack { inner: Arc::new(LoadingTrackInner {
            sender,
            handle,
            progress:AtomicU8::new(0u8),
            state:AtomicU8::new(0u8)
        })}
    }

    pub fn take(&self) -> HandleUntyped {
        let sender = self.inner.sender.clone();
        HandleUntyped::strong(self.handle_id().clone(), sender)
    }

    pub fn handle_id(&self) -> &HandleId {
        &self.inner.handle
    }

    pub fn set_state(&self,state:TrackState) {
        let state_u8:u8 = state as u8;
        self.inner.state.store(state_u8, Ordering::Relaxed);
    }

    pub fn state(&self) -> TrackState {
        self.inner.state.load(Ordering::Relaxed).into()
    }

    pub fn is_finish(&self) -> bool {
        self.inner.state.load(Ordering::Relaxed) == 2u8
    }
    
    pub fn add_progress(&self) {
        //log::debug!("track add progress {}",self.inner.progress.load(Ordering::Relaxed));
        self.inner.progress.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get_progress(&self) -> u8 {
        self.inner.progress.load(Ordering::Relaxed)
    }

}
