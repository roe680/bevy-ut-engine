use bevy::ecs::bundle::Bundle;

pub fn spawn_vecs<T: Bundle>(
    start: f32,
    interval: f32,
    count: usize,
    mut func: impl FnMut(f32) -> T,
) -> Vec<T> {
    (0..count)
        .map(|i| func(start + interval * i as f32))
        .collect()
}
