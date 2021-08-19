use std::time::{Duration, Instant};

use bevy_ecs::prelude::ResMut;
pub struct Time {
    last_update: Instant,
    delta: Duration,
    delta_seconds: f32,
    startup: Instant,
}

impl Default for Time {
    fn default() -> Self {
        let now = Instant::now();
        Time {
            last_update: now,
            delta: Duration::from_secs(0),
            startup: now,
            delta_seconds: 0f32,
        }
    }
}

impl Time {
    #[inline]
    pub fn delta(&self) -> Duration {
        self.delta
    }

    #[inline]
    pub fn startup(&self) -> Instant {
        self.startup
    }

    #[inline]
    pub fn delta_seconds(&self) -> f32 {
        self.delta_seconds
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_update;
        self.delta_seconds = self.delta.as_secs_f32();

        self.last_update = now;
    }
}

pub fn time_system(mut time: ResMut<Time>) {
    time.update();
}
