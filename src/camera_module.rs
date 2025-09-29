use bevy::prelude::*;
use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::render::camera::ScalingMode;
use bevy::transform::systems::propagate_parent_transforms;

pub const VIEW_WIDTH: f32 = 10.0;
pub const CAMERA_Z_OFFSET: f32 = 0.5;
const TRACKING: f32 = 4.0;  // MORE IS FASTER
const CAMERA_RELATIVE: Vec3 = Vec3::new(0.0, 0.0, VIEW_WIDTH);

#[derive(Component, Default, Copy, Clone)]
#[require(CameraCommonComponent)]
pub struct POVCameraFollower{
    pub weight: f32
} impl POVCameraFollower {
    pub fn new(weight: f32) -> Self {
        Self {
            weight
        }
    }
    pub fn default() -> Self {
        Self::new(1.0)
    }
}

pub struct TopDownCameraPlugin;
impl Plugin for TopDownCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(PostUpdate, move_camera.before(propagate_parent_transforms));
    }
}

#[derive(Component)]
struct POVCamera;

#[derive(Component)]
#[require(CameraCommonComponent)]
struct POVCameraLeader;

#[derive(Component)]
struct CameraCommonComponent;
impl Default for CameraCommonComponent {
    fn default() -> Self {
        Self
    }
}

fn spawn_camera(
    mut commands: Commands
) {
    let camera_leader = commands.spawn((
        Transform::default(),
        POVCameraLeader
    )).id();
    commands.spawn((
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Projection::Orthographic(
            OrthographicProjection {
                scaling_mode: ScalingMode::FixedHorizontal {viewport_width: VIEW_WIDTH},
                ..OrthographicProjection::default_3d()
            }
        ),
        Transform::from_translation(CAMERA_RELATIVE)
            .looking_at(Vec3::new(0.0, 0.0, CAMERA_Z_OFFSET), Vec3::Y),
        Bloom::OLD_SCHOOL,
        Tonemapping::AcesFitted,
        Msaa::Sample4,
        POVCamera,
        ChildOf(camera_leader)
    ));
}

fn move_camera(
    mut transform_query: Query<&mut Transform, With<CameraCommonComponent>>,
    leader_query: Query<Entity, With<POVCameraLeader>>,
    follower_query: Query<(Entity, &POVCameraFollower)>,
    time: Res<Time>
) {
    let leader_entity = if let Ok(e) = leader_query.single() {
        e
    } else {
        return;
    };
    let leader_translation = if let Ok(t) = transform_query.get(leader_entity) {
        t.translation
    } else {
        return;
    };

    let mut translation_sum = Vec3::ZERO;
    let mut weight_sum = 0.0f32;
    for (entity, follower) in follower_query {
        if let Ok(t) = transform_query.get_mut(entity) {
            translation_sum += t.translation * follower.weight;
            weight_sum += follower.weight;
        } else {
            continue;
        };
    };
    if weight_sum <= 0.0 {
        return;
    };

    let camera_goal = translation_sum / weight_sum;
    let delta = camera_goal - leader_translation;
    let new_translation = delta * (TRACKING * time.delta_secs()) + leader_translation;
    if let Ok(mut t) = transform_query.get_mut(leader_entity) {
        t.translation = new_translation;
    };
}