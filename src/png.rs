use image::{DynamicImage, GenericImageView};
use png::ColorType::*;
use std::fs::File;

pub fn load(path: &str) -> (DynamicImage, u32, u32) {
	// let decoder = png::Decoder::new(File::open(path).unwrap());
	// let mut reader = decoder.read_info().unwrap().1;
	// let mut data = vec![0; reader.output_buffer_size()];
	// reader.next_frame(&mut data).unwrap();
	// let info = reader.info();

	let img = image::open(path).unwrap();

	(img.clone(), img.dimensions().0, img.dimensions().1)
}
