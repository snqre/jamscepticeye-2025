use ::bevy::prelude::*;

mod macros;
mod animal;
mod camera_module;
mod player_controls;
mod debug_scene;
mod environment;
// mod model_previewer;
// mod assets;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(model_previewer::ModelViewerPlugin)
        .add_plugins(camera_module::TopDownCameraPlugin)
        .add_plugins(player_controls::PlayerControlsPlugin)
        .add_plugins(environment::EnvironmentPlugin)
        .add_plugins(debug_scene::DebugScenePlugin)
        .add_plugins(animal::AnimalPlugin)
        .run();
}
