pub mod hero;
pub mod input;
pub mod render;

use hero::Hero;
use input::{UserInput, UserInputEvent, UserInputEventReciever};
use render::{BitmapAsset, Primitive};

use std::{borrow::Borrow, cell::RefCell, rc::Rc};
use instant::Instant;
// use std::time::Instant;

use wasm_bindgen::prelude::*;

use crate::render::Render;

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

impl UserInputEventReciever for MotionState {
    fn update(&mut self, user_input_event: &UserInputEvent) {
        match user_input_event.key().as_str() {
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
    fn tick(&mut self, game_setting: &GameSettings) {
        if self.speed_x.abs() > 0.0 {
            self.speed_x = self.speed_x.signum() * (self.speed_x.abs() - self.friction).max(0.0);
        }
        self.speed_x += self.acc_x;
        self.x += self.speed_x;
        if self.x < 0.0 || self.x > game_setting.width as f32 {
            self.x = self.x.clamp(0.0, game_setting.width as f32);
            self.speed_x = 0.0;
        }

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
pub struct Bullet {
    pub motion_state: MotionState,
}

impl Bullet {
    fn new(x: f32, y: f32, speed_x: f32, speed_y: f32) -> Self {
        Self {
            motion_state: MotionState {
                x,
                y,
                speed_x,
                speed_y,
                ..Default::default()
            },
        }
    }
}

impl Render for Bullet {
    fn render(&self, ms_delta: u128) -> Primitive {
        Primitive::new(
            BitmapAsset::BulletPlayer,
            (self.motion_state.x, self.motion_state.y),
        )
    }
}

// settings states and game

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
pub struct GameStates {
    pub hero: Hero,
    hero_bullets: Vec<Bullet>,
}

impl GameStates {
    pub fn new() -> Self {
        Self {
            hero: Hero {
                health: 100,
                motion_state: MotionState {
                    acc: 4.0,
                    friction: 1.6,
                    ..Default::default()
                },
                shooting: false,
                shooting_cooldown: 0,
            },
            hero_bullets: vec![],
        }
    }

    pub fn update(&mut self, user_input_event: &UserInputEvent) {
        self.hero.update(user_input_event);
    }

    pub fn tick(&mut self, settings: &GameSettings) {
        self.hero.motion_state.tick(settings);
        if self.hero.shooting {
            if self.hero.shooting_cooldown <= 0 {
                self.hero_bullets.push(Bullet {
                    motion_state: MotionState {
                        x: self.hero.motion_state.x,
                        y: self.hero.motion_state.y,
                        // speed_x: self.hero.motion_state.speed_x,
                        // speed_y: self.hero.motion_state.speed_y + 5.0,
                        speed_x: 0.0,
                        speed_y: 8.0,
                        acc: 0.0,
                        friction: 0.0,
                        ..Default::default()
                    },
                });
                self.hero.shooting_cooldown = 10;
            } else {
                self.hero.shooting_cooldown -= 1;
            }
        } else {
            if self.hero.shooting_cooldown > 0 {
                self.hero.shooting_cooldown -= 1;
            }
        }
        for bullet in self.hero_bullets.iter_mut() {
            bullet.motion_state.tick(settings);
            // TODO: collision detection
        }
        self.hero_bullets.retain(|bullet| {
            bullet.motion_state.y > 0.0 && bullet.motion_state.y < settings.height as f32
        });
    }
}

#[wasm_bindgen]
pub struct Game {
    settings: Rc<GameSettings>,
    last_tick_time: Instant,
    states: GameStates,
    render_primitives: Vec<Primitive>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(setting: GameSettings) -> Self {
        Self {
            settings: Rc::new(setting),
            last_tick_time: Instant::now(),
            states: GameStates::new(),
            render_primitives: Vec::new(),
        }
    }

    pub fn update(&mut self, user_input_event: &UserInputEvent) {
        self.states.update(user_input_event);
    }

    pub fn prepare_primitives(&mut self) {
        let ms_delta = self.last_tick_time.elapsed().as_millis();

        self.render_primitives.clear();
        self.render_primitives.push(self.states.hero.render(ms_delta));
        self.render_primitives.extend(
            self.states
                .hero_bullets
                .iter()
                .map(|bullet| bullet.render(ms_delta)),
        );
    }

    pub fn tick(&mut self) {
        self.states.tick(&self.settings);
        self.last_tick_time = Instant::now();
    }

    pub fn hero(&self) -> Hero {
        self.states.hero.clone()
    }
    pub fn bullets(&self) -> Box<[Bullet]> {
        self.states.hero_bullets.clone().into_boxed_slice()
    }

    pub fn primitives(&self) -> *const Primitive {
        self.render_primitives.as_ptr()
    }
    pub fn primitives_len(&self) -> usize {
        self.render_primitives.len()
    }
    // pub fn primitive_size(&self) -> usize {
    //     std::mem::size_of::<Primitive>()
    // }
}
