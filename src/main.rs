use ::bevy::prelude::*;

mod camera_module;
mod player_controls;
mod debug_scene;
mod environment;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(camera_module::TopDownCameraPlugin)
        .add_plugins(player_controls::PlayerControlsPlugin)
        .add_plugins(environment::EnvironmentPlugin)
        .add_plugins(debug_scene::DebugScenePlugin)
        .run();
}
