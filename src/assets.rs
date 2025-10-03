use bevy::prelude::*;
use bevy::platform::collections::HashMap;
use bevy::transform::systems::propagate_parent_transforms;
use crate::event_exists;

mod hoppy_cube;
mod animations;

const ASSET_CAPACITY: usize = 1;

#[derive(Event, Copy, Clone)]
pub struct PropSpawn {
    pub iou: PropIOU,
    pub entity: Option<Entity>,
    pub transform: Option<Transform>
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PropIOU {
    HoppyCube
}

pub struct HomegrownAssetsPlugin;
impl Plugin for HomegrownAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PropSpawn>();
        app.insert_resource(AssetsAssets::new());
        app.add_systems(PostUpdate, iou_publisher.before(prop_spawn_event_reader));
        app.add_systems(
            PostUpdate, prop_spawn_event_reader
                .run_if(event_exists!(PropSpawn))
                .before(propagate_parent_transforms)
        );
    }
}

#[derive(Resource)]
struct AssetsAssets {
    assets: HashMap<PropIOU, IKEAProduct>
} impl AssetsAssets {
    fn new() -> Self {
        Self {
            assets: HashMap::with_capacity(ASSET_CAPACITY)
        }
    }
}

struct IKEAProduct {
    parts: Vec<(Handle<Mesh>, Handle<StandardMaterial>)>
} impl IKEAProduct {
    fn new(part_count: usize) -> Self {
        Self {
            parts: Vec::with_capacity(part_count)
        }
    }
    fn push(&mut self, mesh_handle: Handle<Mesh>, material_handle: Handle<StandardMaterial>) {
        self.parts.push((mesh_handle, material_handle))
    }
}

#[derive(Component)]
struct ConsumedIOU;

fn iou_publisher(
    mut event_writer: EventWriter<PropSpawn>,
    iou_query: Query<(Entity, &PropIOU), Without<ConsumedIOU>>,
    mut commands: Commands
) {
    for (entity, &prop) in iou_query {
        event_writer.write(
            PropSpawn {
                iou: prop,
                entity: Some(entity),
                transform: None
            }
        );
        commands.entity(entity).insert(ConsumedIOU);
    }
}

fn prop_spawn_event_reader(
    mut event_reader: EventReader<PropSpawn>,
    mut assets_assets: ResMut<AssetsAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands
) {
    for event in event_reader.read() {

    }
}
