use crate::{
    attribute::MotionAttribute,
    render::{BitmapAsset, Primitive, Render},
    MotionState, UserInputEvent, UserInputEventReciever,
};
use wasm_bindgen::prelude::wasm_bindgen;

use super::Entity;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Hero {
    pub health: u16,
    #[wasm_bindgen(skip)]
    pub motion_state: MotionState,
    pub shooting: bool,
    pub shooting_cooldown: u16,
}

impl Entity for Hero {
    fn motion_attribute(&self) -> MotionAttribute {
        MotionAttribute::AcceleratedWithFriction {
            acceleration: 4.0,
            friction: 1.6,
        }
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
        Primitive::new(BitmapAsset::Hero1, (predicted_pos.x, predicted_pos.y), 0.0)
    }
}

// impl Hero {
//     pub fn tick(&mut self, game_setting: &GameSettings) {
//         self.motion_state.tick(game_setting);
//     }
// }
