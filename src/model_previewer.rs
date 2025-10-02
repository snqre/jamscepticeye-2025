use bevy::prelude::*;
use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::render::camera::ScalingMode;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::pbr::{NotShadowCaster, NotShadowReceiver};

// CAMERA
const FOV: f32 = FRAC_PI_2;
const VIEW_WIDTH: f32 = 10.0;
const CAMERA_DISTANCE: f32 = 5.0;
const USE_PERSPECTIVE: bool = true;

// CHECKERS
const SPAWN_PLANE: bool = true;
const SPAWN_GRID: bool = false;
const TILE_Z: f32 = 0.0;
const GRID_Z: f32 = -CAMERA_DISTANCE * 2.0;
const TILING_RADIUS: isize = 4;
const TILE_DIAMETER: f32 = 1.0;
const COLORS: [Color; 2] = [
    Color::linear_rgb(0.8, 0.2, 0.2),
    Color::linear_rgb(0.2, 0.2, 0.8)
];

// LIGHT
const SPAWN_LIGHT: bool = true;
const LIGHT_POS: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const LIGHT_BRIGHTNESS: f32 = 1000.0;
const AMBIENT_LIGHT: f32 = 100.0;

pub struct ModelViewerPlugin;
impl Plugin for ModelViewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Component)]
struct CameraCenter;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // CAMERA
    let camera_center = commands.spawn((
       Transform::default(),
       CameraCenter
    ));
    let camera = commands.spawn((
        Camera3d::default(),
        Camera::default(),
        Transform::from_translation(Vec3::new(0.0, -CAMERA_DISTANCE, CAMERA_DISTANCE))
            .looking_at(Vec3::ZERO, Vec3::Y),
        Tonemapping::AcesFitted,
        Msaa::Sample4,
        Bloom::OLD_SCHOOL
    )).id();
    if USE_PERSPECTIVE {
        commands.entity(camera).insert(
            Projection::Perspective(
                PerspectiveProjection {
                    fov: FOV,
                    ..default()
                }
            )
        );
    } else {
        commands.entity(camera).insert(
            Projection::Orthographic(
                OrthographicProjection {
                    scaling_mode: ScalingMode::FixedHorizontal {viewport_width: VIEW_WIDTH },
                    ..OrthographicProjection::default_3d()
                }
            )
        );
    };

    // PLANE AND GRID
    let tile_mesh = meshes.add(Rectangle::from_length(TILE_DIAMETER));
    let mat_a = materials.add(StandardMaterial::from_color(COLORS[0]));
    let mat_b = materials.add(StandardMaterial::from_color(COLORS[1]));
    if SPAWN_PLANE {
        for xi in -TILING_RADIUS..=TILING_RADIUS {
            let x = xi as f32 * TILE_DIAMETER;
            for yi in -TILING_RADIUS..=TILING_RADIUS {
                let y = yi as f32 * TILE_DIAMETER;
                let mat = if (xi + yi) % 2 == 0 {
                    mat_a.clone()
                } else {
                    mat_b.clone()
                };
                commands.spawn((
                    Transform::from_xyz(x, y, TILE_Z),
                    Mesh3d(tile_mesh.clone()),
                    MeshMaterial3d(mat),
                    NotShadowCaster
                ));
            };
        };
    };
    if SPAWN_GRID {
        for xi in -TILING_RADIUS..=TILING_RADIUS {
            let x = xi as f32 * TILE_DIAMETER;
            for yi in -TILING_RADIUS..=TILING_RADIUS {
                let y = yi as f32 * TILE_DIAMETER;
                let mat = if (xi + yi) % 2 == 0 {
                    mat_a.clone()
                } else {
                    mat_b.clone()
                };
                commands.spawn((
                    Transform::from_xyz(x, y, GRID_Z),
                    Mesh3d(tile_mesh.clone()),
                    MeshMaterial3d(mat),
                    NotShadowCaster,
                    NotShadowReceiver,
                    ChildOf(camera)
                ));
            };
        };
    };

    // LIGHTS
    commands.insert_resource(AmbientLight {color: Color::WHITE, brightness: AMBIENT_LIGHT, ..default()});
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: LIGHT_BRIGHTNESS,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(LIGHT_POS).looking_at(Vec3::ZERO, Vec3::Y)
    ));
}

fn camera_controls(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mouse_scrolling: EventReader<MouseWheel>,
    camera_query: Query<&mut Transform, With<CameraCenter>>
) {
    
}