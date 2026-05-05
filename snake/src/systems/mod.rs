//! 游戏系统模块。

pub mod input_system;
pub mod move_snake_system;
pub mod collision_system;
pub mod food_system;
pub mod game_over_system;
pub mod setup_system;
pub mod score_system;

pub use input_system::*;
pub use move_snake_system::*;
pub use collision_system::*;
pub use food_system::*;
pub use game_over_system::*;
pub use setup_system::*;
pub use score_system::*;
