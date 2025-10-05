use super::*;

/// Trees spawn just outside of this radius to create an illution of the
/// trees already existing.
const PLAYER_VIEW_RADIUS: f32 = 10.0;

const SPAWN_DISTANCE: f32 = 12.0;

/// The maximum number of trees to attempt to spawn per system call.
/// Not all attempts result in a successful spawn due to spacing checks.
const COUNT: usize = 8;

const MIN_SPACING: f32 = 2.0;

/// Track all trees to avoid collision but also to be able to check for
/// any nearby trees from the animal system and go ahead to bonk on it
/// and die.
#[derive(Default)]
#[derive(Resource)]
pub struct Tracker {
    pub positions: Vec<Vec2>
}

#[derive(Component)]
#[require(Transform)]
pub struct Tree;

impl Tree {

    pub fn startup_spawn_system(mut commands: Commands, mut tracker: ResMut<Tracker>, player_motion: Res<player_controls::PlayerMotion>) {
        Self::spawn(&mut commands, &mut tracker, player_motion.translation);
    }

    // Called every update frame ... sorry, there's a better way to optimize this
    // but we won't have time to get to all the other important systems if we
    // write good code here. If its lagging the game, will fix.
    pub fn spawn_system(mut commands: Commands, mut tracker: ResMut<Tracker>, player_motion: Res<player_controls::PlayerMotion>) {
        Self::spawn(&mut commands, &mut tracker, player_motion.translation);
    }

    fn spawn(commands: &mut Commands, tracker: &mut ResMut<Tracker>, center: Vec2) {
        let mut new_positions: Vec<Vec2> = Vec::new();
        for _ in 0..COUNT {
            let angle: f32 = ::fastrand::f32() * ::std::f32::consts::TAU;
            let angle_cos: f32 = angle.cos();
            let angle_sin: f32 = angle.sin();
            let offset: Vec2 = Vec2::new(angle_cos, angle_sin) * SPAWN_DISTANCE;
            let position: Vec2 = center + offset;
            if tracker.positions.iter().any(|existing| {
                let existing: Vec2 = *existing;
                existing.distance(position) < MIN_SPACING
            }) {
                continue
            }
            let position_as_vec3: Vec3 = position.extend(0.0);
            commands.spawn((
                Self,
                Transform::from_translation(position_as_vec3)
            ));
            new_positions.push(position);
        }
        tracker.positions.extend(new_positions);
    }
}

pub struct Plugin;

impl ::bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Tracker>()
            .add_systems(Startup, Tree::startup_spawn_system)
            .add_systems(Update, Tree::spawn_system);
    }
}