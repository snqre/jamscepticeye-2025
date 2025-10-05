use super::*;

const ANIMAL_Z: f32 = 0.0;
const ANIMAL_SPHERE_RADIUS: f32 = 0.25;
const ANIMAL_SPEED: f32 = 1.0;
const ANIMAL_DISTANCE: f32 = 10.0;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Component)]
pub enum Lifecycle {
    Alive,

    // When it hits a tree it will go into the corpse state.
    Corpse
}

#[derive(Component)]
pub struct Brain {
    pub timer: Timer,
    pub chasing_tree: bool
}

pub struct AnimalPlugin;

impl Plugin for AnimalPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, Animal::sys_find_next_position);
        // app.add_systems(Update, Animal::sys_goto_to_next_position);
        app.add_systems(Update, Animal::animal_move_system);
        app.add_systems(Update, Animal::seek_death_system);
        app.add_systems(Update, Animal::check_collision_system);
        app.add_event::<AnimalSpawn>();
        app.add_systems(Startup, setup_animal_assets);
        app.add_systems(Update, animal_spawn_reader.run_if(event_exists!(AnimalSpawn)));
    }
}

#[derive(Event, Copy, Clone)]
pub struct AnimalSpawn {
    pos: Vec2
}

fn animal_spawn_reader(
    mut event_reader: EventReader<AnimalSpawn>,
    mut commands: Commands,
    animal_assets: Res<AnimalAssets>
) {
    for event in event_reader.read() {
        commands.spawn((
            Animal::new(event.pos),
            Brain {
                timer: Timer::from_seconds(10.0, TimerMode::Once),
                chasing_tree: false
            },
            Lifecycle::Alive,
            Transform::from_translation(event.pos.extend(ANIMAL_Z)),
            Mesh3d(animal_assets.mesh.clone()),
            MeshMaterial3d(animal_assets.material.clone())
        ));
    }
}

fn setup_animal_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut event_writer: EventWriter<AnimalSpawn>
) {
    let material = materials.add(
        StandardMaterial {
            base_color: Color::linear_rgb(0.0, 1.0, 0.0),
            unlit: true,
            ..default()
        }
    );
    let mesh = meshes.add(Sphere::new(ANIMAL_SPHERE_RADIUS));
    commands.insert_resource(AnimalAssets{mesh, material});
    event_writer.write(AnimalSpawn{pos: Vec2::ZERO});
}

#[derive(Resource)]  // THIS IS TEMPORARY UNTIL PROPS ARE DONE
struct AnimalAssets{
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>
}

#[derive(Component)]
#[require(Transform)]
pub struct Animal {
    pub position: Vec2,
    pub next_position: Vec2,
    pub move_progress: f32
}

impl Animal {
    pub fn new(pos: Vec2) -> Self {
        Self {
            position: pos,
            next_position: pos,
            move_progress: 1.0
        }
    }

    pub fn find_next_position(&mut self, distance: f32) {
        if self.position != self.next_position {
            return
        }
        self.next_position = Self::random_position(distance);
    }

    pub fn goto_next_position(&mut self, dt: f32) {
        if self.position == self.next_position {
            return
        }
        self.move_progress += dt * ANIMAL_SPEED;
        let t: f32 = self.move_progress.clamp(0.0, 1.0);
        let t_eased: f32 = (t * ::std::f32::consts::PI).sin().abs();
        let new_position: Vec2 = self.position.lerp(self.next_position, t_eased);
        if t >= 1.0 {
            self.position = self.next_position;
            self.move_progress = 0.0;
        } else {
            self.position = new_position;
        }
    }

    fn random_position(distance: f32) -> Vec2 {
        let angle: f32 = ::fastrand::f32() * ::std::f32::consts::TAU;
        let cos: f32 = angle.cos();
        let sin: f32 = angle.sin();
        let offset: Vec2 = Vec2::new(cos, sin);
        let offset: Vec2 = offset * distance;
        offset
    }
}

impl Animal {
    pub fn animal_move_system(
        time: Res<Time>,
        animal_query: Query<(&mut Animal, &mut Transform)>
    ) {
        let dt = time.delta_secs();
        for (mut animal, mut transform) in animal_query {
            animal.find_next_position(ANIMAL_DISTANCE);
            animal.goto_next_position(dt);
            transform.translation = animal.position.extend(ANIMAL_Z);
        }
    }

    pub fn sys_find_next_position(mut query: Query<&mut Self>) {
        for mut s in query.iter_mut() {
            s.find_next_position(200.0);
        }
    }

    pub fn sys_goto_to_next_position(mut query: Query<&mut Self>) {
        for mut s in query.iter_mut() {
            s.goto_next_position(0.02);
        }
    }

    // When the time is right, the animal will seek a tree to b-line to
    // and die on.
    pub fn seek_death_system(mut animals: AnimalQuery, tree_tracker: Res<tree::Tracker>, time: Res<Time>) {
        for (mut animal, mut brain, lifecycle) in &mut animals {
            let lifecycle: Lifecycle = *lifecycle;
            if lifecycle != Lifecycle::Alive {
                continue
            }
            let time_delta = time.delta();
            brain.timer.tick(time_delta);
            if brain.chasing_tree {
                continue
            }
            if brain.timer.finished() {
                if let Some(closest) = tree_tracker.positions
                    .iter()
                    .min_by_key(|tree_position| {
                        let distance = animal.position.distance_squared(**tree_position);
                        (distance * 1000.0) as u32
                    }) {
                    animal.next_position = **closest;
                    brain.chasing_tree = true;
                }
            }
        }
    }

    pub fn check_collision_system(mut animals: Query<'a, 'b, (&mut Animal, &mut Brain, &mut Lifecycle)>, tree_tracker: Res<tree::Tracker>) {
        for (animal, brain, mut lifecycle) in &mut animals {
            if !brain.chasing_tree {
                continue
            }
            if tree_tracker.positions.iter().any(|tree_position| {
                animal.position.distance_squared(**tree_position) < 0.1
            }) {
                *lifecycle = Lifecycle::Corpse;
            }
        }
    }    
}

type AnimalQuery<'a, 'b> = Query<'a, 'b, (&mut Animal, &mut Brain, &Lifecycle)>;

// impl ::bevy::prelude::Plugin for Animal {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Update, Self::sys_find_next_position);
//         app.add_systems(Update, Self::sys_goto_to_next_position);
//         app.add_event::<AnimalSpawn>();
//         app.add_systems(Startup, setup_animal_assets);
//         app.add_systems(Update, setup_animal_assets.run_if(event_exists!(AnimalSpawn)));
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    fn random_position() {
        let random_position_0: Vec2 = Animal::random_position(200.0);
        let random_position_1: Vec2 = Animal::random_position(200.0);
        assert_ne!(random_position_0, random_position_1);
    }

    fn goto_next_position() {
        let mut animal: Animal = Animal {
            position: Vec2::ZERO,
            next_position: Vec2::new(100.0, 0.0),
            move_progress: 0.0
        };
        animal.goto_next_position(0.02);

        // Sorry terrible test will need better ones, but 90% chance this works, the math seams to check out.
        assert!(animal.position.x > 0.0);
        assert!(animal.move_progress > 0.0);
    }
}