// Bevy Queries almost always set off this lint.
#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use stage::setup_stage;

pub mod ant;
pub mod anthill;
pub mod food;
pub mod pheromone;
pub mod png;
pub mod stage;

fn main() {
	App::build()
		.add_plugins(DefaultPlugins)
		.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
		.insert_resource(WindowDescriptor {
			title: "Ants!".to_string(),
			..Default::default()
		})
		.add_startup_system(setup_camera.system())
		.add_startup_system(setup_stage.system())
		.add_startup_system(anthill::make_ants.system())
		.add_system(anthill::make_ants.system())
		.add_system(anthill::take_food.system())
		.add_system(ant::ant_wander.system())
		.add_system(ant::ant_move.system())
		.add_system(ant::ant_collide.system())
		.add_system(ant::ant_emit_pheromone.system())
		.add_system(ant::ant_seek_pheromone.system())
		.add_system(pheromone::pheromone_decay.system())
		.add_system(food::food_carried.system())
		.add_system(food::food_pickup.system())
		.run()
}

fn setup_camera(mut commands: Commands) {
	let mut camera = OrthographicCameraBundle::new_2d();
	camera.transform.scale = Vec3::new(2.0, 2.0, 2.0);
	commands.spawn_bundle(camera);
}
