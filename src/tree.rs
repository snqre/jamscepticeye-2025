use super::*;

#[derive(Component)]
#[require(Transform)]
pub struct Tree;

impl common::RandomSpawnEntityConstructor for Tree {
    type Bundle = (Self, Transform);

    fn new(_: &World, position: Vec3) -> Self::Bundle {(
        Self,
        position.into()
    )}
}

pub struct Plugin;

impl ::bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |world: &mut World| common::RandomSpawnSystem::<Tree>::on_startup(world));
    }
}