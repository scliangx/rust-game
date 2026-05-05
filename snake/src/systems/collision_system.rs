use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::systems::spawn_food;

/// 处理碰撞检测的系统。
///
/// 检查蛇与自身和食物的碰撞（穿墙逻辑在 move_snake_system 中处理）。
pub fn collision_system(
    mut commands: Commands,
    game_over: Res<GameOver>,
    query_head: Query<&Position, With<SnakeHead>>,
    query_body: Query<&Position, With<SnakeBody>>,
    query_food: Query<(Entity, &Position), With<Food>>,
    mut score: ResMut<Score>,
    mut grow: ResMut<Grow>,
) {
    if game_over.0 {
        return;
    }
    let Ok(head_pos) = query_head.get_single() else {
        error!("Snake head not found in collision system!");
        return;
    };

    // 收集所有被占据的位置
    let mut occupied: Vec<(i32, i32)> = query_body.iter().map(|p| (p.x, p.y)).collect();
    occupied.push((head_pos.x, head_pos.y));

    // 检查自身碰撞
    for body_pos in query_body.iter() {
        if head_pos.x == body_pos.x && head_pos.y == body_pos.y {
            commands.insert_resource(GameOver(true));
            return;
        }
    }

    // 检查食物碰撞
    for (food_entity, food_pos) in query_food.iter() {
        if head_pos.x == food_pos.x && head_pos.y == food_pos.y {
            commands.entity(food_entity).despawn();
            spawn_food(&mut commands, &occupied);
            score.0 += 1;
            grow.0 = true;
        }
    }
}
