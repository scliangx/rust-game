use std::time::Duration;

use bevy::prelude::*;


#[derive(Resource,Debug,Default)]
pub struct GameAssets{
    pub player_assets: Handle<Scene>,
    pub lase_assets: Handle<Scene>,
    pub enemy_assets: Handle<Scene>,
}

#[derive(Resource, Debug,Default)]
pub struct SpawnTimer {
    pub timer: Timer,
}

impl SpawnTimer {
    pub fn new() -> Self {
        return SpawnTimer{
            timer: Timer::new(Duration::from_millis(1000), TimerMode::Repeating)
        };
    }
}


#[derive(Resource,Debug,Default)]
pub struct Score(pub u32);


// 敌人的数量
#[derive(Resource,Debug,Default)]
pub struct EnemyCount(pub u32);


impl EnemyCount {
    pub fn new(count:u32) -> Self {
        return EnemyCount(count);
    }
}