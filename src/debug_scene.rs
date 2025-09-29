use ::bevy::prelude::*;
use bevy::pbr::NotShadowCaster;

const TILE_DIAMETER: f32 = 1.0;
const TILING_RADIUS: isize = 4;
pub const TILE_Z: f32 = 0.0;

pub struct DebugScenePlugin;
impl Plugin for DebugScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, checker_spawn);
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
