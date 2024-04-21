use crate::constant::*;
use crate::state::GameState;
use crate::EnemyCount;
use crate::{
    components::{Direction, Enemy, SpriteSize},
    resources::SpawnTimer,
};
use bevy::{prelude::*, window::PrimaryWindow};

use rand::{self, Rng};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer::new())
            .insert_resource(EnemyCount::default())
            .add_systems(
                Update,
                (spawn_enemy, enemy_move, check_direction).run_if(in_state(GameState::InGame)),
            );
    }
}

fn spawn_enemy(
    mut commands: Commands,
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
    let y = window.height() / 2.0;
    let x = window.width() / 2.0;
    let mut rng = rand::thread_rng();
    let (xx, yy) = (
        rng.gen_range(-x..x) as f32,
        rng.gen_range(-y / 2.0..y) as f32,
    );
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("enemy.png"),
            transform: Transform {
                translation: Vec3::new(xx, yy, 0.0),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..default()
            },
            ..default()
        },
        Enemy,
        Direction::default(),
        SpriteSize::from((ENEMY_SIZE.0 * 0.5, ENEMY_SIZE.1 * 0.5)),
    ));
    enemy_count.0 += 1;
}

fn enemy_move(mut query: Query<(&mut Transform, &Direction), With<Enemy>>, time: Res<Time>) {
    let speed = 100.0;

    for (mut tf, dir) in query.iter_mut() {
        let x: f32;
        let y: f32;
        if dir.left && dir.top {
            x = tf.translation.x - speed * time.delta_seconds();
            y = tf.translation.y + speed * time.delta_seconds();
        } else if dir.left && dir.down {
            x = tf.translation.x + speed * time.delta_seconds();
            y = tf.translation.y - speed * time.delta_seconds();
        } else if dir.right && dir.top {
            x = tf.translation.x + speed * time.delta_seconds();
            y = tf.translation.y + speed * time.delta_seconds();
        } else {
            x = tf.translation.x + speed * time.delta_seconds();
            y = tf.translation.y - speed * time.delta_seconds();
        }
        tf.translation = Vec3::new(x, y, 0.0)
    }
}

fn check_direction(
    window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&Transform, &mut Direction), With<Enemy>>,
) {
    let window = window.single();
    let win_size = (window.width(), window.height());
    for (tf, mut dir) in query.iter_mut() {
        if dir.left && tf.translation.x <= -win_size.0 / 2.0 {
            dir.left = false;
            dir.right =  true;
        } else if dir.right && tf.translation.x >= win_size.0 / 2.0 {
            dir.right = false;
            dir.left = true;
        }
        if dir.top && tf.translation.y >= win_size.1 / 2.0 {
            dir.top = false;
            dir.down = true;
        } else if dir.down && tf.translation.y <= -win_size.1 / 4.0 {
            dir.down = false;
            dir.top = true;
        };
    }
}
#[allow(dead_code)]
fn enemy_laser(
    mut commands: Commands,
    query: Query<(&Transform, &SpriteSize), (With<Enemy>, With<SpriteSize>)>,
) {
    for (_tf, _sp) in query.iter() {
        commands.spawn(SpriteBundle { ..default() });
    }
}
