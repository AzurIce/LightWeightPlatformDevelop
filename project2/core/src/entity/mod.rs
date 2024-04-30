use crate::{attribute::MotionAttribute, input::{UserInputEvent, UserInputEventReciever}, GameSettings};
use nalgebra::Vector2;
use wasm_bindgen::prelude::*;

pub mod hero;

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

impl UserInputEventReciever for MotionState {
    fn update(&mut self, user_input_event: &UserInputEvent) {
        match user_input_event.key().as_str() {
            "w" => {
                if user_input_event.pressed {
                    self.acc.y = self.acc_val;
                } else {
                    self.acc.y = 0.0;
                }
            }
            "a" => {
                if user_input_event.pressed {
                    self.acc.x = -self.acc_val;
                } else {
                    self.acc.x = 0.0;
                }
            }
            "s" => {
                if user_input_event.pressed {
                    self.acc.y = -self.acc_val;
                } else {
                    self.acc.y = 0.0;
                }
            }
            "d" => {
                if user_input_event.pressed {
                    self.acc.x = self.acc_val;
                } else {
                    self.acc.x = 0.0;
                }
            }
            _ => (),
        }
    }
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