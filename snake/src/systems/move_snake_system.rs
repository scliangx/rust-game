use bevy::prelude::*;
use crate::components::{SnakeDirection, SnakeHead, SnakeBody, Position};
use crate::constants::*;
use crate::resources::*;

/// 移动蛇的系统。撞墙时从对面穿出。
pub fn move_snake_system(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut commands: Commands,
    game_over: Res<GameOver>,
    mut head_query: Query<(Entity, &mut Position, &SnakeDirection, &mut Transform), (With<SnakeHead>, Without<SnakeBody>)>,
    mut body_query: Query<(Entity, &mut Position, &mut Transform), (With<SnakeBody>, Without<SnakeHead>)>,
    mut grow: ResMut<Grow>,
) {
    if game_over.0 {
        return;
    }

    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let Ok((_head_entity, mut head_pos, direction, mut head_transform)) = head_query.get_single_mut() else {
        return;
    };

    // 穿墙
    let mut new_x = head_pos.x + direction.x;
    let mut new_y = head_pos.y + direction.y;
    if new_x < 0 { new_x = GRID_WIDTH - 1; }
    else if new_x >= GRID_WIDTH { new_x = 0; }
    if new_y < 0 { new_y = GRID_HEIGHT - 1; }
    else if new_y >= GRID_HEIGHT { new_y = 0; }

    let old_x = head_pos.x;
    let old_y = head_pos.y;

    head_pos.x = new_x;
    head_pos.y = new_y;
    head_transform.translation.x = head_pos.x as f32 * GRID_SIZE - (GRID_WIDTH as f32 * GRID_SIZE) / 2.0 + GRID_SIZE / 2.0;
    head_transform.translation.y = head_pos.y as f32 * GRID_SIZE - (GRID_HEIGHT as f32 * GRID_SIZE) / 2.0 + GRID_SIZE / 2.0;

    // 移动身体
    let mut prev_x = old_x;
    let mut prev_y = old_y;
    let mut tail_x = 0;
    let mut tail_y = 0;
    let mut has_tail = false;

    for (_entity, mut pos, mut transform) in body_query.iter_mut() {
        let temp_x = pos.x;
        let temp_y = pos.y;
        pos.x = prev_x;
        pos.y = prev_y;
        transform.translation.x = pos.x as f32 * GRID_SIZE - (GRID_WIDTH as f32 * GRID_SIZE) / 2.0 + GRID_SIZE / 2.0;
        transform.translation.y = pos.y as f32 * GRID_SIZE - (GRID_HEIGHT as f32 * GRID_SIZE) / 2.0 + GRID_SIZE / 2.0;
        prev_x = temp_x;
        prev_y = temp_y;
        tail_x = temp_x;
        tail_y = temp_y;
        has_tail = true;
    }

    if grow.0 && has_tail {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: BODY_COLOR,
                    custom_size: Some(Vec2::new(GRID_SIZE, GRID_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    tail_x as f32 * GRID_SIZE - (GRID_WIDTH as f32 * GRID_SIZE) / 2.0 + GRID_SIZE / 2.0,
                    tail_y as f32 * GRID_SIZE - (GRID_HEIGHT as f32 * GRID_SIZE) / 2.0 + GRID_SIZE / 2.0,
                    0.0,
                ),
                ..default()
            },
            SnakeBody,
            Position { x: tail_x, y: tail_y },
        ));
        grow.0 = false;
    }
}
