#[macro_export]
macro_rules! event_exists {
    ($event:ty) => {
        |reader: bevy::ecs::event::EventReader<$event>| !reader.is_empty()
    };
}