use bevy::{prelude::*, window::PrimaryWindow};
use crate::components::SpriteSize;
use crate::{components::Enemy, resources::SpawnTimer};
use rand::{self, Rng};
use crate::state::GameState;
use crate::EnemyCount;


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
            transform: Transform::from_xyz(xx,yy  , 0.0),
            texture: asset_server.load("enemy.png"),
            ..default()
        },
        Enemy,
        SpriteSize::from((xx,yy))
    ));
    enemy_count.0 += 1;
}