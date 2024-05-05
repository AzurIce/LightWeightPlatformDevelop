pub mod animation;
pub mod attribute;
pub mod entity;
pub mod input;
pub mod render;

use entity::{
    bullet::{self, Bullet},
    collisioned,
    enemy::{Enemy, Enemy1, Enemy2, Enemy3},
    hero::Hero,
    CollisionBox, EntityState, MotionState,
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
    hero_bullets: Vec<Box<Bullet>>,
    enemies: Vec<Box<Enemy>>,
    enemy_bullets: Vec<Box<Bullet>>,
    spawn_cooldown: u32,
}

const MAX_ENEMY_SPAWN_PER_TICK: u32 = 3;
const SPAWN_COOLDOWN: u32 = 10;

impl GameStates {
    pub fn new() -> Self {
        Self {
            score: 0,
            hero: Hero::new(0.0, 0.0),
            hero_bullets: vec![],
            enemies: vec![],
            enemy_bullets: vec![],
            spawn_cooldown: 0,
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
                self.hero_bullets.push(Box::new(Bullet {
                    motion_state: MotionState {
                        pos: self.hero.motion_state.pos,
                        speed: Vector2::y() * 8.0,
                        ..Default::default()
                    },
                }));
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
        }

        // spawn enemies
        self.spawn_cooldown = self.spawn_cooldown.saturating_sub(1);
        if self.spawn_cooldown == 0 {
            self.spawn_cooldown = SPAWN_COOLDOWN;
            let max_enemy_cnt = get_total_cnt_by_score(self.score);
            let gen_frac = get_gen_frac_by_score(self.score);
            let enemy_type = rand::random::<f32>();
            let gen_enemy = || {
                let x = rand::random::<f32>() * settings.width as f32;
                if enemy_type < gen_frac.0 {
                    Enemy::MiddleCup(Enemy1::new(x, settings.height as f32))
                } else if enemy_type < gen_frac.1 {
                    Enemy::BigCup(Enemy2::new(x, settings.height as f32))
                } else if enemy_type < gen_frac.2 {
                    Enemy::SuperBigCup(Enemy3::new(x, settings.height as f32))
                } else {
                    Enemy::MiddleCup(Enemy1::new(x, settings.height as f32))
                    // panic!("Never reach");
                }
            };
            let mut spawn_cnt = 0;
            while self.enemies.len() < max_enemy_cnt as usize {
                spawn_cnt += 1;
                let enemy = gen_enemy();
                self.enemies.push(Box::new(enemy));
                if spawn_cnt >= MAX_ENEMY_SPAWN_PER_TICK {
                    break;
                }
            }
        }

        // tick enemies
        for enemy in &mut self.enemies {
            match enemy.as_mut() {
                Enemy::MiddleCup(enemy) => {
                    if enemy.health == 0 && enemy.state != EntityState::DieAnimating {
                        enemy.state = EntityState::DieAnimating;
                    }
                    if collisioned(enemy, &self.hero) && self.hero.state == EntityState::Normal {
                        self.hero.state = EntityState::DieAnimating
                    }
                    enemy.tick(settings)
                }
                Enemy::BigCup(enemy) => {
                    if enemy.health == 0 && enemy.state != EntityState::DieAnimating {
                        enemy.state = EntityState::DieAnimating;
                    }
                    if collisioned(enemy, &self.hero) && self.hero.state == EntityState::Normal {
                        self.hero.state = EntityState::DieAnimating
                    }
                    enemy.tick(settings)
                }
                Enemy::SuperBigCup(enemy) => {
                    if enemy.health == 0 && enemy.state != EntityState::DieAnimating {
                        enemy.state = EntityState::DieAnimating;
                    }
                    if collisioned(enemy, &self.hero) && self.hero.state == EntityState::Normal {
                        self.hero.state = EntityState::DieAnimating
                    }
                    enemy.tick(settings)
                }
            }
        }

        // Retain hero_bullets:
        // - collisioned: update enemy health, and remove bullet
        // - out of screen: simply remove
        self.hero_bullets.retain(|bullet| {
            let bullet = bullet.as_ref();
            for enemy in &mut self.enemies {
                match enemy.as_mut() {
                    Enemy::MiddleCup(enemy) => {
                        if enemy.state == EntityState::DieAnimating
                            || enemy.state == EntityState::Died
                        {
                            continue;
                        }
                        if collisioned(bullet, enemy) {
                            console::log(&JsValue::from_str("collision 1").into());
                            enemy.health -= 1;
                            return false;
                        }
                    }
                    Enemy::BigCup(enemy) => {
                        if enemy.state == EntityState::DieAnimating
                            || enemy.state == EntityState::Died
                        {
                            continue;
                        }
                        if collisioned(bullet, enemy) {
                            enemy.state = EntityState::HittedAnimating;
                            console::log(&JsValue::from_str("collision 2").into());
                            enemy.health -= 1;
                            return false;
                        }
                    }
                    Enemy::SuperBigCup(enemy) => {
                        if enemy.state == EntityState::DieAnimating
                            || enemy.state == EntityState::Died
                        {
                            continue;
                        }
                        if collisioned(bullet, enemy) {
                            enemy.state = EntityState::HittedAnimating;
                            console::log(&JsValue::from_str("collision 3").into());
                            enemy.health -= 1;
                            return false;
                        }
                    }
                }
            }
            bullet.motion_state.pos.y > 0.0 && bullet.motion_state.pos.y < settings.height as f32
        });

        // Remove enemy with emtpy health and count score
        self.enemies.retain(|enemy| match enemy.as_ref() {
            Enemy::MiddleCup(enemy) => {
                if enemy.health <= 0 && enemy.state == EntityState::Died {
                    self.score += 1;
                    return false;
                };
                enemy.motion_state.pos.y > 0.0
            }
            Enemy::BigCup(enemy) => {
                if enemy.health <= 0 && enemy.state == EntityState::Died {
                    self.score += 5;
                    return false;
                };
                enemy.motion_state.pos.y > 0.0
            }
            Enemy::SuperBigCup(enemy) => {
                if enemy.health <= 0 && enemy.state == EntityState::Died {
                    self.score += 10;
                    return false;
                };
                enemy.motion_state.pos.y > 0.0
            }
        });
    }
}

