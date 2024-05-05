use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum BitmapAsset {
	BulletEnemy, // 5 11
	BulletPlayer,
	Enemy1, // 57 43
	Enemy1Down1, // 57 51
	Enemy1Down2,
	Enemy1Down3,
	Enemy1Down4,
	Enemy2, // 69 99
	Enemy2Hit,
	Enemy2Down1, // 69 95
	Enemy2Down2,
	Enemy2Down3,
	Enemy2Down4,
	Enemy3N1, // 169 258
	Enemy3N2,
	Enemy3Hit,
	Enemy3Down1, // 165 261
	Enemy3Down2,
	Enemy3Down3, // 165 260
	Enemy3Down4, // 165 261
	Enemy3Down5, // 166 260
	Enemy3Down6, /// 166 261
	Hero1, // 102 126
	Hero2,
	HeroDown1,
	HeroDown2,
	HeroDown3,
	HeroDown4,
}

#[wasm_bindgen]
pub fn bitmap_filename(bitmap_asset: BitmapAsset) -> String {
	match bitmap_asset {
		BitmapAsset::BulletEnemy => "bullet_enemy.png",
		BitmapAsset::BulletPlayer => "bullet_player.png",
		BitmapAsset::Enemy1 => "enemy1.png",
		BitmapAsset::Enemy1Down1 => "enemy1_down1.png",
		BitmapAsset::Enemy1Down2 => "enemy1_down2.png",
		BitmapAsset::Enemy1Down3 => "enemy1_down3.png",
		BitmapAsset::Enemy1Down4 => "enemy1_down4.png",
		BitmapAsset::Enemy2 => "enemy2.png",
		BitmapAsset::Enemy2Hit => "enemy2_hit.png",
		BitmapAsset::Enemy2Down1 => "enemy2_down1.png",
		BitmapAsset::Enemy2Down2 => "enemy2_down2.png",
		BitmapAsset::Enemy2Down3 => "enemy2_down3.png",
		BitmapAsset::Enemy2Down4 => "enemy2_down4.png",
		BitmapAsset::Enemy3N1 => "enemy3_n1.png",
		BitmapAsset::Enemy3N2 => "enemy3_n2.png",
		BitmapAsset::Enemy3Hit => "enemy3_hit.png",
		BitmapAsset::Enemy3Down1 => "enemy3_down1.png",
		BitmapAsset::Enemy3Down2 => "enemy3_down2.png",
		BitmapAsset::Enemy3Down3 => "enemy3_down3.png",
		BitmapAsset::Enemy3Down4 => "enemy3_down4.png",
		BitmapAsset::Enemy3Down5 => "enemy3_down5.png",
		BitmapAsset::Enemy3Down6 => "enemy3_down6.png",
		BitmapAsset::Hero1 => "hero1.png",
		BitmapAsset::Hero2 => "hero2.png",
		BitmapAsset::HeroDown1 => "hero_down1.png",
		BitmapAsset::HeroDown2 => "hero_down2.png",
		BitmapAsset::HeroDown3 => "hero_down3.png",
		BitmapAsset::HeroDown4 => "hero_down4.png",
	}.to_string()
}

#[allow(unused)]
#[wasm_bindgen]
pub struct Primitive {
	bitmap: BitmapAsset,
	position: (f32, f32), // x, y
	rotate_angle_rad: f32, // rotation in rad
}

impl Primitive {
	pub fn new(bitmap: BitmapAsset, position: (f32, f32), rotate_angle_rad: f32) -> Self {
		Self {
			bitmap,
			position,
			rotate_angle_rad,
		}
	}
}

pub trait Render {
	fn render(&self, ms_delta: u128) -> Primitive;
}
