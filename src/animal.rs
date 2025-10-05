use super::*;

pub const SPHERE_RADIUS: f32 = 0.25;
pub const Z: f32 = 0.0;
pub const SPEED: f32 = 1.0;
pub const DISTANCE: f32 = 10.0;

pub struct Plugin;

impl ::bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |world: &mut World| common::RandomSpawnSystem::<Animal>::on_startup(world));
        app.add_systems(Startup, AssetSystem::on_startup);
        app.add_systems(Update, MotionSystem::on_update);
        app.add_systems(Update, {
            SpawnEventSystem::on_update.run_if(event_exists!(SpawnEvent))
        });
    }
}

#[derive(Component)]
#[require(Transform)]
pub struct Animal;

impl common::RandomSpawnEntityConstructor for Animal {
    type Bundle = (Self, Transform, Mesh3d, MeshMaterial3d<StandardMaterial>);

    fn new(world: &World, position: Vec3) -> Self::Bundle {
        let asset: &Asset = world.get_resource::<Asset>().expect("`animal::Asset` not initialized.");
        let mesh: Handle<Mesh> = asset.mesh.to_owned();
        let material: Handle<StandardMaterial> = asset.material.to_owned();
        {(
            Self,
            position.into(),
            Mesh3d(mesh),
            MeshMaterial3d(material)
        )}
    }
}


// === Asset System ===

// THIS IS TEMPORARY UNTIL PROPS ARE DONE
#[derive(Resource)]
pub struct Asset {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>
}

pub struct AssetSystem;

impl AssetSystem {
    pub fn on_startup(mut commands: Commands, mut meshes: ResMut<::bevy::Asset<Mesh>>, mut materials: ResMut<::bevy::Asset<StandardMaterial>>, mut event_writer: EventWriter<SpawnEvent>) {
        //             Color::linear_rgb(0.0, 1.0, 0.0),
        let material: StandardMaterial = StandardMaterial {
            base_color: (0.0, 1.0, 0.0).into(),
            unlit: true,
            ..default()
        };
        let material: Handle<StandardMaterial> = materials.add(material);
        let mesh: Handle<Mesh> = meshes.add(Sphere::new(SPHERE_RADIUS));
        let asset: Asset = Asset {
            mesh,
            material
        };
        commands.insert_resource(asset);
        let event: SpawnEvent = SpawnEvent {
            position: Vec2::ZERO
        };
        event_writer.write(event);
    }
}


// === Spawn Event System ===

#[derive(Event)]
#[derive(Clone)]
#[derive(Copy)]
pub struct SpawnEvent {
    position: Vec2
}

pub struct SpawnEventSystem;

impl SpawnEventSystem {
    pub fn on_update(mut commands: Commands, mut event_reader: EventReader<SpawnEvent>, asset: Res<Asset>) {
        for event in event_reader.read() {
            let position: Vec3 = event.position.into();
            let mesh: Handle<Mesh> = asset.mesh.to_owned();
            let material: Handle<StandardMaterial> = asset.material.to_owned();
            commands.spawn((
                Animal,
                Tranform::from_translation(position),
                Mesh3d(mesh),
                MeshMaterial3d(material)
            ));
        }
    }
}


// === Motion System ===
// This system is responsible for the animals randomly picking
// a spot, and moving to it.

pub struct MotionLifecycle {
    from: Vec2,
    to: Vec2,
    progress: f32
}

pub enum MotionSensor {
    Shutdown,
    Idle,
    InMotion(MotionLifecycle)
}

#[derive(Component)]
pub struct MotionTracker {
    pub sensor: MotionSensor
}

pub struct MotionSystem {
    x: f32,
    y: f32,
    world_w: f32,
    world_h: f32,
    distance: f32
}

impl MotionSystem {
    pub fn on_update(mut animals: Query<(&mut Transform, &mut MotionTracker)>, time: Res<Time>) {
        let world_w: f32 = WORLD_W;
        let world_h: f32 = WORLD_H;
        let delta: f32 = time.delta_secs();
        let speed: f32 = SPEED;
        let distance: f32 = DISTANCE;
        for (mut transform, mut tracker) in animals.iter_mut() {
            let position: Vec2 = transform.translation.truncate();
            match &mut tracker.sensor {
                MotionSensor::Shutdown => {},
                MotionSensor::Idle => {
                    let x: f32 = position.x;
                    let y: f32 = position.y;
                    let model: Self = Self {
                        x,
                        y,
                        world_w,
                        world_h,
                        distance
                    };
                    let (x, y) = model.simulate();
                    let lifecycle: MotionLifecycle = MotionLifecycle {
                        from: position,
                        to: (x, y).into(),
                        progress: 0.0
                    };
                    tracker.sensor = MotionSensor::InMotion(lifecycle);
                },
                MotionSensor::InMotion(lifecycle) => {
                    *lifecycle += delta * speed;
                    let t: f32 = lifecycle.progress.clamp(0.0, 1.0);
                    let t_eased: f32 = (t * ::std::f32::consts::PI).sin();
                    transform.translation = lifecycle.from.lerp(*lifecycle.to, t_eased).into();
                    if t >= 1.0 {
                        tracker.sensor = MotionSensor::Idle;
                    }
                }
            }
        }
    }
}

impl Model for MotionSystem {
    type Output = (f32, f32);
    
    fn simulate(self) -> Self::Output {
        let angle: f32 = ::fastrand::f32() * ::std::f32::consts::TAU;
        let offset_x: f32 = angle.cos() * self.distance;
        let offset_y: f32 = angle.sin() * self.distance;
        let new_x: f32 = (self.x + offset_x).clamp(0.0, self.world_w);
        let new_y: f32 = (self.y + offset_y).clamp(0.0, self.world_h);
        (new_x, new_y)
    }
}