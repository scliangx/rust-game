//! 游戏状态资源。
//!
//! 维护游戏全局状态的资源。

use bevy::prelude::*;

/// 控制蛇移动速度的计时器资源。
#[derive(Resource, Reflect)]
pub struct GameTimer(pub Timer);

/// 当前游戏分数。
#[derive(Resource, Reflect)]
pub struct Score(pub u32);

/// 标志游戏是否结束。
#[derive(Resource, Reflect, PartialEq)]
pub struct GameOver(pub bool);

/// 标志蛇是否应在下一次移动时增长。
#[derive(Resource, Reflect)]
pub struct Grow(pub bool);

/// 标志游戏结束文本是否已显示，防止每帧重复创建。
#[derive(Resource, Reflect)]
pub struct GameOverShown(pub bool);