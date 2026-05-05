//! 贪吃蛇游戏的组件模块。
//!
//! 此模块定义了游戏中使用的所有 ECS 组件，
//! 包括位置、方向和实体类型。

pub mod direction;
pub mod position;

pub use direction::SnakeDirection;
pub use position::*;
