//! 移动方向组件。
//!
//! 表示蛇的当前移动方向。

use bevy::prelude::*;

/// 表示移动方向的组件。
///
/// 使用整数值：每个轴为 -1、0 或 1。
#[derive(Component)]
pub struct SnakeDirection {
    /// X 方向：-1（左）、0（无移动）、1（右）
    pub x: i32,
    /// Y 方向：-1（下）、0（无移动）、1（上）
    pub y: i32,
}

impl Default for SnakeDirection {
    fn default() -> Self {
        Self { x: 1, y: 0 }
    }
}
