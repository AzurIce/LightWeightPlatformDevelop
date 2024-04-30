pub mod attribute;
pub mod entity;
pub mod input;
pub mod render;

use entity::{hero::Hero, MotionState};
use input::{UserInputEvent, UserInputEventReciever};
use nalgebra::Vector2;
use render::{BitmapAsset, Primitive};

use instant::Instant;
use std::rc::Rc;
// use std::time::Instant;

use wasm_bindgen::prelude::*;

use crate::render::Render;

#[derive(Clone, Copy)]
pub struct Bullet {
    pub motion_state: MotionState,
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
                    acc_val: 4.0,
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
                        pos: self.hero.motion_state.pos,
                        speed: Vector2::y() * 8.0,
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
            bullet.motion_state.pos.y > 0.0 && bullet.motion_state.pos.y < settings.height as f32
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
        self.render_primitives
            .push(self.states.hero.render(ms_delta));
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

    // pub fn hero(&self) -> Hero {
    //     self.states.hero.clone()
    // }
    // pub fn bullets(&self) -> Box<[Bullet]> {
    //     self.states.hero_bullets.clone().into_boxed_slice()
    // }

    pub fn primitives(&self) -> *const Primitive {
        self.render_primitives.as_ptr()
    }
    pub fn primitives_len(&self) -> usize {
        self.render_primitives.len()
    }

    pub fn debug_info(&self) -> String {
        format!(
            "position: ({}, {}),<br/>speed: ({}, {})<br/>shooting: {}, {}",
            self.states.hero.motion_state.pos.x,
            self.states.hero.motion_state.pos.y,
            self.states.hero.motion_state.speed.x,
            self.states.hero.motion_state.speed.y,
            self.states.hero.shooting,
            self.states.hero.shooting_cooldown
        )
    }
    // pub fn primitive_size(&self) -> usize {
    //     std::mem::size_of::<Primitive>()
    // }
}
