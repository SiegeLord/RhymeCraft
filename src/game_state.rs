use engine::world::WorldState;
use engine::bitmap_cache::BitmapCache;
use engine::id_map::{HasId, IdMint, UniqueId};
use animation::Animation;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use craft::load_spells;

use allegro::*;
use allegro_primitives::*;
use allegro_font::*;
use std::collections::HashSet;

pub const DT: f32 = 1.0 / 120.0;
pub const SCALE: f32 = 4.0;

macro_rules! simple_behavior
{
	($name: ident[$check: expr] |$obj: ident, $state: ident| $e: expr) =>
	{
		pub struct $name;

		impl ::engine::world::Behavior<::game_state::Object, ::game_state::GameState> for $name
		{
			fn check_object(&self, $obj: &::game_state::Object) -> bool
			{
				$check
			}

			fn handle_objects(&mut self, objects: &mut ::engine::id_map::IdMap<::game_state::Object>, $state: &mut ::game_state::GameState)
			{
				for $obj in objects.elems_mut()
				{
					if self.check_object($obj)
					{
						$e
					}
				}
			}
		}
	}
}

macro_rules! complex_behavior
{
	($name: ident[$check: expr] |$self_: ident, $obj: ident, $objects: ident, $state: ident| $e: expr) =>
	{
		impl ::engine::world::Behavior<::game_state::Object, ::game_state::GameState> for $name
		{
			fn check_object(&$self_, $obj: &::game_state::Object) -> bool
			{
				$check
			}

			fn handle_objects(&mut $self_, $objects: &mut ::engine::id_map::IdMap<::game_state::Object>, $state: &mut ::game_state::GameState)
			{
				$e
			}
		}
	}
}

macro_rules! object
{
	($name: ident
	{
		$($member: ident : $type_ : ty = $init: expr),* $(,)*
	}) =>
	{
		pub struct $name
		{
			id: UniqueId,
			$(pub $member : $type_),*
		}

		impl $name
		{
			pub fn new(id: UniqueId) -> $name
			{
				$name
				{
					id: id,
					$($member : $init),*
				}
			}
		}
	}
}

pub struct MapData
{
	pub tilesheet: Vec<SubBitmap>,
	pub collision_map: Vec<bool>,
	pub tiles: Vec<char>,
	pub width: i32,
	pub height: i32,
	pub executing_orders: bool,
	pub camera_x: f32,
	pub camera_y: f32,
	pub camera_vx: f32,
	pub camera_vy: f32,
	pub our_turn: bool,
	pub crafting: bool,
	pub base_inventory: Vec<String>,
	pub inventory: Vec<String>,
	pub spell: Vec<Vec<String>>,
	pub mana: i32,
	pub turn: i32,
	pub wizard_id: usize,
	pub num_circles_held: i32,
	pub want_restart: bool,
	pub want_next_map: bool,
	pub change_time: f64,
	pub filename: String,
	pub next_map: String,
}

