use crate::{attribute::MotionAttribute, GameSettings};
use nalgebra::Vector2;

pub mod bullet;
pub mod enemy;
pub mod hero;

pub trait CollisionBox {
    /// (center_x, center_y, width, height)
    fn bounding_box(&self) -> (f32, f32, f32, f32);
}

pub fn collisioned<A: CollisionBox, B: CollisionBox>(a: &A, b: &B) -> bool {
    if (a.bounding_box().0 - b.bounding_box().0).abs()
        <= (a.bounding_box().2 + b.bounding_box().2) / 2.0
        && (a.bounding_box().1 - b.bounding_box().1).abs()
            <= (a.bounding_box().3 + b.bounding_box().3) / 2.0
    {
        true
    } else {
        false
    }
}

pub trait Entity {
    fn motion_attribute(&self) -> MotionAttribute;
}

#[derive(Default, Clone, Copy)]
pub struct MotionState {
    pub pos: Vector2<f32>,
    pub speed: Vector2<f32>,
    pub acc: Vector2<f32>,
    // constant
    pub acc_val: f32,
    pub friction: f32,
}

impl MotionState {
    pub fn tick(&mut self, game_setting: &GameSettings) {
        if self.speed.x.abs() > 0.0 {
            self.speed.x = self.speed.x.signum() * (self.speed.x.abs() - self.friction).max(0.0);
        }
        self.speed.x += self.acc.x;
        self.pos.x += self.speed.x;
        if self.pos.x < 0.0 || self.pos.x > game_setting.width as f32 {
            self.pos.x = self.pos.x.clamp(0.0, game_setting.width as f32);
            self.speed.x = 0.0;
        }

        // TODO: better border handling

        if self.speed.y.abs() > 0.0 {
            self.speed.y = self.speed.y.signum() * (self.speed.y.abs() - self.friction).max(0.0);
        }
        self.speed.y += self.acc.y;
        self.pos.y += self.speed.y;
        if self.pos.y < 0.0 || self.pos.y > game_setting.height as f32 {
            self.pos.y = self.pos.y.clamp(0.0, game_setting.height as f32);
            self.speed.y = 0.0;
        }
    }
}
