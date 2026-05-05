//! 位置和实体类型组件。

use bevy::prelude::*;

/// 标记蛇头实体的组件。
#[derive(Component)]
pub struct SnakeHead;

/// 标记蛇身体段实体的组件。
#[derive(Component)]
pub struct SnakeBody;

/// 标记食物实体的组件。
#[derive(Component)]
pub struct Food;

/// 标记游戏结束文本实体的组件。
#[derive(Component)]
pub struct GameOverText;

/// 标记分数显示 UI 实体的组件。
#[derive(Component)]
pub struct ScoreDisplay;

/// 表示游戏网格上位置的组件。
#[derive(Component, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
