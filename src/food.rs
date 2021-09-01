use std::collections::HashMap;

use bevy::prelude::{Commands, Entity, Query, QuerySet, Transform, With, Without};

use crate::ant::{Ant, Carrying, State};

pub struct Food;
pub struct Carried(Entity);

pub fn food_pickup(
	mut commands: Commands,
	mut ant: Query<(Entity, &Transform, &mut State), (With<Ant>, Without<Carrying>)>,
	food: Query<(Entity, &Transform), (With<Food>, Without<Carried>)>,
) {
	for (ant, ant_transform, mut state) in ant.iter_mut() {
		for (food, food_transform) in food.iter() {
			if ant_transform
				.translation
				.distance(food_transform.translation)
				< 1.0 && *state == State::SearchFood
			{
				*state = State::FoundFood;
				commands.entity(ant).insert(Carrying(food));
				commands.entity(food).insert(Carried(ant));
			}
		}
	}
}

pub fn food_carried(
	// mut food: Query<(&mut Transform, &Carried), With<Food>>,
	// ant: Query<(&Transform, Entity), With<Ant>>,
	mut commands: Commands,
	mut set: QuerySet<(
		Query<(&mut Transform, &Carried, Entity), With<Food>>,
		Query<(&Transform, Entity), With<Ant>>,
	)>,
) {
	let mut details = HashMap::new();
	for (transform, ant) in set.q1().iter() {
		details.insert(ant, (transform.translation, transform.local_y()));
	}
	for (mut transform, carrier, food) in set.q0_mut().iter_mut() {
		let details = match details.get(&carrier.0) {
			Some(v) => v,
			None => {
				commands.entity(food).despawn();
				continue;
			}
		};
		transform.translation = details.0 + details.1 * 5.0;
	}
}
