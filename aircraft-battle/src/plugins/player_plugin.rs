use crate::components::GameScore;
use crate::state::{GameState, AppState};

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, score_system.run_if(in_state(GameState::InGame)));
    }
}

fn score_system(mut commands: Commands, app_state: Res<State<AppState>>, windows: Query<&Window>) {
    // 通过窗口大小和棋盘大小计算stats位置
    let window = windows.single();
    // gameboard左上角在窗口上的位置
    let gameboard_left_corner_pos = (
        window.width() / 2.0 - 120.0,
        window.height() / 2.0 - 20.0,
    );
    if let AppState::Playing = app_state.get() {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.4375, 0.5, 0.5625).into(),
                    ..default()
                },
                GameScore,
            ))
            .with_children(|parent| {
                // 标题
                parent.spawn(
                    TextBundle::from_section(
                        "Game Score: 0.0",
                        TextStyle {
                            font_size: 25.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    )
                    .with_style(Style {
                        // margin: UiRect::all(Val::Px(20.0)),
                        // ..default()
                        // position_type: PositionType::Absolute,
                        top: Val::Px(-gameboard_left_corner_pos.1),
                        left: Val::Px(gameboard_left_corner_pos.0),
                        ..default()
                    }),
                );
            });
    }
}
