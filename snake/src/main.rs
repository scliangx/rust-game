//! # 贪吃蛇游戏
//!
//! 使用 Rust 和 Bevy 游戏引擎实现的简单贪吃蛇游戏。
//!
//! ## 控制方式
//! - 方向键 / WASD 控制方向
//! - 吃到食物增长并增加分数
//! - 撞墙穿出，咬到自己则游戏结束

use bevy::prelude::*;
use bevy::render::camera::ClearColor;
use plugins::SnakePlugin;

mod components;
mod constants;
mod plugins;
mod resources;
mod systems;

fn main() {
    App::new()
        // 在 DefaultPlugins 之前设置 ClearColor，防止第一帧闪现白色
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake Game".to_string(),
                resolution: (constants::GRID_WIDTH as f32 * constants::GRID_SIZE, constants::GRID_HEIGHT as f32 * constants::GRID_SIZE).into(),
                focused: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(SnakePlugin)
        .run();
}
