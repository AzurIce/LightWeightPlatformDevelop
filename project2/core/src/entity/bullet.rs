use nalgebra::Vector2;

use crate::render::{BitmapAsset, Primitive, Render};

use super::{CollisionBox, MotionState};


#[derive(Clone, Copy)]
pub struct Bullet {
    pub motion_state: MotionState,
}

impl CollisionBox for Bullet {
    fn bounding_box(&self) -> (f32, f32, f32, f32) {
        (self.motion_state.pos.x, self.motion_state.pos.y, 5.0, 11.0)
    }
}

impl Bullet {
    fn new(x: f32, y: f32, speed_x: f32, speed_y: f32) -> Self {
        Self {
            motion_state: MotionState {
                pos: Vector2::new(x, y),
                speed: Vector2::new(speed_x, speed_y),
                ..Default::default()
            },
        }
    }
}

impl Render for Bullet {
    fn render(&self, ms_delta: u128) -> Primitive {
        let predicted_pos =
            self.motion_state.pos + (self.motion_state.speed / 50.0) * ms_delta as f32;

        Primitive::new(
            BitmapAsset::BulletPlayer,
            (predicted_pos.x, predicted_pos.y),
            0.0,
        )
    }
}
