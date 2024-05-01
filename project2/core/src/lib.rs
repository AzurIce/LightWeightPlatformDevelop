pub mod animation;
pub mod attribute;
pub mod entity;
pub mod input;
pub mod render;

use entity::{
    bullet::Bullet,
    collisioned,
    enemy::{Enemy, Enemy1},
    hero::Hero,
    MotionState,
};
use input::{UserInputEvent, UserInputEventReciever};
use nalgebra::Vector2;
use render::Primitive;

use instant::Instant;
use std::rc::Rc;
use web_sys::console;
// use std::time::Instant;

use wasm_bindgen::{convert::IntoWasmAbi, prelude::*};

use crate::render::Render;

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
    score: u32,
    hero: Hero,
    hero_bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    enemy_bullets: Vec<Bullet>,
}

impl GameStates {
    pub fn new() -> Self {
        Self {
            score: 0,
            hero: Hero::new(0.0, 0.0),
            hero_bullets: vec![],
            enemies: vec![],
            enemy_bullets: vec![],
        }
    }

    pub fn update(&mut self, user_input_event: &UserInputEvent) {
        self.hero.update(user_input_event);
    }

    pub fn tick(&mut self, settings: &GameSettings) {
        // internal state of hero
        self.hero.tick(settings);

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

        // hero_bullets
        for bullet in self.hero_bullets.iter_mut() {
            bullet.motion_state.tick(settings);
            for enemy in &mut self.enemies {
                match enemy {
                    Enemy::SmallCup(enemy) => {
                        if collisioned(bullet, enemy) {
                            console::log(&JsValue::from_str("collision").into());
                            enemy.health -= 1;
                        }
                    }
                }
            }
            // TODO: collision detection
        }

        // Remove enemy with emtpy health
        self.enemies.retain(|enemy| match enemy {
            Enemy::SmallCup(enemy) => {
                if enemy.health <= 0 {
                    self.score += 2
                };
                enemy.health > 0 && enemy.motion_state.pos.y > 0.0
            },
        });

        // spawn enemies
        let max_enemy_cnt = get_total_cnt_by_score(self.score);
        let gen_frac = get_gen_frac_by_score(self.score);
        let enemy_type = rand::random::<f32>();
        let gen_enemy = || {
            let x = rand::random::<f32>() * settings.width as f32;
            if enemy_type < gen_frac.0 {
                Enemy::SmallCup(Enemy1::new(x, settings.height as f32))
            } else {
                Enemy::SmallCup(Enemy1::new(x, settings.height as f32))
            }
        };

        while self.enemies.len() < max_enemy_cnt as usize {
            let enemy = gen_enemy();
            self.enemies.push(enemy);
        }

        for enemy in &mut self.enemies {
            match enemy {
                Enemy::SmallCup(enemy) => {
                    enemy.tick(settings)

                    // TODO: collision detection
                }
            }
        }

        self.hero_bullets.retain(|bullet| {
            bullet.motion_state.pos.y > 0.0 && bullet.motion_state.pos.y < settings.height as f32
        });
    }
}

/// Difficulty cauculation
fn get_gen_frac_by_score(score: u32) -> (f32, f32, f32) {
    let big = score / 1000 % 10;
    let middle = score / 100 % 10;
    let small = (score / 10 % 10).max(1);
    let total = (small + middle + big).max(1);

    (
        small as f32 / total as f32,
        middle as f32 / total as f32,
        big as f32 / total as f32,
    )
}

fn get_total_cnt_by_score(score: u32) -> u32 {
    (score / 10).max(1)
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
        self.render_primitives.extend(
            self.states
                .enemies
                .iter()
                .map(|enemy| enemy.render(ms_delta)),
        );
        self.render_primitives.extend(
            self.states
                .enemy_bullets
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
            "score: {},<br/>position: ({}, {}),<br/>speed: ({}, {})<br/>shooting: {}, {}",
            self.states.score,
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
