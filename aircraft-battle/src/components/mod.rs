
use bevy::prelude::*;


// 玩家组件
#[derive(Component,Debug)]
pub struct Player;

#[derive(Component,Debug)]
pub struct Airship;

// 敌人
#[derive(Component,Debug)]
pub struct Enemy;

// 子弹
#[derive(Component,Debug)]
pub struct Bullt;


// 分数
#[derive(Component,Debug)]
pub struct GameScore;


#[derive(Component,Debug)]
pub enum GameMenu {
    Start,
    Quit,
}


#[derive(Component,Debug)]
pub struct OnMainMenuScreen;


#[derive(Component,Debug)]
pub struct ExplosionToSpawn(pub Vec3);


// 大小
#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
	fn from(val: (f32, f32)) -> Self {
		SpriteSize(Vec2::new(val.0, val.1))
	}
}