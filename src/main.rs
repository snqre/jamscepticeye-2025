use ::bevy::prelude::*;

mod example;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(example::Plugin)
        .run()
}
