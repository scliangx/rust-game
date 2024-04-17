use bevy::prelude::*;

mod components;
mod constant;
mod plugins;
mod resources;
mod state;

pub use components::*;
pub use constant::*;
use plugins::{
    AssetsPlugin, EnemyPlugin, InitGamePlugin, MenuStatePlugin, PlayerPlugin,DebugPlugin,
};
pub use resources::GameAssets;
pub use state::{AppState,GameState};


fn main() {
    App::new()
        // 添加默认插件
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .init_state::<GameState>()
        // 游戏初始化相关插件，背景颜色，窗口大小，标题
        .add_plugins(InitGamePlugin)
        .add_plugins(DebugPlugin)
        // 资源相关插件
        .add_plugins(AssetsPlugin)
        // 菜单插件
        .add_plugins(MenuStatePlugin)
        // 玩家相关插件
        .add_plugins(PlayerPlugin)
        // 敌人相关插件
        .add_plugins(EnemyPlugin)
        .run();

    println!("aircraft battle!");
}
