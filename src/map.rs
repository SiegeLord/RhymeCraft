use game_state::*;

use allegro::*;
use engine::id_map::HasId;
use std::cmp::{min, max};
use util::populate_from_file;
use cursor::create_cursor;
use announce::create_announcement;
use unit::{create_unit, create_magic_circle};
use main_menu::create_main_menu;

slr_def!
{
    #[derive(Debug)]
    pub struct MapConfig
    {
        tilesheet: String = String::new(),
        width: i32 = 32,
        height: i32 = 32,
        tiles: String = String::new(),
        words: Vec<String> = vec![],
        next_map: String = String::new()
    }
}

fn tiles_to_index(tl: char, tr: char, br: char, bl: char) -> usize
{
	match (tl, tr, br, bl)
	{
		('w', 'w', 'w', 'w') => 0,
		('s', 's', 's', 's') => 1,
		
		('w', 'w', 's', 's') => 2,
		('s', 'w', 'w', 's') => 3,
		('s', 's', 'w', 'w') => 4,
		('w', 's', 's', 'w') => 5,
		
		('w', 'w', 's', 'w') => 6,
		('w', 'w', 'w', 's') => 7,
		('s', 'w', 'w', 'w') => 8,
		('w', 's', 'w', 'w') => 9,
		
		('s', 'w', 's', 'w') => 10,
		('w', 's', 'w', 's') => 11,
		
		('s', 's', 'w', 's') => 12,
		('s', 's', 's', 'w') => 13,
		('w', 's', 's', 's') => 14,
		('s', 'w', 's', 's') => 15,
		
		_ => 0,
	}
}

// This clamps
pub fn xy_to_index(mut x: i32, mut y: i32, w: i32, h: i32) -> usize
{
	x = min(x, w - 1);
	y = min(y, h - 1);
	x = max(x, 0);
	y = max(y, 0);
	
	(y * w + x) as usize
}

pub fn index_to_xy(idx: i32, w: i32) -> (i32, i32)
{
	(idx % w, idx / w)
}

fn is_solid(tile: char) -> bool
{
	match tile
	{
		's' => false,
		'w' => true,
		_ => true,
	}
}

pub fn map_tile_solid(x: i32, y: i32, map_data: &MapData) -> bool
{
	let idx = xy_to_index(x, y, map_data.width, map_data.height);
	return x < 1 || y < 1 || x > map_data.width - 2 || y > map_data.height - 2 || 
		is_solid(map_data.tiles[idx]) || map_data.collision_map[idx];
}

fn load_tilesheet(filename: &str, w: i32, h: i32, state: &mut GameState) -> Vec<SubBitmap>
{
	let bmp = state.bitmap_cache.load(&state.core, filename).unwrap();
	let tiles_x = bmp.get_width() / w;
	let tiles_y = bmp.get_height() / h;
	
	let mut ret = Vec::with_capacity((tiles_x * tiles_y) as usize);
	for y in 0..tiles_y
	{
		for x in 0..tiles_x
		{
			ret.push(bmp.create_sub_bitmap(x * w, y * h, w, h).unwrap());
		}
	}
	ret
}

