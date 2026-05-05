//! 游戏常量和配置值。

use bevy::prelude::*;

/// 每个网格单元的大小（像素）。
pub const GRID_SIZE: f32 = 20.0;

/// 游戏网格的宽度（单元格数）。
pub const GRID_WIDTH: i32 = 20;

/// 游戏网格的高度（单元格数）。
pub const GRID_HEIGHT: i32 = 20;

/// 蛇头颜色
pub const HEAD_COLOR: Color = Color::rgb(0.2, 0.9, 0.3);

/// 蛇身颜色
pub const BODY_COLOR: Color = Color::rgb(0.1, 0.5, 0.2);

/// 网格线颜色
pub const GRID_LINE_COLOR: Color = Color::rgb(0.08, 0.08, 0.08);
