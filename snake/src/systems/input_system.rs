use bevy::prelude::*;
use crate::components::SnakeDirection;
use crate::components::SnakeHead;
use crate::resources::GameOver;

/// 纯键盘输入系统。
/// 方向键 / WASD 控制，只能 90 度转向，不能 180 度反向。
pub fn input_system(
    game_over: Res<GameOver>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut SnakeDirection, With<SnakeHead>>,
) {
    if game_over.0 {
        return;
    }

    let Ok(mut direction) = query.get_single_mut() else {
        return;
    };

    let go_left = keyboard_input.just_pressed(KeyCode::ArrowLeft) || keyboard_input.just_pressed(KeyCode::KeyA);
    let go_right = keyboard_input.just_pressed(KeyCode::ArrowRight) || keyboard_input.just_pressed(KeyCode::KeyD);
    let go_up = keyboard_input.just_pressed(KeyCode::ArrowUp) || keyboard_input.just_pressed(KeyCode::KeyW);
    let go_down = keyboard_input.just_pressed(KeyCode::ArrowDown) || keyboard_input.just_pressed(KeyCode::KeyS);

    // 方向切换只能 90 度：水平移动时只能切到垂直，反之亦然
    if go_left && direction.x == 0 {
        direction.x = -1;
        direction.y = 0;
    } else if go_right && direction.x == 0 {
        direction.x = 1;
        direction.y = 0;
    } else if go_up && direction.y == 0 {
        direction.x = 0;
        direction.y = 1;
    } else if go_down && direction.y == 0 {
        direction.x = 0;
        direction.y = -1;
    }
}
