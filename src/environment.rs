use ::bevy::prelude::*;

const LIGHT_INTENSITY: f32 = 10_000.0;
const AMBIENT_INTENSITY: f32 = 100.0;  // darken this value for darker shadows
const LIGHT_TRANSLATION: Vec3 = Vec3::new(1.0, 1.0, 2.0);  // light looks at 0,0,0 so this controls its angle easily

pub struct EnvironmentPlugin;
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_lights);
    }
}

fn spawn_lights(
    mut commands: Commands
) {
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: LIGHT_INTENSITY,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(LIGHT_TRANSLATION).looking_at(Vec3::ZERO, Vec3::Z)
    ));
    commands.insert_resource(AmbientLight{
        color: Color::WHITE,
        brightness: AMBIENT_INTENSITY,
        ..default()
    });
}