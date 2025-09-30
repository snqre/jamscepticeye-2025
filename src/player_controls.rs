use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::camera_module::{POVCameraFollower, CameraPos, VIEW_WIDTH};

pub const PLAYER_SPEED: f32 = 4.0;
pub const PLAYER_DEFAULT_POS: Vec3 = Vec3::new(0.0, 0.0, PLAYER_Z);
pub const PLAYER_Z: f32 = 0.5;

const SPAWN_PLAYER: bool = true;  // this specifies if the player-test-sphere is spawned
const PLAYER_RADIUS: f32 = 0.5;
const SPAWN_MOUSE_FRIEND: bool = true;
const FRIEND_RADIUS: f32 = 0.125;

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Component)]
pub struct FollowsMouse;

#[derive(Resource)]
pub struct PlayerMotion {
    pub velocity: Vec2,
    pub translation: Vec2
} impl PlayerMotion {
    fn new() -> Self {
        Self {
            velocity: Vec2::ZERO,
            translation: PLAYER_DEFAULT_POS.xy()
        }
    }
}

#[derive(Resource)]
pub struct MousePos {
    pub translation2d: Vec2
}

pub struct PlayerControlsPlugin;
impl Plugin for PlayerControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup);
        app.add_systems(PreUpdate, player_movement);
        app.add_systems(PreUpdate, mouse_translocator);
        app.add_systems(PreUpdate, update_mouse_followers.after(mouse_translocator));
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
    commands.insert_resource(MousePos{translation2d: Vec2::ZERO});
    if SPAWN_PLAYER {
        commands.spawn((
            Transform::from_translation(PLAYER_DEFAULT_POS),
            PlayerMarker,
            Mesh3d(meshes.add(Sphere::new(PLAYER_RADIUS))),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::linear_rgb(1.0, 0.0, 0.0)))),
            POVCameraFollower::new(1.0)
        ));
    };
    if SPAWN_MOUSE_FRIEND {
        commands.spawn((
            Transform::from_translation(Vec3::ZERO),
            Mesh3d(meshes.add(Sphere::new(FRIEND_RADIUS))),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::linear_rgb(0.0, 0.0, 1.0)))),
            POVCameraFollower::new(1.0),
            FollowsMouse
        ));
    };
}

fn mouse_translocator(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_pos: Res<CameraPos>,
    mut mouse_pos: ResMut<MousePos>
) {
    let result = window_query.single();
    let window = if let Ok(w) = result {
        w
    } else {
        return;
    };
    let window_size = window.size();
    let ratio = window_size.x / window_size.y;
    let mut ndc = if let Some(p) = window.cursor_position() {
        (p / window_size) - 0.5
    } else {
        Vec2::ZERO
    };
    ndc.y /= -ratio;
    mouse_pos.translation2d = (ndc * VIEW_WIDTH) + camera_pos.vec2;
}

fn update_mouse_followers(
    mouse_pos: Res<MousePos>,
    follower_query: Query<&mut Transform, With<FollowsMouse>>
) {
    for mut transform in follower_query {
        let z = transform.translation.z;
        transform.translation = mouse_pos.translation2d.extend(z);
    };
}
