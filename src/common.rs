use super::*;

// === Random Spawn System ===
// NOTE: Generic system that randomly spawns `T: Bundle` across
//       a world with a predefined `w: f32` and `h: f32`.

pub trait RandomSpawnEntityConstructor {
    type Bundle: Bundle;

    fn new(world: &World, position: Vec3) -> Self::Bundle;
}

pub struct RandomSpawnSystem<T> 
where
    T: RandomSpawnEntityConstructor {
    pub world_w: f32,
    pub world_h: f32,
    pub count: usize,
    pub min_spacing: f32,
    pub max_attempt: usize
}

impl<T> RandomSpawnSystem<T>
where
    T: RandomSpawnEntityConstructor {
    pub fn on_startup(world: &mut World) {
        let model: Self = Self {
            world_w: WORLD_W,
            world_h: WORLD_H,
            count: 200,
            min_spacing: 20.0,
            max_attempt: 256
        };
        for position in model.simulate() {
            let position: Vec3 = position.into();
            world.spawn(T::new(world, position));
        }
    }
}

impl<T> Model for RandomSpawnSystem<T>
where
    T: RandomSpawnEntityConstructor {
    type Output = Vec<Vec2>;
    
    fn simulate(self) -> Self::Output {
        let mut positions: Vec<Vec2> = Vec::new();
        for _ in 0..self.max_attempt {
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