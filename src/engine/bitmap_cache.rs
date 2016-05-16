// Copyright 2015 SiegeLord
//
// See LICENSE for terms.

use allegro::*;
use std::collections::HashMap;
use std::rc::Rc;

pub struct BitmapCache
{
	bitmaps: HashMap<String, Rc<Bitmap>>,
}

impl BitmapCache
{
	pub fn new() -> BitmapCache
	{
		BitmapCache
		{
			bitmaps: HashMap::new(),
		}
	}
	
	pub fn load(&mut self, core: &Core, filename: &str) -> Result<Rc<Bitmap>, String>
	{
		if let Some(bmp) = self.bitmaps.get(filename)
		{
			return Ok(bmp.clone());
		}
		
		let bmp = Rc::new(try!(Bitmap::load(core, filename).map_err(|_| format!("Could not load {}", filename))));
		self.bitmaps.insert(filename.to_string(), bmp.clone());
		Ok(bmp)
	}
}
