use nalgebra::Point2;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
pub enum BitmapAsset {
	BulletEnemy,
	BulletPlayer,
	Enemy1,
	Enemy1Down1,
	Enemy1Down2,
	Enemy1Down3,
	Enemy1Down4,
	Enemy2,
	Enemy2Hit,
	Enemy2Down1,
	Enemy2Down2,
	Enemy2Down3,
	Enemy2Down4,
	Enemy3N1,
	Enemy3N2,
	Enemy3Hit,
	Enemy3Down1,
	Enemy3Down2,
	Enemy3Down3,
	Enemy3Down4,
	Enemy3Down5,
	Enemy3Down6,
	Hero1,
	Hero2,
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
	}.to_string()
}

#[allow(unused)]
#[wasm_bindgen]
pub struct Primitive {
	bitmap: BitmapAsset,
	position: (f32, f32), // x, y
}

impl Primitive {
	pub fn new(bitmap: BitmapAsset, position: (f32, f32)) -> Self {
		Self {
			bitmap,
			position,
		}
	}
}

pub trait Render {
	fn render(&self, ms_delta: u128) -> Primitive;
}
