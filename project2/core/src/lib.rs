use std::rc::Rc;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default, Clone, Copy)]
pub struct MotionState {
    pub x: f32,
    pub y: f32,
    pub speed_x: f32,
    pub speed_y: f32,
    pub acc_x: f32,
    pub acc_y: f32,
    // constant
    pub acc: f32,
    pub friction: f32,
}

#[wasm_bindgen]
pub struct UserInput {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
    pub space: bool,
}

#[wasm_bindgen]
pub struct UserInputEvent {
    key: String,
    pressed: bool,
}

#[wasm_bindgen]
impl UserInputEvent {
    #[wasm_bindgen(constructor)]
    pub fn new(key: String, pressed: bool) -> Self {
        Self { key, pressed }
    }
}

pub trait UserInputEventReciever {
    fn update(&mut self, user_input_event: &UserInputEvent);
}

impl UserInputEventReciever for MotionState {
    fn update(&mut self, user_input_event: &UserInputEvent) {
        match user_input_event.key.as_str() {
            "w" => {
                if user_input_event.pressed {
                    self.acc_y = self.acc;
                } else {
                    self.acc_y = 0.0;
                }
            }
            "a" => {
                if user_input_event.pressed {
                    self.acc_x = -self.acc;
                } else {
                    self.acc_x = 0.0;
                }
            }
            "s" => {
                if user_input_event.pressed {
                    self.acc_y = -self.acc;
                } else {
                    self.acc_y = 0.0;
                }
            }
            "d" => {
                if user_input_event.pressed {
                    self.acc_x = self.acc;
                } else {
                    self.acc_x = 0.0;
                }
            }
            _ => (),
        }
    }
}

impl MotionState {
    fn tick(&mut self, game_setting: Rc<GameSettings>) {
        if self.speed_x.abs() > 0.0 {
            self.speed_x = self.speed_x.signum() * (self.speed_x.abs() - self.friction).max(0.0);
        }
        self.speed_x += self.acc_x;
        self.x += self.speed_x;
        if self.x < 0.0 || self.x > game_setting.width as f32 {
            self.x = self.x.clamp(0.0, game_setting.width as f32);
            self.speed_x = 0.0;
        }
        // }

        // TODO: better border handling

        if self.speed_y.abs() > 0.0 {
            self.speed_y = self.speed_y.signum() * (self.speed_y.abs() - self.friction).max(0.0);
        }
        self.speed_y += self.acc_y;
        self.y += self.speed_y;
        if self.y < 0.0 || self.y > game_setting.height as f32 {
            self.y = self.y.clamp(0.0, game_setting.height as f32);
            self.speed_y = 0.0;
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Hero {
    pub health: u16,
    pub shooting: bool,
    pub motion_state: MotionState,
}

impl UserInputEventReciever for Hero {
    fn update(&mut self, user_input_event: &UserInputEvent) {
        self.motion_state.update(user_input_event);
        if user_input_event.key.as_str() == " " {
            if user_input_event.pressed {
                self.shooting = true;
            } else {
                self.shooting = false;
            }
        }
    }
}

impl Hero {
    pub fn tick(&mut self, game_setting: Rc<GameSettings>) {
        self.motion_state.tick(game_setting);
    }
}

pub trait Entity {}

#[wasm_bindgen]
pub struct Game {
    setting: Rc<GameSettings>,
    pub hero: Hero,
    // enemies: Box<dyn Entity>
}

#[wasm_bindgen]
pub struct GameSettings {
    pub width: u16,
    pub height: u16,
}

#[wasm_bindgen]
impl GameSettings {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(setting: GameSettings) -> Self {
        Self {
            setting: Rc::new(setting),
            hero: Hero {
                health: 100,
                motion_state: MotionState {
                    acc: 4.0,
                    friction: 1.6,
                    ..Default::default()
                },
                shooting: false,
            },
        }
    }

    pub fn update(&mut self, user_input_event: &UserInputEvent) {
        self.hero.update(user_input_event);
    }

    pub fn tick(&mut self) {
        self.hero.tick(self.setting.clone());
    }
}
