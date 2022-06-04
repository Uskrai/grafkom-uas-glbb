use std::{ops::RangeInclusive, time::Duration};

use crate::{calculate_distance, calculate_velocity, Now};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct HorizontalState {
    play: Option<i8>,
    #[serde(skip)]
    pub start: Now,
    #[serde(skip)]
    pub duration: Duration,

    pub velocity: f64,
    pub acceleration: f64,
}

impl HorizontalState {
    pub fn is_play(&self) -> bool {
        self.play.is_some()
    }
    pub fn stop(&mut self) {
        self.play = None;
    }

    pub fn play_left(&mut self) {
        self.play(-1);
    }

    pub fn play_right(&mut self) {
        self.play(1);
    }

    fn play(&mut self, direction: i8) {
        self.start.reset();
        self.play = Some(direction);
        let duration =
            self.velocity.abs() / self.acceleration.abs();
        self.duration = Duration::from_secs_f64(duration);
    }

    pub fn distance_at(&self, time: f64) -> f64 {
        calculate_distance(
            self.velocity,
            -self.acceleration,
            time,
        )
    }

    pub fn velocity_at(&self, time: f64) -> f64 {
        calculate_velocity(
            self.velocity,
            -self.acceleration,
            time,
        )
    }

    pub fn mv(
        &mut self,
        pos: &mut f32,
        range: RangeInclusive<f32>,
    ) {
        if let Some(mut direction) = self.play {
            let time = self
                .start
                .elapsed()
                .min(self.duration)
                .as_secs_f64();
            self.velocity = self.velocity_at(time);
            let mut distance = self.distance_at(time);

            while distance > 0.0 {
                let move_by = distance.min(5.0);
                distance -= move_by;
                *pos +=
                    (move_by as f32) * (direction as f32);

                if !range.contains(pos) {
                    direction *= -1;
                }
            }

            self.play = Some(direction);

            if self.start.elapsed() < self.duration {
                self.play(direction);
            } else {
                self.stop();
            }
        }
    }
}
