use std::f32::consts::PI;

use bevy::{
	core::{Time, Timer},
	math::{Quat, Vec2},
	prelude::{
		Assets, Color, Commands, Entity, Query, QuerySet, Res, ResMut, SpriteBundle, Transform,
		With,
	},
	sprite::{ColorMaterial, Sprite},
};
use image::GenericImageView;
use rand::{thread_rng, Rng};

use crate::{
	pheromone::{Pheromone, Scent, Strength},
	stage::{Stage, SOLID_COLOR},
};

pub struct Ant;
pub struct Soldier;
pub struct Worker;
#[derive(PartialEq, Clone)]
pub struct Team(pub u8, pub u8, pub u8);
pub struct Carrying(pub Entity);
pub struct Health(pub u8);
#[derive(PartialEq)]
pub enum State {
	FoundFood,
	FoundEnemy,
	SearchFood,
	SearchEnemy,
}

pub fn ant_wander(time: Res<Time>, mut ant: Query<&mut Transform, With<Ant>>) {
	let mut rng = thread_rng();
	let delta = f32::min(time.delta_seconds(), 0.1);
	for mut transform in ant.iter_mut() {
		transform.rotate(Quat::from_rotation_z(rng.gen_range(-3.0..3.0) * delta));
	}
}

pub fn ant_move(time: Res<Time>, mut ant: Query<&mut Transform, With<Ant>>) {
	for mut transform in ant.iter_mut() {
		let movement = transform.local_y() * f32::min(time.delta_seconds(), 0.075) * 100.0;
		transform.translation += movement;
	}
}

pub fn ant_collide(
	mut commands: Commands,
	mut ant: Query<(Entity, &mut Transform), With<Ant>>,
	stage: Res<Stage>,
) {
	for (ant_entity, mut transform) in ant.iter_mut() {
		fn check_intersecting(transform: &Transform, stage: &Stage) -> bool {
			let (x, y) = (transform.translation.x, transform.translation.y);
			let x = x + stage.width as f32 / 2.0;
			let y = y + stage.height as f32 / 2.0;
			let y = stage.height as f32 - y;
			x < 0.0
				|| x >= stage.width as f32
				|| y < 0.0 || y >= stage.height as f32
				|| stage.image.get_pixel(x as u32, y as u32).0 == SOLID_COLOR
		}
		if check_intersecting(&transform, &stage) {
			let movement = transform.local_y();
			let mut attempts = 0;
			while check_intersecting(&transform, &stage) && attempts < 100 {
				transform.translation -= movement;
				attempts += 1;
			}
			transform.rotate(Quat::from_rotation_z(PI));
			if attempts >= 100 {
				println!("{:?} became stuck; removing.", ant_entity);
				commands.entity(ant_entity).despawn()
			}
		}
	}
}

pub struct PheromoneTimer(pub Timer);

pub fn ant_emit_pheromone(
	mut commands: Commands,
	time: Res<Time>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut ant: Query<(Entity, &Transform, &Team, &mut PheromoneTimer, &State), With<Ant>>,
) {
	for (_, transform, team, mut timer, state) in ant.iter_mut() {
		if timer.0.tick(time.delta()).just_finished() {
			let scent = match state {
				State::FoundFood => Scent::ToFood,
				State::FoundEnemy => Scent::ToEnemy,
				State::SearchFood => Scent::ToHive,
				State::SearchEnemy => Scent::ToHive,
			};
			commands
				.spawn_bundle(SpriteBundle {
					transform: Transform::from_translation(transform.translation),
					sprite: Sprite::new(Vec2::new(2.0, 2.0)),
					material: materials.add(ColorMaterial::color(match scent {
						Scent::ToHive => Color::rgb_u8(128, 0, 0),
						Scent::ToFood => Color::rgb_u8(0, 128, 0),
						Scent::ToEnemy => Color::rgb_u8(0, 0, 128),
					})),
					..Default::default()
				})
				.insert(Pheromone)
				.insert(Strength(128))
				.insert(scent)
				.insert(Team(team.0, team.1, team.2));
		}
	}
}

pub fn ant_seek_pheromone(
	time: Res<Time>,
	// mut ant: Query<(&mut Transform, &Team, &State), With<Ant>>,
	// pheromones: Query<(&Transform, &Strength, &Team, &Scent), With<Pheromone>>,
	mut set: QuerySet<(
		Query<(&mut Transform, &Team, &State), With<Ant>>,
		Query<(&Transform, &Strength, &Team, &Scent), With<Pheromone>>,
	)>,
) {
	// Copy pheromone details
	let pheromones: Vec<(Transform, Strength, Team, Scent)> = set
		.q1()
		.iter()
		.map(|v| (*v.0, v.1.clone(), v.2.clone(), v.3.clone()))
		.collect();
	// For each ant...
	for (mut transform, team, state) in set.q0_mut().iter_mut() {
		// Determine which pheromone type it is looking for.
		let sought = match state {
			State::FoundFood => Scent::ToHive,
			State::FoundEnemy => Scent::ToHive,
			State::SearchFood => Scent::ToFood,
			State::SearchEnemy => Scent::ToEnemy,
		};
		// Record our forward vector
		let forward = transform.local_y();
		// Create a list of influences
		let mut influences: Vec<f32> = vec![];
		for (ph_transform, strength, ph_team, scent) in pheromones.iter() {
			if ph_team == team
				&& sought == *scent
				&& ph_transform.translation.distance(transform.translation) < 50.0
			{
				let ph_pos = ph_transform.translation - transform.translation;
				let angle = forward.angle_between(ph_pos);
				if angle.abs() < PI / 4.0 {
					influences.push(angle * (strength.0 as f32 / 16.0) * -1.0);
				}
			}
		}
		let influences = influences.iter().sum::<f32>() / (influences.len() as f32).max(1.0);
		transform.rotate(Quat::from_rotation_z(
			influences * f32::min(time.delta_seconds(), 0.1),
		))
	}
}