pub fn create_map(filename: &str, state: &mut GameState) -> Object
{
	let mut obj = Object::new(state.new_id());
	obj.is_map = true;
	
	let mut map_config = MapConfig::new();
	populate_from_file(filename, &mut map_config).unwrap();
	
	let mut map_data = MapData::new();
	map_data.filename = filename.to_string();
	map_data.next_map = map_config.next_map.clone();
	
	for map_char in map_config.tiles.trim().chars().filter(|c| !c.is_whitespace())
	{
		let (x, y) = index_to_xy(map_data.tiles.len() as i32, map_config.width);
		let x = x as f32 * 32.0;
		let y = y as f32 * 32.0;
		let map_char = match map_char
		{
			'W' =>
			{
				let mut obj = create_unit(obj.get_id(), "data/wizard.cfg", true, state);
				obj.x = x;
				obj.y = y;
				map_data.wizard_id = obj.get_id();
				state.add_object(obj);
				's'
			},
			'g' =>
			{
				let mut obj = create_unit(obj.get_id(), "data/goblin.cfg", false, state);
				obj.x = x;
				obj.y = y;
				state.add_object(obj);
				's'
			},
			'd' =>
			{
				let mut obj = create_unit(obj.get_id(), "data/duck.cfg", false, state);
				obj.x = x;
				obj.y = y;
				state.add_object(obj);
				's'
			},
			'D' =>
			{
				let mut obj = create_unit(obj.get_id(), "data/duck.cfg", true, state);
				obj.x = x;
				obj.y = y;
				state.add_object(obj);
				's'
			},
			'r' =>
			{
				let mut obj = create_unit(obj.get_id(), "data/dragon.cfg", false, state);
				obj.x = x;
				obj.y = y;
				state.add_object(obj);
				's'
			},
			'R' =>
			{
				let mut obj = create_unit(obj.get_id(), "data/dragon.cfg", true, state);
				obj.x = x;
				obj.y = y;
				state.add_object(obj);
				's'
			},
			'G' =>
			{
				let mut obj = create_unit(obj.get_id(), "data/goblin.cfg", true, state);
				obj.x = x;
				obj.y = y;
				state.add_object(obj);
				's'
			},
			other =>
			{
				if let Some(idx) = other.to_digit(10)
				{
					let mut obj = create_magic_circle(obj.get_id(), &map_config.words[idx as usize], state);
					obj.x = x;
					obj.y = y;
					state.add_object(obj);
					's'
				}
				else
				{
					other
				}
			}
		};
		map_data.tiles.push(map_char);
	}
	map_data.width = map_config.width;
	map_data.height = map_config.height;
	if map_data.height * map_data.width != map_data.tiles.len() as i32 {
		panic!("Incorrect number of tiles! {} {} vs {}", filename, map_data.tiles.len(), map_data.height * map_data.width);
	}
	map_data.collision_map.resize(map_data.tiles.len(), false);
	map_data.tilesheet = load_tilesheet(&map_config.tilesheet, 32, 32, state);
	*obj.map_data.borrow_mut() = map_data;
	
	let cursor = create_cursor(obj.get_id(), state);
	state.add_object(cursor);
	
	state.current_map_id = obj.get_id();
	
	let announce = create_announcement(obj.get_id(), "Defeat all monsters!", state);
	state.add_object(announce);
	
	obj
}

simple_behavior!
{
	MapDraw[obj.is_map] |obj, state|
	{
		let map_data = obj.map_data.borrow();
		state.core.hold_bitmap_drawing(true);
		for y in 0..map_data.height as i32 - 1
		{
			for x in 0..map_data.width as i32 - 1
			{
				let tl = map_data.tiles[xy_to_index(x    , y    , map_data.width, map_data.height)];
				let tr = map_data.tiles[xy_to_index(x + 1, y    , map_data.width, map_data.height)];
				let br = map_data.tiles[xy_to_index(x + 1, y + 1, map_data.width, map_data.height)];
				let bl = map_data.tiles[xy_to_index(x    , y + 1, map_data.width, map_data.height)];
				let idx = tiles_to_index(tl, tr, br, bl);
				let bmp = &map_data.tilesheet[idx];
				state.core.draw_bitmap(bmp, (x * 32) as f32, (y * 32) as f32, BitmapDrawingFlags::zero());
			}
		}
		state.core.hold_bitmap_drawing(false);
		break;
	}
}

simple_behavior!
{
	CameraInput[obj.is_map] |obj, state|
	{
		let mut map_data = obj.map_data.borrow_mut();
		map_data.camera_vx = 0.0;
		map_data.camera_vy = 0.0;
		if let (Some(mouse_x), Some(mouse_y)) = (state.mouse_x, state.mouse_y)
		{
			const SPEED: f32 = 256.0;
			if mouse_x < 8 * SCALE as i32
			{
				map_data.camera_vx = -SPEED;
			}
			if mouse_x > state.disp.get_width() - 8 * SCALE as i32
			{
				map_data.camera_vx = SPEED;
			}
			if mouse_y < 8 * SCALE as i32
			{
				map_data.camera_vy = -SPEED;
			}
			if mouse_y > state.disp.get_height() - 8 * SCALE as i32
			{
				map_data.camera_vy = SPEED;
			}
			break;
		}
	}
}

