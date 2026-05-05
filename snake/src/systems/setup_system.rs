use bevy::prelude::*;
use crate::components::{SnakeDirection, SnakeHead, SnakeBody, Position, ScoreDisplay};
use crate::constants::*;
use crate::systems::spawn_food;

/// 游戏初始化：相机、网格线、蛇、食物、分数显示。
pub fn setup(mut commands: Commands) {
    info!("Setting up game...");

    // 2D 相机
    commands.spawn(Camera2dBundle::default());

    // 绘制背景网格线 (z=-1 保证在游戏对象后面)
    let w = GRID_WIDTH as f32 * GRID_SIZE;
    let h = GRID_HEIGHT as f32 * GRID_SIZE;
    for i in 0..=GRID_WIDTH {
        let x = i as f32 * GRID_SIZE - w / 2.0;
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: GRID_LINE_COLOR,
                custom_size: Some(Vec2::new(1.0, h)),
                ..default()
            },
            transform: Transform::from_xyz(x, 0.0, -1.0),
            ..default()
        });
    }
    for i in 0..=GRID_HEIGHT {
        let y = i as f32 * GRID_SIZE - h / 2.0;
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: GRID_LINE_COLOR,
                custom_size: Some(Vec2::new(w, 1.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, y, -1.0),
            ..default()
        });
    }

    // 分数显示 UI（绝对定位在左上角）
    commands.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        ScoreDisplay,
    ));

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

    // 初始食物
    spawn_food(&mut commands, &[(10, 10), (9, 10)]);

    info!("Game setup complete!");
}
