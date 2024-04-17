use crate::components::GameMenu;
use crate::components::OnMainMenuScreen;
use crate::state::{AppState, GameState};
use bevy::app::AppExit;
use bevy::prelude::*;

pub struct MenuStatePlugin;

impl Plugin for MenuStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            menu_system.run_if(
                in_state(AppState::MainMenu)
                    .or_else(in_state(GameState::GameOver).or_else(in_state(GameState::Paused))),
            ),
        )
        .add_systems(
            OnExit(AppState::MainMenu),
            super::despawn_screen::<OnMainMenuScreen>,
        )
        .add_systems(
            Update,
            click_button.run_if(in_state(AppState::MainMenu).or_else(in_state(GameState::Paused))),
        )
        .add_systems(Update, pause_game.run_if(in_state(AppState::Playing)));
    }
}

fn menu_system(mut commands: Commands) {
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
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        width: Val::Percent(20.0),
                        height: Val::Percent(30.0),
                        ..default()
                    },
                    background_color: Color::rgb(0.4375, 0.5, 0.5625).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // 标题
                    parent.spawn(
                        TextBundle::from_section(
                            "GAME",
                            TextStyle {
                                font_size: 25.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        }),
                    );

                    // 开始游戏
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(90.0),
                                    height: Val::Px(30.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..default()
                            },
                            GameMenu::Start,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "StartGame",
                                TextStyle {
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            ));
                        });

                    // 退出按钮
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(90.0),
                                    height: Val::Px(30.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..default()
                            },
                            GameMenu::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Quit",
                                TextStyle {
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

// 点击事件
fn click_button(
    mut interaction_query: Query<(&Interaction, &GameMenu), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, menu_button_action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match menu_button_action {
                GameMenu::Start => {
                    info!("StartGame button clicked");
                    app_state.set(AppState::Playing);
                    game_state.set(GameState::InGame);
                }
                GameMenu::Quit => {
                    info!("Quit button clicked");
                    exit.send_default();
                }
            },
            _ => {}
        }
    }
}

// 暂停游戏
fn pause_game(
    game_state: Res<State<GameState>>,
    mut change_game_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if let GameState::InGame = game_state.get() {
            info!("just pressed ecs pause game. GameStae: InGame,Next: PausedGame");
            change_game_state.set(GameState::Paused)
        } else {
            info!("just pressed ecs pause game. GameStae: Paused,Next: InGame");
            change_game_state.set(GameState::InGame)
        }
    }
}
