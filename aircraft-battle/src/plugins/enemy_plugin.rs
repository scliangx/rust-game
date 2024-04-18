use bevy::math::bounding::BoundingCircle;
use bevy::{prelude::*, window::PrimaryWindow};
use crate::components::{SpriteSize,Bullt};
use crate::{components::Enemy, resources::SpawnTimer};
use crate::state::GameState;
use crate::EnemyCount;
use crate::constant::*;

use rand::{self, Rng};


pub struct EnemyPlugin;


impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer::new())
        .add_systems(Update, spawn_enemy.run_if(in_state(GameState::InGame)));
    }
}


fn spawn_enemy(mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut enemy_count: ResMut<EnemyCount>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    if enemy_count.0 >= 10 {
        return;
    }
    let window = window_query.single();
    let y = window.height()/2.0;
    let x = window.width()/2.0;
    let mut rng = rand::thread_rng();
    let (xx,yy) = (rng.gen_range(-x..x) as f32,rng.gen_range(-y/2.0..y)as f32);
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("enemy.png"),
            transform:  Transform{
                translation: Vec3::new(xx,yy  , 0.0),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..default()
            },
            ..default()
        },
        Enemy,
        Bullt,
        SpriteSize::from((ENEMY_SIZE.0 * 0.5,ENEMY_SIZE.1 * 0.5))
    ));
    enemy_count.0 += 1;
}


#[allow(dead_code)]
fn enemy_laser(mut commands: Commands,query: Query<(&Transform,&SpriteSize),(With<Enemy>,With<SpriteSize>)>) {
    for (_tf,_sp) in query.iter() {
        commands.spawn(SpriteBundle{
            ..default()
        });
    }
}