mod menu_state_plugin;
mod assets_plugin;
mod player_plugin;
mod enemy_plugin;
mod init_plugin;
mod debug_plugin;

pub use menu_state_plugin::MenuStatePlugin;
pub use assets_plugin::AssetsPlugin;
pub use player_plugin::PlayerPlugin;
pub use enemy_plugin::EnemyPlugin;
pub use init_plugin::InitGamePlugin;
pub use debug_plugin::DebugPlugin;


use bevy::prelude::*;

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}