use super::*;

#[derive(Component)]
#[require(Transform)]
pub struct Tree;


// === Spawn System ===

pub struct SpawnSystem {
    pub world_w: f32,
    pub world_h: f32,
    pub count: usize,
    pub min_spacing: f32,
    pub max_attempts: usize
}

impl SpawnSystem {
    pub fn on_startup(mut commands: Commands) {
        let model: Self = Self {
            world_w: WORLD_W,
            world_h: WORLD_H,
            count: 200,
            min_spacing: 20.0,
            max_attempts: 256
        };
        for position in model.simulate() {
            let position: Vec3 = position.into();
            commands.spawn((
                Tree,
                Transform::from_translation(position)
            ));
        }
    }
}

impl Model for SpawnSystem {
    type Output = Vec<Vec2>;
    
    fn simulate(self) -> Self::Output {
        let mut positions: Vec<Vec2> = Vec::new();
        for _ in 0..self.max_attempts {
            let x: f32 = ::fastrand::f32() * self.world_w;
            let y: f32 = ::fastrand::f32() * self.world_h;
            let next: Vec2 = (x, y).into();
            if positions.iter().any(|position| {
                position.distance(next) < self.min_spacing
            }) {
                continue
            }
            positions.push(next);
        }
        positions
    }
}


// === Plugin ===

pub struct Plugin;

impl ::bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Tracker>();
        app.add_systems(Startup, SpawnSystem::on_startup);
    }
}