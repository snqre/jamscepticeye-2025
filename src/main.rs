use ::bevy::prelude::*;

mod camera_module;
mod player_controls;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(camera_module::TopDownCameraPlugin)
        .add_plugins(player_controls::PlayerControlsPlugin)
        .run();
}
