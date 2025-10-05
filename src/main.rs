use ::bevy::prelude::*;

// mod model_previewer;

mod macros;
mod animal;
mod camera_module;
mod common;
mod player_controls;
mod debug_scene;
mod environment;
mod player;
mod tree;
// mod model_previewer;
// mod assets;
mod assets;

const WORLD_W: f32 = 0.0;
const WORLD_H: f32 = 0.0;

trait Model {
    type Output;

    fn simulate(self) -> Self::Output;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(model_previewer::ModelViewerPlugin)
        .add_plugins(camera_module::TopDownCameraPlugin)
        .add_plugins(player_controls::PlayerControlsPlugin)
        .add_plugins(environment::EnvironmentPlugin)
        .add_plugins(debug_scene::DebugScenePlugin)
        .add_plugins(animal::Plugin)
        .add_plugins(assets::HomegrownAssetsPlugin)
        .add_plugins(tree::Plugin)
        .run();
}
