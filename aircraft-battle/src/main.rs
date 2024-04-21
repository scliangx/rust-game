use bevy::prelude::*;

pub mod components;
pub mod constant;
pub mod resources;

mod plugins;
mod state;

use plugins::{
    DebugPlugin, EnemyPlugin, InitGamePlugin, MenuStatePlugin, PlayerPlugin,
};
use resources::{EnemyCount,Score};
pub use state::{AppState, GameState};


fn main() {
    App::new()
        // 添加默认插件
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .init_state::<GameState>()
        // 初始化相关资源
        // .init_resource::<GameAssets>()
        // 游戏初始化相关插件，背景颜色，窗口大小，标题
        .add_plugins(InitGamePlugin)
        .add_plugins(DebugPlugin)
        // 菜单插件
        .add_plugins(MenuStatePlugin)
        // 玩家相关插件
        .add_plugins(PlayerPlugin)
        // 敌人相关插件
        .add_plugins(EnemyPlugin)
        .run();

    println!("aircraft battle!");
}
