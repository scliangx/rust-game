use crate::components::{Airship, Bullt, Enemy, GameScore, Player, SpriteSize};
use crate::constant::*;
use crate::state::AppState;

use crate::Score;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (
                    score_system,
                    move_player_contrl,
                    confine_move_player,
                    spawn_player_bullt,
                    move_player_bullt,
                    enemy_laser_hit_player_system,
                    player_laser_despawn
                )
                    .run_if(in_state(AppState::Playing)),
            );
    }
}

fn score_system(
    mut commands: Commands,
    app_state: Res<State<AppState>>,
    windows: Query<&Window>,
    score: Res<Score>,
    query1: Query<Entity, With<GameScore>>,
) {
    if let Ok(e) = query1.get_single() {
        commands.entity(e).despawn();
    }
    // 通过窗口大小和棋盘大小计算stats位置
    let window = windows.single();
    // gameboard左上角在窗口上的位置
    let gameboard_left_corner_pos = (window.width() / 2.0 - 120.0, window.height() / 2.0 - 20.0);
    if let AppState::Playing = app_state.get() {
        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                // background_color: Color::rgb(0.4375, 0.5, 0.5625).into(),
                ..default()
            })
            .with_children(|parent| {
                // 标题
                parent.spawn((
                    TextBundle::from_section(
                        format!("Game Score: {}", score.0),
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
                    GameScore,
                ));
            });
    }
}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();
    let bottom = -window.height() / 2.;
    let x = 0.0;
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, bottom + PLAYER_SIZE.1 / 2., 0.0),
            texture: asset_server.load("player.png"),
            ..default()
        },
        Player,
        Airship,
        SpriteSize::from(PLAYER_SIZE),
    ));
}

// 控制飞船移动
fn move_player_contrl(
    mut query: Query<&mut Transform, (With<Player>, With<Airship>)>,
    keycode: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();

    let mut direction = Vec3::ZERO;
    if keycode.pressed(KeyCode::KeyS) {
        direction += Vec3::new(0.0, -1.0, 0.0);
    }
    if keycode.pressed(KeyCode::KeyW) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }
    if keycode.pressed(KeyCode::KeyA) {
        direction += Vec3::new(-1.0, 0.0, 0.0);
    }
    if keycode.pressed(KeyCode::KeyD) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }
    if direction.length() > 0.0 {
        direction = direction.normalize();
    }
    transform.translation += direction * 500.0 * time.delta_seconds();
}

// 控制边界
fn confine_move_player(
    mut query: Query<&mut Transform, (With<Player>, With<Airship>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let mut trandform = query.single_mut();
    let window = window_query.single();
    let half_player_width = PLAYER_SIZE.0 / 2.0;
    let half_player_heigt = PLAYER_SIZE.1 / 2.0;
    let x_min = -window.width() / 2.0 + half_player_width;
    let x_max = window.width() / 2.0 - half_player_width;
    let y_min = -window.height() / 2.0 + half_player_heigt;
    let y_max = window.height() / 2.0 - half_player_heigt;

    if trandform.translation.x > x_max {
        trandform.translation.x = x_max;
    } else if trandform.translation.x < x_min {
        trandform.translation.x = x_min;
    }

    if trandform.translation.y > y_max {
        trandform.translation.y = y_max;
    } else if trandform.translation.y < y_min {
        trandform.translation.y = y_min
    }
}

// 产生子弹
fn spawn_player_bullt(
    mut commands: Commands,
    query: Query<&Transform, (With<Player>, With<Airship>)>,
    keycode: Res<ButtonInput<KeyCode>>,
    assets_server: Res<AssetServer>,
) {
    let transform = query.single();
    if keycode.just_pressed(KeyCode::Space) {
        let mut transform = Transform::from_translation(
            transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR,
        );
        // 玩家上半部分的位置发射子弹
        transform.translation.y += 20.0;
        commands.spawn((
            SpriteBundle {
                texture: assets_server.load("laser.png"),
                transform: transform,
                ..default()
            },
            Player,
            Bullt,
            SpriteSize::from(PLAYER_LASER_SIZE),
        ));
    }
}

fn move_player_bullt(
    mut query: Query<(&mut Transform, &mut SpriteSize), (With<Bullt>, With<Player>)>,
    time: Res<Time>,
) {
    for (mut transform, mut sprite_szie) in query.iter_mut() {
        transform.translation.y += 500.0 * time.delta_seconds();
        sprite_szie.0.y = transform.translation.y;
    }
}

fn enemy_laser_hit_player_system(
    mut commands: Commands,
    mut score: ResMut<Score>,
    // window: Query<&Window, With<PrimaryWindow>>,
    // 玩家子弹
    laser_query: Query<(Entity, &Transform), (With<Player>, With<Bullt>)>,
    // 敌人
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
) {
    for (laser_entity, laser_tf) in laser_query.iter() {
        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
            // 不相同的两个entity之间的距离
            let distance = laser_tf.translation.xy().distance(enemy_tf.translation.xy());
            
            if distance < (enemy_size.0.x *0.5 * enemy_size.0.x*0.5 + enemy_size.0.y*0.5 * enemy_size.0.y*0.5).sqrt() {
                commands.entity(laser_entity).despawn();
                commands.entity(enemy_entity).despawn();
                score.0 += 10;
            }
            // if laser_size.0.y > y {
            //     commands.entity(laser_entity).despawn();
            // }
        }
    }
}


fn player_laser_despawn(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    // 玩家子弹
    laser_query: Query<(Entity, &Transform), (With<Player>, With<Bullt>)>,
) {
    let window = window.single();
    let top = window.height() / 2.0;
    for (enity,tf) in laser_query.iter() {
        if tf.translation.y > top {
            commands.entity(enity).despawn();
        }
    }
}
