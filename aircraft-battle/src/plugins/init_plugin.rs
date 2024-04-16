use bevy::{prelude::*, window::PrimaryWindow};

pub struct InitGamePlugin;

impl Plugin for InitGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
            .add_systems(Startup, init);
    }
}

fn init(mut commands: Commands, mut window: Query<&mut Window, With<PrimaryWindow>>) {
    commands.spawn(Camera2dBundle::default());
    let mut window = window.single_mut();
    window.title = "aircraft-battle".to_string();
}
