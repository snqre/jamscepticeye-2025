use bevy::prelude::*;
use crate::event_exists;

mod hoppy_cube;
mod animations;

#[derive(Event, Copy, Clone)]  // for spawning props directly
pub struct PropSpawn {
    pub iou: PropIOU,
    pub entity: Option<Entity>,
    pub transform: Option<Transform>
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash)]  // for spawning props as children when spawning entities
pub enum PropIOU {
    HoppyCube
}

#[derive(Component)]
pub struct PropPartMarker;

pub struct HomegrownAssetsPlugin;
impl Plugin for HomegrownAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PropSpawn>();
        app.add_systems(PostUpdate, iou_consumer.before(prop_spawn_event_reader));
        app.add_systems(PostUpdate, prop_spawn_event_reader
                .before(update_motion_trackers)
                .run_if(event_exists!(PropSpawn))
        );
        app.add_systems(Startup, assets_assets_setup);
        app.add_systems(PostUpdate, update_motion_trackers);
        app.add_systems(Update, (
            hoppy_cube::animate
        ));
    }
}

fn assets_assets_setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands
) {
    let (hoppy_mesh, hoppy_material)
        = hoppy_cube::generate_assets(&mut meshes, &mut materials);
    commands.insert_resource(
        AssetsAssets {
            hoppy_mesh, hoppy_material
        }
    );
}

#[derive(Resource)]
struct AssetsAssets {
    hoppy_mesh: Handle<Mesh>,
    hoppy_material: Handle<StandardMaterial>
}


impl PropIOU {
    fn spawn(
        &self,
        commands: &mut Commands,
        assets_assets: &Res<AssetsAssets>,
        parent: Entity
    ) {
        match self {
            Self::HoppyCube => hoppy_cube::spawn(commands, parent, &assets_assets)
        }
    }
}

#[derive(Component)]
struct ConsumedIOU;

fn iou_consumer(
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
    assets_assets: Res<AssetsAssets>,
    mut commands: Commands
) {
    for event in event_reader.read() {
        let parent = if let Some(e) = event.entity {
            e
        } else {
            commands.spawn((
                event.transform.unwrap_or_default(),
                event.iou,
                ConsumedIOU
            )).id()
        };
        event.iou.spawn(&mut commands, &assets_assets, parent);
    }
}

#[derive(Component)]
pub struct MotionTracker {
    pub prop_parts: Vec<Entity>,
    pub velocity: Vec2,
    pub speed: f32,
    time: f32,
    dt: f32,
    moving: bool,
    initialized: bool,
    alive: bool,
    last_pos: Vec2
} impl MotionTracker {
    pub fn new(prop_parts: Vec<Entity>) -> Self {
        Self {
            prop_parts,
            velocity: Vec2::ZERO,
            speed: 0.0,
            time: 0.0,
            dt: 0.0,
            moving: false,
            initialized: false,
            alive: true,
            last_pos: Vec2::ZERO,
        }
    }
    pub fn update(&mut self, new_pos: Vec2, dt: f32) {
        if !self.initialized {
            self.last_pos = new_pos;
            self.initialized = true;
            self.dt = dt;
        };
        let delta = new_pos - self.last_pos;
        self.velocity = delta / dt;
        self.speed = self.velocity.length();
        self.last_pos = new_pos;
        self.time += dt;
        self.dt = dt;
        if (self.speed <= 0.0) == self.moving {
            self.moving = !self.moving;
            self.time = 0.0;
        };
    }
    pub fn kill(&mut self) {
        self.alive = false;
    }
}

fn update_motion_trackers(
    time: Res<Time>,
    query: Query<(&Transform, &mut MotionTracker)>
) {
    let dt = time.delta_secs();
    for (transform, mut motion_tracker) in query {
        motion_tracker.update(transform.translation.xy(), dt);
    };
}
