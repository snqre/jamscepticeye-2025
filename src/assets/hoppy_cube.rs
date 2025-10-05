use bevy::prelude::*;
use std::f32::consts::PI;
use crate::assets::{AssetsAssets, MotionTracker, PropPartMarker};

const SPIN: f32 = -PI;
const Z: f32 = 0.5;
const BOUNCE_SPEED: f32 = 8.0;
const HOP_HEIGHT: f32 = 1.0;

pub fn spawn(
    commands: &mut Commands,
    parent: Entity,
    aa: &Res<AssetsAssets>
) {
    let entity = commands.spawn((
        Mesh3d(aa.hoppy_mesh.clone()),
        MeshMaterial3d(aa.hoppy_material.clone()),
        PropPartMarker,
        ChildOf(parent),
    )).id();
    commands.entity(parent).insert(
        (
            MotionTracker::new(vec![entity]),
            HoppyAnimation
        )
    );
}

pub fn generate_assets(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) -> (Handle<Mesh>, Handle<StandardMaterial>) {
    let material = materials.add(
        StandardMaterial {
            base_color: Color::linear_rgb(1.0, 0.0, 0.0),
            metallic: 0.5,
            perceptual_roughness: 0.5,
            ..default()
        }
    );
    let mesh = meshes.add(Cuboid::from_length(1.0));
    (mesh, material)
}

#[derive(Component)]
pub struct HoppyAnimation;

pub fn animate(
    motion_query: Query<&MotionTracker, With<HoppyAnimation>>,
    mut part_transforms: Query<&mut Transform, With<PropPartMarker>>
) {
    for motion_tracker in motion_query {
        let mut transform = part_transforms.get_mut(motion_tracker.prop_parts[0]).unwrap();
        if motion_tracker.moving {
            transform.translation.z = Z + (motion_tracker.time * BOUNCE_SPEED).sin().abs() * HOP_HEIGHT;
            if motion_tracker.speed > 1.0 {
                transform.rotate_z(motion_tracker.dt * SPIN * motion_tracker.speed);
            } else {
                transform.rotate_z(motion_tracker.dt * SPIN);
            };
        } else {
            transform.translation.z = Z;
        }
    }
}