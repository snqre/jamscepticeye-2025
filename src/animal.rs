use super::*;

#[derive(Component)]
pub struct Animal {
    pub position: Vec2,
    pub next_position: Vec2,
    pub move_progress: f32
}

impl Animal {
    pub fn spawn(mut commands: Commands) {
        commands.spawn((
            Transform::from((0.0, 0.0, 0.0))
        ));
    }

    pub fn find_next_position(&mut self, distance: f32) {
        if self.position != self.next_position {
            return
        }
        self.next_position = Self::random_position(distance);
    }

    pub fn goto_next_position(&mut self) {
        if self.position == self.next_position {
            return
        }
        self.move_progress = self.move_progress + 0.02;
        let t: f32 = self.move_progress.clamp(0.0, 1.0);
        let t_eased: f32 = (t * ::std::f32::consts::PI).sin().abs();
        let new_position: Vec2 = self.position.lerp(self.next_position, t_eased);
        if t >= 1.0 {
            self.position = self.next_position;
            self.move_progress = 0.0;
        } else {
            self.position = new_position;
        }
    }

    fn random_position(distance: f32) -> Vec2 {
        let angle: f32 = ::fastrand::f32() * ::std::f32::consts::TAU;
        let cos: f32 = angle.cos();
        let sin: f32 = angle.sin();
        let offset: Vec2 = Vec2::new(cos, sin);
        let offset: Vec2 = offset * distance;
        offset
    }
}

impl Animal {
    pub fn sys_find_next_position(query: Query<&mut Self>) {
        for s in query.iter() {
            s.find_next_position(200.0);
        }
    }

    pub fn sys_goto_to_next_position(query: Query<&mut Self>) {
        for s in query.iter() {
            s.goto_next_position();
        }
    }
}

impl ::bevy::prelude::Plugin for Animal {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, Self::sys_find_next_position)
            .add_systems(Update, Self::sys_goto_to_next_position);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn random_position() {
        let random_position_0: Vec2 = Animal::random_position(200.0);
        let random_position_1: Vec2 = Animal::random_position(200.0);
        assert_ne!(random_position_0, random_position_1);
    }

    fn goto_next_position() {
        let mut animal: Animal = Animal {
            position: Vec2::ZERO,
            next_position: Vec2::new(100.0, 0.0),
            move_progress: 0.0
        };
        animal.goto_next_position();

        // Sorry terrible test will need better ones, but 90% chance this works, the math seams to check out.
        assert!(animal.position.x > 0.0);
        assert!(animal.move_progress > 0.0);
    }
}