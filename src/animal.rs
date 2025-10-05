use super::*;

pub const SPHERE_RADIUS: f32 = 0.25;
pub const Z: f32 = 0.0;
pub const SPEED: f32 = 1.0;
pub const DISTANCE: f32 = 10.0;
pub const MIN_MOTION_SLEEP_SECONDS: f32 = 10.0;
pub const MAX_MOTION_SLEEP_SECONDS: f32 = 30.0;
pub const MIN_DEATH_TIMEOUT_SECONDS: f32 = 60.0;
pub const MAX_DEATH_TIMEOUT_SECONDS: f32 = 120.0;

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
pub struct Animal;

impl common::RandomSpawnEntityConstructor for Animal {
    type Bundle = (
        Self, 
        Transform, 
        MotionTracker,
        Mesh3d, 
        MeshMaterial3d<StandardMaterial>
    );

    fn new(world: &World, position: Vec3) -> Self::Bundle {
        let asset: &Asset = world.get_resource::<Asset>().expect("`animal::Asset` not initialized.");
        let mesh: Handle<Mesh> = asset.mesh.to_owned();
        let material: Handle<StandardMaterial> = asset.material.to_owned();
        {(
            Self,
            position.into(),
            MotionTracker::default(),
            Mesh3d(mesh),
            MeshMaterial3d(material)
        )}
    }
}


// === Asset System ===

// THIS IS TEMPORARY UNTIL PROPS ARE DONE
#[derive(Resource)]
pub struct Asset {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>
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
    pub position: Vec2
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
                Transform::from_translation(position),
                Mesh3d(mesh),
                MeshMaterial3d(material)
            ));
        }
    }
}


// === Motion System ===
// This system is responsible for the animals randomly picking
// a spot, and moving to it.

pub struct InMotionLifecycle {
    pub from: Vec2,
    pub to: Vec2,
    pub progress: f32
}

#[derive(Default)]
pub enum MotionLifecycle {
    #[default]
    Idle,
    InMotion(InMotionLifecycle),
    Shutdown,
}

#[derive(Component)]
#[derive(Default)]
pub struct MotionTracker {
    pub lifecycle: MotionLifecycle,
    pub cooldown: f32
}

impl MotionTracker {
    pub fn random_cooldown() -> f32 {
        MIN_MOTION_SLEEP_SECONDS + fastrand::f32() * (MAX_MOTION_SLEEP_SECONDS - MIN_MOTION_SLEEP_SECONDS)
    }
}

impl Default for MotionTracker {
    fn default() -> Self {
        Self {
            lifecycle: MotionLifecycle::default(),
            cooldown: Self::random_cooldown()
        }
    }
}

pub struct MotionSystem {
    x: f32,
    y: f32,
    world_w: f32,
    world_h: f32,
    distance: f32
}

impl MotionSystem {
    pub fn on_update(mut animals: Query<(&mut Transform, &mut MotionTracker, &mut Animal)>, time: Res<Time>) {
        let world_w: f32 = WORLD_W;
        let world_h: f32 = WORLD_H;
        let delta: f32 = time.delta_secs();
        let speed: f32 = SPEED;
        let distance: f32 = DISTANCE;
        for (mut transform, mut tracker, _) in animals.iter_mut() {
            let position: Vec2 = transform.translation.truncate();
            match &mut tracker.lifecycle {
                MotionLifecycle::Shutdown => {},
                MotionLifecycle::Idle => {
                    if tracker.cooldown > 0.0 {
                        tracker.cooldown = (tracker.cooldown - delta).max(0.0);
                        continue
                    }
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
                    let lifecycle: InMotionLifecycle = InMotionLifecycle {
                        from: position,
                        to: (x, y).into(),
                        progress: 0.0
                    };
                    tracker.lifecycle = MotionLifecycle::InMotion(lifecycle);
                },
                MotionLifecycle::InMotion(lifecycle) => {
                    lifecycle.progress += delta * speed;
                    let t: f32 = lifecycle.progress.clamp(0.0, 1.0);
                    let t_eased: f32 = (t * ::std::f32::consts::PI).sin();
                    transform.translation = lifecycle.from.lerp(*lifecycle.to, t_eased).into();
                    if t >= 1.0 {
                        tracker.lifecycle = MotionLifecycle::Idle;
                        tracker.cooldown = MotionTracker::random_cooldown();
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


// === Death System ===
// NOTE After timeout the animal will stop motion and ram directly into the
//      nearest tree.

pub struct DeathRammingLifecycle {
    pub from: Vec2,
    pub to: Vec2
}

#[derive(Default)]
pub enum DeathLifecycle {
    #[default]
    Alive,
    Ramming(DeathRammingLifecycle),
    Corpse
}

#[derive(Component)]
pub struct DeathTracker {
    pub lifecycle: DeathLifecycle,
    pub timeout: f32
}

impl DeathTracker {
    pub fn random_timeout() -> f32 {
        MIN_DEATH_TIMEOUT_SECONDS + ::fastrand::f32() * (MAX_DEATH_TIMEOUT_SECONDS - MIN_DEATH_TIMEOUT_SECONDS)
    }
}

impl Default for DeathTracker {
    fn default() -> Self {
        Self {
            lifecycle: DeathLifecycle::default(),
            timeout: Self::random_timeout()
        }
    }
}

pub struct DeathSystem {
    x: f32,
    y: f32,
    tree_positions: Vec<(f32, f32)>
}

impl DeathSystem {
    pub fn on_update(mut animals: Query<(&mut Transform, &mut DeathTracker, &mut Animal)>, mut trees: Query<(&mut Transform, &mut tree::Tree)>, time: Res<Time>) {
        let delta: f32 = time.delta_secs();
        let tree_positions: Vec<(f32, f32)> = trees
            .iter()
            .map(|(transform, _)| {
                let position: Vec2 = transform.translation.truncate();
                let x: f32 = position.x;
                let y: f32 = position.y;
                (x, y)
            })
            .collect();
        for (mut transform, mut tracker, _) in animals.iter_mut() {
            let position: Vec2 = transform.translation.truncate();
            match &mut tracker.lifecycle {
                DeathLifecycle::Corpse => {},
                DeathLifecycle::Alive => {
                    tracker.timeout -= delta;
                    if tracker.timeout > 0.0 {
                        tracker.lifecycle = DeathLifecycle::Corpse;
                        continue
                    }
                    let x: f32 = position.x;
                    let y: f32 = position.y;
                    let model: Self = Self {
                        x,
                        y,
                        tree_positions: tree_positions.to_owned()
                    };
                    let target: Vec2 = model.simulate();
                    let lifecycle: DeathRammingLifecycle = DeathRammingLifecycle {
                        from: position,
                        to: target
                    };
                    tracker.lifecycle = DeathLifecycle::Ramming(lifecycle);
                },
                DeathLifecycle::Ramming(ramming_lifecycle) => {
                    
                }
            }
        }
    }
}

impl Model for DeathSystem {
    type Output = Vec2;

    // Determines the closest tree to ram into.
    fn simulate(self) -> Self::Output {
        let position: Vec2 = (self.x, self.y).into();
        let tree_positions: Vec<Vec2> = self.tree_positions
            .iter()
            .map(|(x, y)| {
                (x, y).into()
            })
            .collect();
        tree_positions
            .into_iter()
            .min_by(|x, y| {
                let dx: f32 = x.distance_squared(position);
                let dy: f32 = y.distance_squared(position);
                dx.partial_cmp(&dy).unwrap_or(::std::cmp::Ordering::Equal);
            })
            .unwrap_or(position)
    }
}