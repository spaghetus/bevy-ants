use bevy::{
	math::{Vec2, Vec3},
	prelude::*,
	sprite::Sprite,
};
use rand::{thread_rng, Rng};

pub use crate::ant::Team;
use crate::ant::{Ant, Carrying, Health, PheromoneTimer, State, Worker};
pub struct Anthill;
pub struct HasFood(pub usize);
pub struct NeedsSoldier(bool);

impl Anthill {
	pub fn new(team: Team, position: (i32, i32)) -> (Anthill, Team, Transform) {
		(
			Anthill,
			team,
			Transform::from_translation(Vec3::new(position.0 as f32, -(position.1 as f32), 0.0)),
		)
	}
}

pub fn make_ants(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut hill: Query<(&mut HasFood, &Team, &Transform), With<Anthill>>,
) {
	for (mut food, team, transform) in hill.iter_mut() {
		if food.0 == 0 {
			continue;
		}
		let mut rng = thread_rng();
		while food.0 > 0 {
			food.0 -= 1;
			let ant = commands
				.spawn_bundle(SpriteBundle {
					transform: {
						let mut transform = Transform::from_translation(transform.translation);
						transform.rotate(Quat::from_rotation_z(rng.gen()));
						transform
					},
					sprite: Sprite::new(Vec2::new(2.0, 5.0)),
					material: materials
						.add(ColorMaterial::color(Color::rgb_u8(team.0, team.1, team.2))),
					..Default::default()
				})
				.insert(Ant)
				.insert(Team(team.0, team.1, team.2))
				.insert(Health(255))
				.insert(PheromoneTimer(Timer::from_seconds(0.1, true)))
				.id();
			commands
				.entity(ant)
				.insert(Worker)
				.insert(State::SearchFood);
		}
	}
}

pub fn take_food(
	mut commands: Commands,
	mut hill: Query<(&mut HasFood, &Team, &Transform), With<Anthill>>,
	mut ant: Query<(Entity, &Team, &mut Carrying, &Transform, &mut State), With<Worker>>,
) {
	for (mut food, team, transform) in hill.iter_mut() {
		for (ant, ant_team, carrying, ant_transform, mut state) in ant.iter_mut() {
			if team == ant_team {
				let distance = transform.translation.distance(ant_transform.translation);
				if distance < 20.0 {
					food.0 += 1;
					commands.entity(carrying.0).despawn();
					commands.entity(ant).remove::<Carrying>();
					*state = State::SearchFood;
				}
			}
		}
	}
}
