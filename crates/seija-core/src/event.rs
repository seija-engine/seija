use std::marker::PhantomData;

use bevy_ecs::{system::SystemParam,component::Component, prelude::*};
use seija_app::App;

use crate::CoreStage;

#[derive(Debug,Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EventId<T> {
    pub id: usize,
    _marker: PhantomData<T>,
}

impl<T> Copy for EventId<T> {}
impl<T> Clone for EventId<T> {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Debug)]
struct EventInstance<T> {
    pub event_id: EventId<T>,
    pub event: T,
}

#[derive(Debug)]
enum State {
    A,
    B,
}


#[derive(Debug)]
pub struct Events<T> {
    events_a: Vec<EventInstance<T>>,
    events_b: Vec<EventInstance<T>>,
    a_start_event_count: usize,
    b_start_event_count: usize,
    event_count: usize,
    state: State,
}

impl<T> Default for Events<T> {
    fn default() -> Self {
        Events {
            a_start_event_count: 0,
            b_start_event_count: 0,
            event_count: 0,
            events_a: Vec::new(),
            events_b: Vec::new(),
            state: State::A,
        }
    }
}


#[derive(SystemParam)]
pub struct EventReader<'a, T: Component> {
    last_event_count: Local<'a, (usize, PhantomData<T>)>,
    events: Res<'a, Events<T>>,
}

impl<'a, T: Component> EventReader<'a, T> {   
    pub fn iter(&mut self) -> impl DoubleEndedIterator<Item = &T> {
        self.iter_with_id().map(|(event, _id)| event)
    }

    pub fn iter_with_id(&mut self) -> impl DoubleEndedIterator<Item = (&T, EventId<T>)> {
        internal_event_reader(&mut self.last_event_count.0, &self.events).map(|(event, id)| {
            (event, id)
        })
    }
}

/// Sends events of type `T`.
#[derive(SystemParam)]
pub struct EventWriter<'a, T: Component> {
    events: ResMut<'a, Events<T>>,
}

pub struct ManualEventReader<T> {
    last_event_count: usize,
    _marker: PhantomData<T>,
}

impl<T> Default for ManualEventReader<T> {
    fn default() -> Self {
        ManualEventReader {
            last_event_count: 0,
            _marker: Default::default(),
        }
    }
}

impl<T> ManualEventReader<T> {
    pub fn iter<'a>(&mut self, events: &'a Events<T>) -> impl DoubleEndedIterator<Item = &'a T> {
        internal_event_reader(&mut self.last_event_count, events).map(|(e, _)| e)
    }
}

impl<'a, T: Component> EventWriter<'a, T> {
    pub fn send(&mut self, event: T) {
        self.events.send(event);
    }

    pub fn send_batch(&mut self, events: impl Iterator<Item = T>) {
        self.events.extend(events);
    }
}

impl<T: Component> Events<T> {
    pub fn send(&mut self, event: T) {
        let event_id = EventId {
            id: self.event_count,
            _marker: PhantomData,
        };

        let event_instance = EventInstance { event_id, event };
        match self.state {
            State::A => self.events_a.push(event_instance),
            State::B => self.events_b.push(event_instance),
        }
        self.event_count += 1;
    }

    pub fn extend<I>(&mut self, events: I) where I: Iterator<Item = T> {
        for event in events {
            self.send(event);
        }
    }


    pub fn update_system(mut events: ResMut<Self>) {
        events.update();
    }


    pub fn update(&mut self) {
        match self.state {
            State::A => {
                self.events_b = Vec::new();
                self.state = State::B;
                self.b_start_event_count = self.event_count;
            }
            State::B => {
                self.events_a = Vec::new();
                self.state = State::A;
                self.a_start_event_count = self.event_count;
            }
        }
    }

    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        let map = |i: EventInstance<T>| i.event;
        match self.state {
            State::A => self.events_b.drain(..).map(map).chain(self.events_a.drain(..).map(map)),
            State::B => self.events_a.drain(..).map(map).chain(self.events_b.drain(..).map(map)),
        }
    }
}


fn map_instance_event_with_id<T>(event_instance: &EventInstance<T>) -> (&T, EventId<T>) {
    (&event_instance.event, event_instance.event_id)
}

fn internal_event_reader<'a, T>(last_event_count: &mut usize,events: &'a Events<T>) -> impl DoubleEndedIterator<Item = (&'a T, EventId<T>)> {
    let a_index = if *last_event_count > events.a_start_event_count {
        *last_event_count - events.a_start_event_count
    } else {
        0
    };
    let b_index = if *last_event_count > events.b_start_event_count {
        *last_event_count - events.b_start_event_count
    } else {
        0
    };
    *last_event_count = events.event_count;
    match events.state {
        State::A => events.events_b.get(b_index..).unwrap_or_else(|| &[]).iter()
            .map(map_instance_event_with_id)
            .chain(
                events.events_a.get(a_index..).unwrap_or_else(|| &[]).iter().map(map_instance_event_with_id),
            ),
        State::B => events.events_a.get(a_index..).unwrap_or_else(|| &[]).iter()
            .map(map_instance_event_with_id)
            .chain(
                events.events_b.get(b_index..).unwrap_or_else(|| &[]).iter().map(map_instance_event_with_id),
            ),
    }
}