impl MapData
{
	pub fn new() -> MapData
	{
		MapData
		{
			tilesheet: vec![],
			collision_map: vec![],
			tiles: vec![],
			width: 0,
			height: 0,
			executing_orders: false,
			camera_x: 0.0,
			camera_y: 0.0,
			camera_vx: 0.0,
			camera_vy: 0.0,
			our_turn: true,
			crafting: false,
			base_inventory: vec!["bösh".to_string(), "bïshi".to_string(), "caråzo".to_string()],
			inventory: vec![],
			spell: vec![vec![]],
			mana: 10,
			turn: 1,
			wizard_id: 0,
			num_circles_held: 0,
			want_restart: false,
			want_next_map: false,
			change_time: 0.0,
			filename: "".to_string(),
			next_map: "".to_string(),
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub enum OrderType
{
	MoveTo,
	Attack,
}

#[derive(Copy, Clone, Debug)]
pub struct Order
{
	pub x: i32,
	pub y: i32,
	pub order_type: OrderType,
}

object!
{
	Object
	{
		parent: usize = 0,

		has_pos: bool = false,
		x: f32 = 0.0,
		y: f32 = 0.0,
		tile_x: i32 = 0,
		tile_y: i32 = 0,
		old_x: f32 = 0.0,
		old_y: f32 = 0.0,

		has_vel: bool = false,
		vx: f32 = 0.0,
		vy: f32 = 0.0,

		debug_draw: bool = false,
		
		is_ours: bool = false,
		
		is_map: bool = false,
		map_data: Rc<RefCell<MapData>> = Rc::new(RefCell::new(MapData::new())),
		
		has_sprite: bool = false,
		sprite: Option<Animation> = None,
		
		is_selectable: bool = false,
		selected: bool = false,
		
		is_solid: bool = false,
		
		has_health: bool = false,
		health: i32 = 0,
		max_health: i32 = 0,
		
		can_act: bool = false,
		max_action_points: i32 = 0,
		action_points: i32 = 0,
		orders: Vec<Order> = vec![],
		executing_orders: bool = false,
		sight_range: i32 = 0,
		damage: i32 = 0,
		fire: bool = false,
		
		is_cursor: bool = false,
		
		is_effect: bool = false,
		effect_death_time: f64 = 0.0,
		
		is_announcement: bool = false,
		announcement: String = "".to_string(),
		start_fall_time: f64 = 0.0,
		
		has_name: bool = false,
		name: String = "".to_string(),
		
		is_magic_circle: bool = false,
		word: String = "".to_string(),
		
		is_main_menu: bool = false,
	}
}

impl HasId for Object
{
	fn get_id(&self) -> usize
	{
		self.id.get()
	}
}

pub struct GameState
{
	pub core: Core,
	pub prim: PrimitivesAddon,
	pub disp: Display,
	pub buffer: Bitmap,
	pub font: FontAddon,

	pub id_mint: IdMint,

	new_objects: Vec<Object>,
	ids_to_remove: HashSet<usize>,

	pub key_down: Option<KeyCode>,
	pub key_up: Option<KeyCode>,
	pub mouse_button_down: Option<u32>,
	pub quit: bool,
	pub paused: bool,
	pub time: f64,
	pub draw_interp: f32,
	pub ui_font: Font,
	pub bitmap_cache: BitmapCache,
	pub cursor_select: Option<Animation>,
	pub bob_selected: Option<Animation>,
	pub path_marker: Option<Animation>,
	pub path_end: Option<Animation>,
	pub path_attack: Option<Animation>,
	pub our_moves: Option<Animation>,
	pub our_no_moves: Option<Animation>,
	pub enemy_flag: Option<Animation>,
	pub menu_button: Option<Animation>,
	pub craft_button: Option<Animation>,
	pub turn_button: Option<Animation>,
	pub menu_background: Option<Animation>,
	pub blank_button: Option<Animation>,
	pub newline_button: Option<Animation>,
	pub backspace_button: Option<Animation>,
	pub spells: HashMap<String, String>,
	
	pub mouse_x: Option<i32>,
	pub mouse_y: Option<i32>,
	pub cursor_x: i32,
	pub cursor_y: i32,
	
	pub current_map_id: usize,
}

impl GameState
{
	pub fn new(core: Core, prim: PrimitivesAddon, disp: Display, buffer: Bitmap, font: FontAddon) -> GameState
	{
		let mut cache = BitmapCache::new();
		let font_path = "data/a4_font.tga";
		let bmp = cache.load(&core, font_path).unwrap();
		let ranges = [
			(0x0020, 0x007F),  /* ASCII */
			(0x00A1, 0x00FF),  /* Latin 1 */
			(0x0100, 0x017F),  /* Extended-A */
			(0x20AC, 0x20AC)   /* Euro */
		];
		
		let ui_font = Font::grab_from_bitmap(&font, &*bmp, &ranges).expect(&format!("Couldn't load {}", font_path));
		let mut state = GameState
		{
			core: core,
			prim: prim,
			disp: disp,
			buffer: buffer,
			font: font,
			ui_font: ui_font,
			key_down: None,
			key_up: None,
			mouse_button_down: None,
			quit: false,
			paused: false,
			time: 0.0,
			draw_interp: 0.0,
			new_objects: vec![],
			ids_to_remove: HashSet::new(),
			id_mint: IdMint::new(),
			bitmap_cache: cache,
			cursor_select: None,
			mouse_x: None,
			mouse_y: None,
			cursor_x: 0,
			cursor_y: 0,
			current_map_id: 0,
			bob_selected: None,
			path_marker: None,
			path_end: None,
			path_attack: None,
			our_moves: None,
			our_no_moves: None,
			enemy_flag: None,
			menu_button: None,
			turn_button: None,
			craft_button: None,
			blank_button: None,
			backspace_button: None,
			newline_button: None,
			menu_background: None,
			spells: load_spells(),
		};
		state.cursor_select = Some(Animation::new("data/cursor_select.png", false, &mut state));
		state.bob_selected = Some(Animation::new("data/bob_selected.cfg", false, &mut state));
		state.path_marker = Some(Animation::new("data/path_marker.cfg", false, &mut state));
		state.path_attack = Some(Animation::new("data/path_attack.cfg", false, &mut state));
		state.path_end = Some(Animation::new("data/path_end.cfg", false, &mut state));
		state.our_moves = Some(Animation::new("data/flag_ours_moves_left.cfg", false, &mut state));
		state.our_no_moves = Some(Animation::new("data/flag_ours_no_moves.cfg", false, &mut state));
		state.enemy_flag = Some(Animation::new("data/flag_enemy.cfg", false, &mut state));
		state.menu_button = Some(Animation::new("data/menu_button.png", false, &mut state));
		state.turn_button = Some(Animation::new("data/turn_button.png", false, &mut state));
		state.craft_button = Some(Animation::new("data/craft_button.png", false, &mut state));
		state.menu_background = Some(Animation::new("data/title.png", false, &mut state));
		state.blank_button = Some(Animation::new("data/blank_button.png", false, &mut state));
		state.backspace_button = Some(Animation::new("data/backspace_button.png", false, &mut state));
		state.newline_button = Some(Animation::new("data/newline_button.png", false, &mut state));
		state
	}

	pub fn add_object(&mut self, obj: Object)
	{
		self.new_objects.push(obj);
	}

	pub fn remove_object(&mut self, id: usize)
	{
		self.ids_to_remove.insert(id);
	}

	pub fn new_id(&mut self) -> UniqueId
	{
		self.id_mint.new_id()
	}
}

impl WorldState<Object> for GameState
{
	fn get_new_objects(&mut self) -> &mut Vec<Object>
	{
		&mut self.new_objects
	}

	fn get_ids_to_remove(&mut self) -> &mut HashSet<usize>
	{
		&mut self.ids_to_remove
	}
}
