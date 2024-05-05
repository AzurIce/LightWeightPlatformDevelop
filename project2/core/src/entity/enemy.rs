use nalgebra::Vector2;
use wasm_bindgen::prelude::*;

use crate::{
    animation::AnimatedBitmap,
    attribute::MotionAttribute,
    render::{BitmapAsset, Primitive, Render},
    GameSettings,
};

use super::{CollisionBox, Entity, EntityState, MotionState};

pub enum Enemy {
    MiddleCup(Enemy1),
    BigCup(Enemy2),
    SuperBigCup(Enemy3),
}

impl Render for Enemy {
    fn render(&self, ms_delta: u128) -> Primitive {
        match self {
            Enemy::MiddleCup(enemy) => enemy.render(ms_delta),
            Enemy::BigCup(enemy) => enemy.render(ms_delta),
            Enemy::SuperBigCup(enemy) => enemy.render(ms_delta),
        }
    }
}

#[wasm_bindgen]
pub struct Enemy1 {
    pub health: u16,
    #[wasm_bindgen(skip)]
    pub motion_state: MotionState,
    pub shooting_cooldown: u16,
    pub state: EntityState,
    normal_animation: AnimatedBitmap,
    die_animation: AnimatedBitmap,
}

impl Enemy1 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            health: 1,
            motion_state: MotionState {
                pos: Vector2::new(x, y),
                speed: Vector2::new(0.0, -3.0),
                acc: Vector2::zeros(),
                acc_val: 0.0,
                friction: 0.0,
            },
            shooting_cooldown: 0,
            state: EntityState::Normal,
            normal_animation: AnimatedBitmap::new(vec![BitmapAsset::Enemy1], 3),
            die_animation: AnimatedBitmap::new(
                vec![
                    BitmapAsset::Enemy1Down1,
                    BitmapAsset::Enemy1Down2,
                    BitmapAsset::Enemy1Down3,
                    BitmapAsset::Enemy1Down4,
                ],
                2,
            ),
        }
    }

    /// The tick method of an entity only handles it self's inner state
    pub fn tick(&mut self, settings: &GameSettings) {
        self.motion_state.tick(settings);
        match self.state {
            EntityState::Normal => {
                self.normal_animation.tick();
            }
            EntityState::DieAnimating => {
                if self.die_animation.tick() {
                    self.state = EntityState::Died;
                }
            }
            _ => (),
        }
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

        let bitmap = match self.state {
            EntityState::Normal => self.normal_animation.cur_bitmap(),
            EntityState::DieAnimating => self.die_animation.cur_bitmap(),
            _ => self.normal_animation.cur_bitmap(),
        };

        Primitive::new(bitmap, (predicted_pos.x, predicted_pos.y), 0.0)
    }
}

#[wasm_bindgen]
pub struct Enemy2 {
    pub health: u16,
    #[wasm_bindgen(skip)]
    pub motion_state: MotionState,
    pub shooting_cooldown: u16,
    pub state: EntityState,
    normal_animation: AnimatedBitmap,
    hitted_animation: AnimatedBitmap,
    die_animation: AnimatedBitmap,
}

impl Enemy2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            health: 5,
            motion_state: MotionState {
                pos: Vector2::new(x, y),
                speed: Vector2::new(0.0, -2.0),
                acc: Vector2::zeros(),
                acc_val: 0.0,
                friction: 0.0,
            },
            shooting_cooldown: 0,
            state: EntityState::Normal,
            normal_animation: AnimatedBitmap::new(vec![BitmapAsset::Enemy2], 3),
            hitted_animation: AnimatedBitmap::new(vec![BitmapAsset::Enemy2Hit], 3),
            die_animation: AnimatedBitmap::new(
                vec![
                    BitmapAsset::Enemy2Down1,
                    BitmapAsset::Enemy2Down2,
                    BitmapAsset::Enemy2Down3,
                    BitmapAsset::Enemy2Down4,
                ],
                2,
            ),
        }
    }

    /// The tick method of an entity only handles it self's inner state
    pub fn tick(&mut self, settings: &GameSettings) {
        self.motion_state.tick(settings);
        match self.state {
            EntityState::Normal => {
                self.normal_animation.tick();
            }
            EntityState::HittedAnimating => {
                if self.hitted_animation.tick() {
                    self.state = EntityState::Normal;
                }
            }
            EntityState::DieAnimating => {
                if self.die_animation.tick() {
                    self.state = EntityState::Died;
                }
            }
            _ => (),
        }
    }
}

