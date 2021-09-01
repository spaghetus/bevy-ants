use std::fs::File;

use bevy::{prelude::*, ui::widget::Image};
use image::{DynamicImage, GenericImageView};
use png::Transformations;
use rayon::prelude::*;

use crate::{
	anthill::{Anthill, HasFood, Team},
	food::{self, Food},
};

pub struct Stage {
	pub image: DynamicImage,
	pub width: u32,
	pub height: u32,
}

pub const SOLID_COLOR: &[u8] = &[255, 255, 255, 255];
const FOOD_COLOR: &[u8] = &[0, 255, 0, 255];

pub fn setup_stage(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	asset_server: Res<AssetServer>,
) {
	let (image, width, height) = crate::png::load("./assets/stage.png");
	let stage = Stage {
		image: image.clone(),
		width,
		height,
	};
	let texture_handle = asset_server.load("stage.png");
	commands.spawn_bundle(SpriteBundle {
		sprite: Sprite::new(Vec2::new(width as f32, height as f32)),
		material: materials.add(ColorMaterial {
			texture: Some(texture_handle),
			..Default::default()
		}),
		transform: Transform::from_translation(Vec3::ZERO),
		..Default::default()
	});
	let hives: Vec<(Anthill, Team, Transform)> = (0..width)
		.flat_map(|x| {
			let x = x.clone();
			let image = image.clone();
			(0..height)
				.filter(|y| {
					let pixel = image.get_pixel(x, *y);
					pixel.0 != FOOD_COLOR && pixel.0 != SOLID_COLOR && pixel[3] == 255
				})
				.map(|y| {
					let pixel = image.get_pixel(x, y);
					let x = x as i32 - width as i32 / 2;
					let y = y as i32 - height as i32 / 2;
					Anthill::new(Team(pixel[0], pixel[1], pixel[2]), (x, y))
				})
				.collect::<Vec<(Anthill, Team, Transform)>>()
		})
		.collect();
	for hive in hives {
		commands
			.spawn_bundle(SpriteBundle {
				sprite: Sprite::new(Vec2::new(10.0, 10.0)),
				material: materials.add(ColorMaterial::color(Color::rgb_u8(
					hive.1 .0, hive.1 .1, hive.1 .2,
				))),
				transform: hive.2,
				..Default::default()
			})
			.insert(hive.0)
			.insert(hive.1)
			.insert(HasFood(10));
	}
	let (food_image, food_width, food_height) = crate::png::load("./assets/food.png");
	assert_eq!(width, food_width);
	assert_eq!(height, food_height);
	let foods: Vec<(Food, Transform)> = (0..width)
		.flat_map(|x| {
			let x = x.clone();
			let food_image = food_image.clone();
			(0..height)
				.filter(|y| {
					let pixel = food_image.get_pixel(x, *y);
					pixel.0 == FOOD_COLOR
				})
				.map(|y| {
					let x = x as i32 - width as i32 / 2;
					let y = y as i32 - height as i32 / 2;
					(
						Food,
						Transform::from_translation(Vec3::new(x as f32, y as f32, 0.0)),
					)
				})
				.collect::<Vec<(Food, Transform)>>()
		})
		.collect();
	for food in foods {
		commands
			.spawn_bundle(SpriteBundle {
				sprite: Sprite::new(Vec2::new(2.0, 2.0)),
				material: materials.add(ColorMaterial::color(Color::rgb_u8(0, 255, 0))),
				transform: food.1,
				..Default::default()
			})
			.insert(food.0);
	}
	commands.insert_resource(stage);
}
