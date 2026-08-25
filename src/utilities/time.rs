use bevy::prelude::*;
use std::ops::{Deref, DerefMut};
//EntityTimerがSpawDelayを超えたら、LifeTimerをつける。EntityTimeがLifeDuration + SpawnDelayを超えたら、entityを削除する。
// Total lifetime of the bone
#[derive(Component, Default, Clone, Debug)]
pub struct LifeTimer(pub f32);

// Time the bone has been alive
#[derive(Component, Default, Clone, Debug)]
#[require(EntityTimer)]
pub struct LiveDuration(pub f32);

/// Delay before spawning the bone
#[derive(Component, Default, Clone, Debug)]
#[require(EntityTimer, Visibility::Hidden)]
pub struct SpawnDelay(pub f32);

/// Timer for bone lifecycle
#[derive(Component, Default, Clone, Debug)]
pub struct EntityTimer(pub f32);

pub fn spawn_entity_on_timer(
    mut cmds: Commands,
    mut query: Query<(Entity, &SpawnDelay, &EntityTimer), Without<LifeTimer>>,
) {
    for (entity, spawn_delay, entity_timer) in query.iter_mut() {
        if entity_timer.0 >= spawn_delay.0 {
            cmds.entity(entity)
                .insert((LifeTimer::default(), Visibility::Visible));
        }
    }
}

pub fn add_time(
    time: Res<Time>,
    mut entity: Query<&mut EntityTimer>,
    mut life: Query<&mut LifeTimer>,
) {
    for mut entity_timer in entity.iter_mut() {
        **entity_timer += time.delta().as_secs_f32();
    }
    for mut life_timer in life.iter_mut() {
        **life_timer += time.delta().as_secs_f32();
    }
}

pub fn remove_entity_on_timer(
    mut commands: Commands,
    a: Query<(Entity, &LiveDuration, &SpawnDelay, &EntityTimer)>,
    b: Query<(Entity, &LiveDuration, &EntityTimer), Without<SpawnDelay>>,
) {
    for (entity, live_duration, spawn_delay, entity_timer) in a.iter() {
        if live_duration.0 + spawn_delay.0 < entity_timer.0 {
            commands.entity(entity).despawn();
        }
    }
    for (entity, live_duration, entity_timer) in b.iter() {
        if live_duration.0 < entity_timer.0 {
            commands.entity(entity).despawn();
        }
    }
}

impl Deref for LifeTimer {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LifeTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for LiveDuration {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LiveDuration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for SpawnDelay {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SpawnDelay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for EntityTimer {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EntityTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
