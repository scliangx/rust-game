use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::constants::*;

/// 在未被占据的随机位置生成食物。
pub fn spawn_food(commands: &mut Commands, occupied: &[(i32, i32)]) {
    let mut rng = rand::thread_rng();
    let (x, y) = loop {
        let x = rng.gen_range(0..GRID_WIDTH);
        let y = rng.gen_range(0..GRID_HEIGHT);
        if !occupied.contains(&(x, y)) {
            break (x, y);
        }
    };

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(GRID_SIZE, GRID_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(
                x as f32 * GRID_SIZE - (GRID_WIDTH as f32 * GRID_SIZE) / 2.0 + GRID_SIZE / 2.0,
                y as f32 * GRID_SIZE - (GRID_HEIGHT as f32 * GRID_SIZE) / 2.0 + GRID_SIZE / 2.0,
                0.0,
            ),
            ..default()
        },
        Food,
        Position { x, y },
    ));
}

/// 食物系统。
///
/// 食物在碰撞系统中生成。
pub fn food_system() {
    // 食物在碰撞系统中生成
}
