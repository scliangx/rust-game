use bevy::prelude::*;
use crate::resources::*;
use crate::systems::*;

/// 贪吃蛇游戏的主要插件。
pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTimer(Timer::from_seconds(0.2, TimerMode::Repeating)))
            .insert_resource(Score(0))
            .insert_resource(GameOver(false))
            .insert_resource(GameOverShown(false))
            .insert_resource(Grow(false))
            .add_systems(Startup, setup)
            .add_systems(Update, (
                input_system,
                update_score_display,
                move_snake_system,
                collision_system,
                food_system,
                game_over_system,
            ));
    }
}