simple_behavior!
{
	CameraDraw[obj.is_map] |obj, state|
	{
		let map_data = obj.map_data.borrow();
		let mut trans = Transform::identity();
		trans.translate(-map_data.camera_x.floor(), -map_data.camera_y.floor());
		state.core.use_transform(&trans);
		break;
	}
}

simple_behavior!
{
	CameraLogic[obj.is_map] |obj, state|
	{
		let mut map_data = obj.map_data.borrow_mut();
		map_data.camera_x += DT * map_data.camera_vx;
		map_data.camera_y += DT * map_data.camera_vy;
		let max_x = ((map_data.width - 1) * 32 as i32 - state.buffer.get_width()) as f32;
		let max_y = ((map_data.height - 1) * 32 as i32 - state.buffer.get_height()) as f32;
		if map_data.camera_x > max_x
		{
			map_data.camera_x = max_x;
		}
		if map_data.camera_y > max_y
		{
			map_data.camera_y = max_y;
		}
		if map_data.camera_x < 0.0
		{
			map_data.camera_x = 0.0;
		}
		if map_data.camera_y < 0.0
		{
			map_data.camera_y = 0.0;
		}
		break;
	}
}

pub struct InventoryLogic;

complex_behavior!
{
	InventoryLogic[obj.is_map] |self, obj, objects, state|
	{
		let map_data = objects.get(state.current_map_id).map(|obj| obj.map_data.clone());
		if let Some(ref map_data) = map_data
		{
			let mut map_data = map_data.borrow_mut();
			let mut map_data = &mut *map_data;
			map_data.inventory.clear();
			for word in &map_data.base_inventory
			{
				map_data.inventory.push(word.clone());
			}
			map_data.num_circles_held = 0;
			for our_obj in objects.elems().iter().filter(|obj| obj.is_ours && obj.can_act && obj.has_pos)
			{
				for magic_circle in objects.elems().iter().filter(|obj| obj.is_magic_circle && obj.has_pos && obj.has_name)
				{
					if our_obj.tile_x == magic_circle.tile_x && our_obj.tile_y == magic_circle.tile_y
					{
						map_data.inventory.push(magic_circle.word.clone());
						map_data.num_circles_held += 1;
					}
				}
			}
			map_data.inventory.sort();
		}
	}
}

pub struct VictoryLogic;

complex_behavior!
{
	VictoryLogic[obj.is_map] |self, obj, objects, state|
	{
		let map_data = objects.get(state.current_map_id).map(|obj| obj.map_data.clone());
		if let Some(ref map_data) = map_data
		{
			let mut map_data = map_data.borrow_mut();
			if objects.get(map_data.wizard_id).is_none() && !map_data.want_restart
			{
				let announce = create_announcement(state.current_map_id, "You have died!", state);
				state.add_object(announce);
				map_data.want_restart = true;
				map_data.change_time = state.time + 4.0;
			}
			let num_enemies = objects.elems().iter().filter(|obj| !obj.is_ours && obj.can_act).count();
			if num_enemies == 0 && !map_data.want_next_map
			{
				let text = if map_data.next_map.is_empty()
				{
					"You've won the game!"
				}
				else
				{
					"Victory!"
				};
				let announce = create_announcement(state.current_map_id, text, state);
				state.add_object(announce);
				map_data.want_next_map = true;
				map_data.change_time = state.time + 4.0;
			}
			if map_data.want_restart && state.time > map_data.change_time
			{
				let old_map_id = state.current_map_id;
				state.remove_object(old_map_id);
				let same_map = create_map(&map_data.filename, state);
				state.add_object(same_map);
			}
			if map_data.want_next_map && state.time > map_data.change_time
			{
				let old_map_id = state.current_map_id;
				state.remove_object(old_map_id);
				if map_data.next_map.is_empty()
				{
					info!("Going back to menu!");
					let menu = create_main_menu(state);
					state.add_object(menu);
				}
				else
				{
					let new_map = create_map(&map_data.next_map, state);
					state.add_object(new_map);
				}
			}
		}
	}
}
