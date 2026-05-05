use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use crate::systems::spawn_food;

/// 游戏结束系统：显示结束文本，处理 R 键重新开始。
pub fn game_over_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_over: Res<GameOver>,
    score: Res<Score>,
    mut shown: ResMut<GameOverShown>,
    query_snake_head: Query<Entity, With<SnakeHead>>,
    query_snake_body: Query<Entity, With<SnakeBody>>,
    query_food: Query<Entity, With<Food>>,
    query_text: Query<Entity, With<GameOverText>>,
) {
    if !game_over.0 {
        return;
    }

    if !shown.0 {
        let h = GRID_HEIGHT as f32 * GRID_SIZE;
        commands.spawn((
            TextBundle::from_section(
                format!("Game Over! Score: {}\nPress R to restart", score.0),
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(h / 2.0 - 40.0),
                left: Val::Auto,
                right: Val::Auto,
                ..default()
            }),
            GameOverText,
        ));
        shown.0 = true;
    }

    if keyboard_input.just_pressed(KeyCode::KeyR) {
        // 清理所有游戏实体
        for entity in query_snake_head.iter() {
            commands.entity(entity).despawn();
        }
        for entity in query_snake_body.iter() {
            commands.entity(entity).despawn();
        }
        for entity in query_food.iter() {
            commands.entity(entity).despawn();
        }
        for entity in query_text.iter() {
            commands.entity(entity).despawn();
        }

        // 重置资源
        commands.insert_resource(Score(0));
        commands.insert_resource(GameOver(false));
        commands.insert_resource(GameOverShown(false));
        commands.insert_resource(Grow(false));
        commands.insert_resource(GameTimer(Timer::from_seconds(0.2, TimerMode::Repeating)));

        let w = GRID_WIDTH as f32 * GRID_SIZE;
        let h = GRID_HEIGHT as f32 * GRID_SIZE;

        // 蛇头
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: HEAD_COLOR,
                    custom_size: Some(Vec2::new(GRID_SIZE, GRID_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    10.0 * GRID_SIZE - w / 2.0 + GRID_SIZE / 2.0,
                    10.0 * GRID_SIZE - h / 2.0 + GRID_SIZE / 2.0,
                    0.0,
                ),
                ..default()
            },
            SnakeHead,
            Position { x: 10, y: 10 },
            SnakeDirection { x: 1, y: 0 },
        ));

        // 蛇身
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: BODY_COLOR,
                    custom_size: Some(Vec2::new(GRID_SIZE, GRID_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    9.0 * GRID_SIZE - w / 2.0 + GRID_SIZE / 2.0,
                    10.0 * GRID_SIZE - h / 2.0 + GRID_SIZE / 2.0,
                    0.0,
                ),
                ..default()
            },
            SnakeBody,
            Position { x: 9, y: 10 },
        ));

        spawn_food(&mut commands, &[(10, 10), (9, 10)]);
    }
}
