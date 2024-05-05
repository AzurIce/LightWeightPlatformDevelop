use crate::{
    animation::AnimatedBitmap,
    attribute::MotionAttribute,
    render::{BitmapAsset, Primitive, Render},
    GameSettings, MotionState, UserInputEvent, UserInputEventReciever,
};
use nalgebra::Vector2;
use wasm_bindgen::prelude::wasm_bindgen;

use super::{CollisionBox, Entity, EntityState};

#[wasm_bindgen]
pub struct Hero {
    pub health: u16,
    #[wasm_bindgen(skip)]
    pub motion_state: MotionState,
    pub shooting: bool,
    pub shooting_cooldown: u16,
    pub state: EntityState,
    animation: AnimatedBitmap,
    die_animation: AnimatedBitmap,
}

impl Hero {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            health: 100,
            motion_state: MotionState {
                pos: Vector2::new(x, y),
                speed: Vector2::zeros(),
                acc: Vector2::zeros(),
                acc_val: 4.0,
                friction: 1.6,
            },
            shooting: false,
            shooting_cooldown: 0,
            state: EntityState::Normal,
            animation: AnimatedBitmap::new(vec![BitmapAsset::Hero1, BitmapAsset::Hero2], 3),
            die_animation: AnimatedBitmap::new(
                vec![
                    BitmapAsset::HeroDown1,
                    BitmapAsset::HeroDown2,
                    BitmapAsset::HeroDown3,
                    BitmapAsset::HeroDown4,
                ],
                2,
            ),
        }
    }

    /// The tick method of an entity only handles it self's inner state
    pub fn tick(&mut self, settings: &GameSettings) {
        if self.state == EntityState::Normal {
            self.motion_state.tick(settings);
        }
        match self.state {
            EntityState::Normal => {
                self.animation.tick();
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

impl Entity for Hero {
    fn motion_attribute(&self) -> MotionAttribute {
        MotionAttribute::AcceleratedWithFriction {
            acceleration: 4.0,
            friction: 1.6,
        }
    }
}

impl CollisionBox for Hero {
    fn bounding_box(&self) -> (f32, f32, f32, f32) {
        (
            self.motion_state.pos.x,
            self.motion_state.pos.y,
            102.0,
            126.0,
        )
    }
}

impl UserInputEventReciever for Hero {
    fn update(&mut self, user_input_event: &UserInputEvent) {
        // self.motion_state.update(user_input_event);

        match user_input_event.key().as_str() {
            "w" => {
                if user_input_event.pressed {
                    self.motion_state.acc.y = self.motion_state.acc_val;
                } else {
                    self.motion_state.acc.y = 0.0;
                }
            }
            "a" => {
                if user_input_event.pressed {
                    self.motion_state.acc.x = -self.motion_state.acc_val;
                } else {
                    self.motion_state.acc.x = 0.0;
                }
            }
            "s" => {
                if user_input_event.pressed {
                    self.motion_state.acc.y = -self.motion_state.acc_val;
                } else {
                    self.motion_state.acc.y = 0.0;
                }
            }
            "d" => {
                if user_input_event.pressed {
                    self.motion_state.acc.x = self.motion_state.acc_val;
                } else {
                    self.motion_state.acc.x = 0.0;
                }
            }
            _ => (),
        }

        if user_input_event.key().as_str() == " " {
            if user_input_event.pressed {
                self.shooting = true;
            } else {
                self.shooting = false;
            }
        }
    }
}

impl Render for Hero {
    fn render(&self, ms_delta: u128) -> Primitive {
        let predicted_pos =
            self.motion_state.pos + (self.motion_state.speed / 50.0) * ms_delta as f32;

        let bitmap = match self.state {
            EntityState::Normal => self.animation.cur_bitmap(),
            EntityState::DieAnimating => self.die_animation.cur_bitmap(),
            _ => self.animation.cur_bitmap(),
        };

        Primitive::new(bitmap, (predicted_pos.x, predicted_pos.y), 0.0)
    }
}

// impl Hero {
//     pub fn tick(&mut self, game_setting: &GameSettings) {
//         self.motion_state.tick(game_setting);
//     }
// }
