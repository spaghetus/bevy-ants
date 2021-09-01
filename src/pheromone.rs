pub struct Pheromone;
use bevy::prelude::{Commands, Entity, Query, With};

pub use crate::ant::Team;
#[derive(PartialEq, Clone)]
pub struct Strength(pub u16);
#[derive(PartialEq, Clone)]
pub enum Scent {
	ToHive,
	ToFood,
	ToEnemy,
}

pub fn pheromone_decay(
	mut commands: Commands,
	mut pheromone: Query<(Entity, &mut Strength), With<Pheromone>>,
) {
	for (pheromone, mut strength) in pheromone.iter_mut() {
		strength.0 -= 1;
		if strength.0 <= 0 {
			commands.entity(pheromone).despawn();
		}
	}
}
