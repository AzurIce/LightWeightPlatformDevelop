use crate::{MotionState, UserInputEventReciever, UserInputEvent, GameSettings};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Hero {
    pub health: u16,
    pub motion_state: MotionState,
    pub shooting: bool,
    pub shooting_cooldown: u16,
}

impl UserInputEventReciever for Hero {
    fn update(&mut self, user_input_event: &UserInputEvent) {
        self.motion_state.update(user_input_event);
        if user_input_event.key().as_str() == " " {
            if user_input_event.pressed {
                self.shooting = true;
            } else {
                self.shooting = false;
            }
        }
    }
}

// impl Hero {
//     pub fn tick(&mut self, game_setting: &GameSettings) {
//         self.motion_state.tick(game_setting);
//     }
// }
