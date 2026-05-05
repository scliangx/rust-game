use bevy::prelude::*;
use crate::components::ScoreDisplay;
use crate::resources::Score;

/// 每帧更新分数显示 UI 的系统。
pub fn update_score_display(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreDisplay>>,
) {
    if !score.is_changed() {
        return;
    }
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!("Score: {}", score.0);
    }
}
