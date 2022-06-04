use crate::{calculate_distance, calculate_velocity, Now};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct VerticalState {
    play: bool,
    #[serde(skip)]
    start: Now,
    direction: f64,

    pub accel: f64,
    pub velocity: f64,
}

impl VerticalState {
    pub fn fall(&mut self) {
        self.accel = 800.0;
        self.velocity = 0.0;
        self.play = true;
        self.direction = -1.0;
        self.start.reset();
    }

    pub fn is_drop(&self) -> bool {
        self.direction.is_sign_negative()
    }

    pub fn stop(&mut self) {
        self.play = false;
    }

    pub fn mv(&mut self, pos: &mut f32, _: f32) {
        if self.play {
            let time = self.start.elapsed().as_secs_f64();
            self.velocity = calculate_velocity(
                self.velocity,
                self.accel * self.direction as f64,
                time,
            );
            let mut distance = calculate_distance(
                self.velocity,
                self.accel * self.direction,
                time,
            );
            let new_distance = distance;

            while distance != 0.0 {
                let move_by = distance.min(5.0);
                distance -= move_by;

                *pos -= (move_by as f32)
                    * (self.direction as f32);
                if *pos <= 0.0 {
                    self.direction *= -1.0;
                    self.velocity *= 0.8;
                    break;
                }
            }

            self.start.reset();

            if new_distance.abs() <= 0.5 && pos.abs() <= 0.5
            {
                self.play = false;
            }
        }
    }

    pub fn is_play(&self) -> bool {
        self.play
    }
}
