#![allow(dead_code)]

use std::cmp::max;
use std::rc::Rc;

use allegro::*;
use game_state::GameState;
use util::populate_from_file;

use std::path::Path;

slr_def!
{
    #[derive(Clone, Debug)]
    pub struct AnimationConfig
    {
        file: String = String::new(),
        width: i32 = 0,
        height: i32 = 0,
        x_offset: i32 = 0,
        y_offset: i32 = 0,
        rate: f32 = 0.0,
        tiles: String = String::new()
    }
}

#[derive(Clone)]
pub struct Animation
{
	bmp: Rc<Bitmap>,
	offset_time: f64,
	play_once: bool,
	num_frames: i32,
	num_frames_x: i32,
	config: AnimationConfig,
}

impl Animation
{
	pub fn new(filename: &str, play_once: bool, state: &mut GameState) -> Animation
	{
		let path = Path::new(filename);
		let mut config = AnimationConfig::new();
		if path.extension().unwrap() == "png"
		{
			let bmp = state.bitmap_cache.load(&state.core, filename).unwrap();
			config.width = bmp.get_width();
			config.height = bmp.get_height();
			Animation
			{
				bmp: bmp,
				offset_time: state.core.get_time(),
				play_once: play_once,
				num_frames: 1,
				num_frames_x: 1,
				config: config,
			}
		}
		else
		{
			populate_from_file(filename, &mut config).unwrap();
			
			let bmp = state.bitmap_cache.load(&state.core, &config.file).unwrap();
			
			if config.width <= 0
			{
				config.width = bmp.get_width();
			}
			if config.height <= 0
			{
				config.height = bmp.get_height();
			}
			
			let num_frames_x = bmp.get_width() / config.width;
			let num_frames_y = bmp.get_height() / config.height;
			let num_frames = max(1, num_frames_x * num_frames_y);
			
			Animation
			{
				bmp: bmp,
				offset_time: state.time,
				play_once: play_once,
				num_frames: num_frames,
				num_frames_x: num_frames_x,
				config: config,
			}
		}
	}

	pub fn draw(&self, x: f32, y: f32, state: &GameState)
	{
		let raw_frame = (self.num_frames as f64 * (state.time - self.offset_time) * self.config.rate as f64) as i32;
		if self.play_once && raw_frame >= self.num_frames
		{
			return;
		}
		let frame = raw_frame % self.num_frames;
		let sx = (frame % self.num_frames_x * self.config.width) as f32;
		let sy = (frame / self.num_frames_x * self.config.height) as f32;
		state.core.draw_bitmap_region(&*self.bmp, sx, sy, self.config.width as f32, self.config.height as f32, x + self.config.x_offset as f32, y + self.config.y_offset as f32, Flag::zero());
	}

	pub fn get_width(&self) -> i32
	{
		self.config.width
	}

	pub fn get_height(&self) -> i32
	{
		self.config.height
	}

	pub fn get_y_offset(&self) -> i32
	{
		self.config.y_offset
	}

	pub fn get_x_offset(&self) -> i32
	{
		self.config.x_offset
	}
}
