use bevy::prelude::*;


#[derive(Resource,Default)]
pub struct GameAssets{

}

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct Score(pub u32);