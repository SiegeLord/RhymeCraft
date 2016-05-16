use slr_config::{Error, ConfigElement, ElementRepr, Source};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

pub fn populate_from_file<'l, T>(filename: &'l str, val: &mut T) -> Result<(), Vec<Error>> where T: ElementRepr<'l>
{
	let mut src = String::new();
	File::open(&filename).expect(&format!("{}", filename)).read_to_string(&mut src).unwrap();
	let mut src = Source::new(Path::new(filename), &src);
	let elem = ConfigElement::from_source(&mut src).unwrap();
	// XXX: Why can't I pass source here?
	val.from_element(&elem, None)
}

pub fn l1_dist(x1: i32, y1: i32, x2: i32, y2: i32) -> i32
{
	(x2 - x1).abs() + (y2 - y1).abs()
}

pub fn dist(x1: i32, y1: i32, x2: i32, y2: i32) -> i32
{
	let dx = (x1 - x2) as f32;
	let dy = (y1 - y2) as f32;
	(dx * dx + dy * dy).sqrt() as i32
}
