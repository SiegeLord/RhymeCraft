use allegro::*;
use allegro_font::*;
use util::populate_from_file;
use engine::id_map::HasId;
use game_state::*;
use animation::Animation;
use map::xy_to_index;
use effect::create_death;

slr_def!
{
    #[derive(Clone, Debug)]
    pub struct UnitConfig
    {
        sprite: String = "".to_string(),
        action_points: i32 = 25,
        health: i32 = 4,
        sight_range: i32 = 4,
        damage: i32 = 1,
        fire: i32 = 0,
        name: String = "unit".to_string()
    }
}

pub fn create_unit(parent: usize, file: &str, ours: bool, state: &mut GameState) -> Object
{
	let mut config = UnitConfig::new();
	populate_from_file(file, &mut config).unwrap();
	
	let mut obj = Object::new(state.new_id());
	obj.has_name = true;
	obj.name = config.name.clone();
	obj.parent = parent;
	obj.has_pos = true;
	obj.has_vel = true;
	obj.has_sprite = true;
	obj.is_selectable = true;
	obj.is_solid = true;
	obj.can_act = true;
	obj.has_health = true;
	obj.health = config.health;
	obj.max_health = config.health;
	obj.is_ours = ours;
	obj.fire = config.fire != 0;
	obj.max_action_points = config.action_points;
	obj.action_points = config.action_points;
	obj.sight_range = config.sight_range;
	obj.damage = config.damage;
	obj.sprite = Some(Animation::new(&config.sprite, false, state));
	
	obj
}

pub fn create_magic_circle(parent: usize, word: &str, state: &mut GameState) -> Object
{
	let mut obj = Object::new(state.new_id());
	obj.has_name = true;
	obj.name = "Magic crcl".to_string();
	obj.word = word.to_string();
	obj.parent = parent;
	obj.has_pos = true;
	obj.has_sprite = true;
	obj.is_selectable = true;
	obj.is_magic_circle = true;
	obj.sprite = Some(Animation::new("data/magic_circle.cfg", false, state));
	
	obj
}

simple_behavior!
{
	UnitDraw[obj.has_sprite && obj.has_pos && !obj.is_effect && !obj.is_magic_circle] |obj, state|
	{
		obj.sprite.as_ref().unwrap().draw(obj.x - 16.0, obj.y - 16.0, state);
	}
}

simple_behavior!
{
	MagicCircleDraw[obj.has_sprite && obj.has_pos && obj.is_magic_circle] |obj, state|
	{
		obj.sprite.as_ref().unwrap().draw(obj.x - 16.0, obj.y - 16.0, state);
	}
}

simple_behavior!
{
	UnitDrawPathable[obj.has_sprite && obj.has_pos && obj.can_act] |obj, state|
	{
		let ani = if obj.is_ours && obj.action_points == 0
		{
			state.our_no_moves.as_ref().unwrap()
		}
		else if obj.is_ours
		{
			state.our_moves.as_ref().unwrap()
		}
		else
		{
			state.enemy_flag.as_ref().unwrap()
		};
		
		ani.draw(obj.x - 16.0, obj.y - 16.0, state);
	}
}

simple_behavior!
{
	UnitLogic[obj.has_pos] |obj, _state|
	{
		obj.tile_x = (obj.x as i32 + 16) / 32;
		obj.tile_y = (obj.y as i32 + 16) / 32;
	}
}

simple_behavior!
{
	HealthDeathLogic[obj.has_health] |obj, state|
	{
		if obj.health <= 0
		{
			state.remove_object(obj.get_id());
			let death = create_death(state.current_map_id, obj.x, obj.y, state);
			state.add_object(death);
		}
	}
}

simple_behavior!
{
	SelectedDraw[obj.is_selectable && obj.selected] |obj, state|
	{
		let x = state.buffer.get_width() - 80;
		let mut y = state.buffer.get_height() - 48;
		if obj.has_name
		{
			state.core.draw_text(&state.ui_font, Color::from_rgba(224, 224, 224, 255),
				x as f32, y as f32, FontAlign::Left, &obj.name);
			y += 10;
		}
		
		if obj.has_health
		{
			state.core.draw_text(&state.ui_font, Color::from_rgba(224, 224, 224, 255),
				x as f32, y as f32, FontAlign::Left, &format!("HP {}/{}", obj.health, obj.max_health));
			y += 10;
		}
		
		if obj.can_act
		{
			state.core.draw_text(&state.ui_font, Color::from_rgba(224, 224, 224, 255),
				x as f32, y as f32, FontAlign::Left, &format!("AP {}/{}", obj.action_points, obj.max_action_points));
		}
		
		if obj.is_magic_circle
		{
			state.core.draw_text(&state.ui_font, Color::from_rgba(224, 224, 224, 255),
				x as f32, y as f32, FontAlign::Left, &format!(r#""{}""#, obj.word));
		}
	}
}

pub struct SolidLogic;

complex_behavior!
{
	SolidLogic[obj.has_pos && obj.is_solid] |self, obj, objects, state|
	{
		let map_data = objects.get(state.current_map_id).map(|obj| obj.map_data.clone());
		if let Some(ref map_data) = map_data
		{
			let mut map_data = map_data.borrow_mut();
			for e in map_data.collision_map.iter_mut()
			{
				*e = false;
			}
			for obj in objects.elems_mut()
			{
				if self.check_object(obj)
				{
					let idx = xy_to_index(obj.tile_x, obj.tile_y, map_data.width, map_data.height);
					map_data.collision_map[idx] = true;
				}
			}
		}
	}
}

simple_behavior!
{
	SelectableDraw[obj.is_selectable && obj.has_pos] |obj, state|
	{
		if obj.selected
		{
			state.bob_selected.as_ref().unwrap().draw(obj.x - 16.0, obj.y - 16.0, state);
		}
	}
}

pub struct SelectableInput;

complex_behavior!
{
	SelectableInput[obj.is_selectable && obj.has_pos] |self, obj, objects, state|
	{
		let button = state.mouse_button_down.unwrap_or(0);
		if button == 1
		{
			let mut new_selection = None;
			for obj in objects.elems_mut()
			{
				if self.check_object(obj)
				{
					if obj.tile_x == state.cursor_x && obj.tile_y == state.cursor_y && !obj.selected
					{
						obj.selected = true;
						new_selection = Some(obj.get_id());
					}
				}
			}
			for obj in objects.elems_mut()
			{
				if self.check_object(obj)
				{
					if let Some(new_selection) = new_selection
					{
						if obj.get_id() != new_selection
						{
							obj.selected = false;
						}
					}
					//~ else
					//~ {
						//~ obj.selected = false;
					//~ }
				}
			}
		}
	}
}