impl CollisionBox for Enemy2 {
    fn bounding_box(&self) -> (f32, f32, f32, f32) {
        (self.motion_state.pos.x, self.motion_state.pos.y, 69.0, 99.0)
    }
}

impl Entity for Enemy2 {
    fn motion_attribute(&self) -> MotionAttribute {
        MotionAttribute::UniformSpeed { speed: 2.0 }
    }
}

impl Render for Enemy2 {
    fn render(&self, ms_delta: u128) -> Primitive {
        let predicted_pos =
            self.motion_state.pos + (self.motion_state.speed / 50.0) * ms_delta as f32;

        let bitmap = match self.state {
            EntityState::Normal => self.normal_animation.cur_bitmap(),
            EntityState::HittedAnimating => self.hitted_animation.cur_bitmap(),
            EntityState::DieAnimating => self.die_animation.cur_bitmap(),
            _ => self.normal_animation.cur_bitmap(),
        };

        Primitive::new(bitmap, (predicted_pos.x, predicted_pos.y), 0.0)
    }
}

#[wasm_bindgen]
pub struct Enemy3 {
    pub health: u16,
    #[wasm_bindgen(skip)]
    pub motion_state: MotionState,
    pub shooting_cooldown: u16,
    pub state: EntityState,
    normal_animation: AnimatedBitmap,
    hitted_animation: AnimatedBitmap,
    die_animation: AnimatedBitmap,
}

impl Enemy3 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            health: 10,
            motion_state: MotionState {
                pos: Vector2::new(x, y),
                speed: Vector2::new(0.0, -1.0),
                acc: Vector2::zeros(),
                acc_val: 0.0,
                friction: 0.0,
            },
            shooting_cooldown: 0,
            state: EntityState::Normal,
            normal_animation: AnimatedBitmap::new(
                vec![BitmapAsset::Enemy3N1, BitmapAsset::Enemy3N2],
                3,
            ),
            hitted_animation: AnimatedBitmap::new(vec![BitmapAsset::Enemy3Hit], 3),
            die_animation: AnimatedBitmap::new(
                vec![
                    BitmapAsset::Enemy3Down1,
                    BitmapAsset::Enemy3Down2,
                    BitmapAsset::Enemy3Down3,
                    BitmapAsset::Enemy3Down4,
                    BitmapAsset::Enemy3Down5,
                    BitmapAsset::Enemy3Down6,
                ],
                2,
            ),
        }
    }

    /// The tick method of an entity only handles it self's inner state
    pub fn tick(&mut self, settings: &GameSettings) {
        self.motion_state.tick(settings);
        match self.state {
            EntityState::Normal => {
                self.normal_animation.tick();
            }
            EntityState::HittedAnimating => {
                if self.hitted_animation.tick() {
                    self.state = EntityState::Normal;
                }
            }
            EntityState::DieAnimating => {
                if self.die_animation.tick() {
                    self.state = EntityState::Died;
                }
            }
            _ => (),
        }
    }
}

impl CollisionBox for Enemy3 {
    fn bounding_box(&self) -> (f32, f32, f32, f32) {
        (
            self.motion_state.pos.x,
            self.motion_state.pos.y,
            169.0,
            258.0,
        )
    }
}

impl Entity for Enemy3 {
    fn motion_attribute(&self) -> MotionAttribute {
        MotionAttribute::UniformSpeed { speed: 2.0 }
    }
}

impl Render for Enemy3 {
    fn render(&self, ms_delta: u128) -> Primitive {
        let predicted_pos =
            self.motion_state.pos + (self.motion_state.speed / 50.0) * ms_delta as f32;

        let bitmap = match self.state {
            EntityState::Normal => self.normal_animation.cur_bitmap(),
            EntityState::HittedAnimating => self.hitted_animation.cur_bitmap(),
            EntityState::DieAnimating => self.die_animation.cur_bitmap(),
            _ => self.normal_animation.cur_bitmap(),
        };

        Primitive::new(bitmap, (predicted_pos.x, predicted_pos.y), 0.0)
    }
}