const MAX_SMALL_ENEMY: u32 = 5;
const MAX_MIDDLE_ENEMY: u32 = 3;
const MAX_BIG_ENEMY: u32 = 2;
const MAX_SPEED_FRAC: f32 = 2.0;

/// Difficulty cauculation
fn get_gen_frac_by_score(score: u32) -> (f32, f32, f32) {
    let small = score / 3;

    let middle = small / (MAX_SMALL_ENEMY + 1);
    let small = small % (MAX_SMALL_ENEMY + 1);

    let big = middle / (MAX_MIDDLE_ENEMY + 1);
    let middle = middle % (MAX_MIDDLE_ENEMY + 1);

    let big = big.min(MAX_BIG_ENEMY);

    // let big = score / 1000 % 10;
    // let middle = score / 100 % 10;
    // let small = (score / 10 % 10).max(1);
    let total = (small + middle + big).max(1);

    (
        (small as f32 / total as f32),
        (middle as f32 / total as f32),
        (big as f32 / total as f32),
    )

    // (
    //     (small as f32 / total as f32).min(MAX_SMALL_ENEMY as f32 / 15.0),
    //     (middle as f32 / total as f32).min(MAX_MIDDLE_ENEMY as f32 / 15.0),
    //     (big as f32 / total as f32).min(MAX_BIG_ENEMY as f32 / 15.0),
    // )
}

fn get_total_cnt_by_score(score: u32) -> u32 {
    (score / 3).max(1).min(10)
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

    pub fn end(&self) -> bool {
        self.states.hero.state == EntityState::Died
    }

    pub fn score(&self) -> u32 {
        self.states.score
    }
}
