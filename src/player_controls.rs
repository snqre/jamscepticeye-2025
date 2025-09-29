use bevy::prelude::*;

use crate::camera_module::POVCameraFollower;

pub const PLAYER_SPEED: f32 = 4.0;
pub const PLAYER_DEFAULT_POS: Vec3 = Vec3::new(0.0, 0.0, PLAYER_Z);
pub const PLAYER_Z: f32 = 0.5;

const SPAWN_PLAYER: bool = true;
const PLAYER_RADIUS: f32 = 0.5;

pub struct PlayerControlsPlugin;
impl Plugin for PlayerControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup);
        app.add_systems(PreUpdate, player_movement);
    }
}

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Resource)]
pub struct PlayerMotion {
    velocity: Vec2,
    translation: Vec2
} impl PlayerMotion {
    fn new() -> Self {
        Self {
            velocity: Vec2::ZERO,
            translation: PLAYER_DEFAULT_POS.xy()
        }
    }
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_motion: ResMut<PlayerMotion>,
    mut transform_query: Query<&mut Transform, With<PlayerMarker>>
) {
    let mut player_transform = if let Ok(t) = transform_query.single_mut() {
        t
    } else {
        return;
    };
    let mut x = 0i8;
    let mut y = 0i8;
    let pressed = keys.get_pressed();
    for &key in pressed {
        match key {
            KeyCode::KeyA => x -= 1,
            KeyCode::KeyD => x += 1,
            KeyCode::KeyW => y += 1,
            KeyCode::KeyS => y -= 1,
            _ => {}
        }
    };
    let player_velocity = if x.abs() + y.abs() > 1 {
        Vec2::new(x as f32, y as f32).normalize() * PLAYER_SPEED
    } else {
        Vec2::new(x as f32, y as f32) * PLAYER_SPEED
    };
    player_transform.translation += (player_velocity * time.delta_secs()).extend(0.0);
    player_motion.velocity = player_velocity;
    player_motion.translation = player_transform.translation.xy();
}

fn player_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.insert_resource(PlayerMotion::new());
    if SPAWN_PLAYER {
        commands.spawn((
            Transform::from_translation(PLAYER_DEFAULT_POS),
            PlayerMarker,
            Mesh3d(meshes.add(Sphere::new(PLAYER_RADIUS))),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::linear_rgb(1.0, 0.0, 0.0)))),
            POVCameraFollower::new(1.0)
        ));
    };
}