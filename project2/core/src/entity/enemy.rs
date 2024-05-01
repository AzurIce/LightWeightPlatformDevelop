use nalgebra::Vector2;
use wasm_bindgen::prelude::*;

use crate::{
    animation::AnimatedBitmap,
    attribute::MotionAttribute,
    render::{BitmapAsset, Primitive, Render},
    GameSettings,
};

use super::{CollisionBox, Entity, MotionState};

pub enum Enemy {
    SmallCup(Enemy1),
}

impl Render for Enemy {
    fn render(&self, ms_delta: u128) -> Primitive {
        match self {
            Enemy::SmallCup(enemy1) => enemy1.render(ms_delta),
        }
    }
}

#[wasm_bindgen]
pub struct Enemy1 {
    pub health: u16,
    #[wasm_bindgen(skip)]
    pub motion_state: MotionState,
    pub shooting_cooldown: u16,
    animation: AnimatedBitmap,
}

impl Enemy1 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            health: 1,
            motion_state: MotionState {
                pos: Vector2::new(x, y),
                speed: Vector2::new(0.0, -1.0),
                acc: Vector2::zeros(),
                acc_val: 0.0,
                friction: 0.0,
            },
            shooting_cooldown: 0,
            animation: AnimatedBitmap::new(vec![BitmapAsset::Enemy1], 3),
        }
    }

    /// The tick method of an entity only handles it self's inner state
    pub fn tick(&mut self, settings: &GameSettings) {
        self.motion_state.tick(settings);
        self.animation.tick();
    }
}

impl CollisionBox for Enemy1 {
    fn bounding_box(&self) -> (f32, f32, f32, f32) {
        (self.motion_state.pos.x, self.motion_state.pos.y, 57.0, 43.0)
    }
}

impl Entity for Enemy1 {
    fn motion_attribute(&self) -> MotionAttribute {
        MotionAttribute::UniformSpeed { speed: 2.0 }
    }
}

impl Render for Enemy1 {
    fn render(&self, ms_delta: u128) -> Primitive {
        let predicted_pos =
            self.motion_state.pos + (self.motion_state.speed / 50.0) * ms_delta as f32;

        let bitmap = self.animation.cur_bitmap();

        Primitive::new(bitmap, (predicted_pos.x, predicted_pos.y), 0.0)
    }
}
