use std::fmt::Debug;
use ::bevy::prelude::*;
use bevy::pbr::NotShadowCaster;
use fastrand::Rng;

const TILE_DIAMETER: f32 = 1.0;
const TILING_RADIUS: isize = 4;
pub const TILE_Z: f32 = 0.0;

pub struct DebugScenePlugin;
impl Plugin for DebugScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, checker_spawn);
        app.add_systems(Startup, foresting_algorithm);
    }
}

fn checker_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let tile_mesh = meshes.add(Rectangle::from_length(TILE_DIAMETER));
    let white = materials.add(StandardMaterial::from_color(Color::WHITE));
    let black = materials.add(StandardMaterial::from_color(Color::BLACK));
    for xi in -TILING_RADIUS..=TILING_RADIUS {
        let x = xi as f32 * TILE_DIAMETER;
        for yi in -TILING_RADIUS..=TILING_RADIUS {
            let y = yi as f32 * TILE_DIAMETER;
            let mat = if (xi + yi) % 2 == 0 {
                white.clone()
            } else {
                black.clone()
            };
            commands.spawn((
                Transform::from_xyz(x, y, TILE_Z),
                Mesh3d(tile_mesh.clone()),
                MeshMaterial3d(mat),
                NotShadowCaster
            ));
        }
    }
}

fn foresting_algorithm(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let stone_mesh = meshes.add(Sphere::new(0.125));
    let stone_material = materials.add(StandardMaterial {
        base_color: Color::linear_rgb(0.3, 0.3, 0.3),
        ..default()
    });
    let mut rng = Rng::new();
    let spacing = 0.5f32;
    let max = TILING_RADIUS as f32 * TILE_DIAMETER;
    let mut x = -max;
    let mut y = -max;
    while x < max {
        while y < max {
            let (r1, r2, r3) = (rng.f32() - 0.5, rng.f32() - 0.5, rng.f32() + 0.5);
            commands.spawn((
                Mesh3d(stone_mesh.clone()),
                MeshMaterial3d(stone_material.clone()),
                Transform::from_xyz(x + r1, y + r2, 0.0).with_scale(Vec3::ONE * r3)
            ));
            y+=spacing;
        }
        x+=spacing;
        y = -max;
    }
}