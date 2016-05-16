use allegro::*;
use allegro_font::*;
use game_state::*;
use util::dist;
use announce::create_announcement;
use map::map_tile_solid;
use unit::create_unit;
use main_menu::create_main_menu;
use effect::create_spawn;

const POEM_WIDTH: i32 = 256;
const POEM_HEIGHT: i32 = 128;
const Y_OFFSET: i32 = 64;
const WIDTH: i32 = 64;
const HEIGHT: i32 = 16;
const SPACE: i32 = 8;

fn get_inventory_xy(idx: usize) -> (i32, i32)
{
	let x_slot = idx as i32 % 4;
	let y_slot = idx as i32 / 4;
	
	((x_slot - 2) * (WIDTH + SPACE) + SPACE / 2, y_slot * (HEIGHT + SPACE))
}

simple_behavior!
{
	IdentityTransformDraw[obj.is_map] |obj, state|
	{
		state.core.use_transform(&Transform::identity());
	}
}

simple_behavior!
{
	UIDraw[obj.is_map] |obj, state|
	{
		let map_data = obj.map_data.borrow();
		state.menu_button.as_ref().unwrap().draw(0.0, 0.0, state);
		let turn_button = state.turn_button.as_ref().unwrap();
		let craft_button = state.craft_button.as_ref().unwrap();
		turn_button.draw(0.0, (state.buffer.get_height() - turn_button.get_height()) as f32, state);
		craft_button.draw((state.buffer.get_width() - turn_button.get_width()) as f32, 0.0, state);
		
		let mid_x = state.buffer.get_width() / 2;
		let mid_y = state.buffer.get_height() / 2;
		
		state.core.draw_text(&state.ui_font, Color::from_rgba(192, 255, 128, 255),
			mid_x as f32, 4 as f32, FontAlign::Centre, &format!("MANA: {}", map_data.mana));
		
		if map_data.crafting
		{			
			let x = mid_x - POEM_WIDTH / 2;
			let y = mid_y - POEM_HEIGHT + Y_OFFSET - (HEIGHT + SPACE + SPACE);
			state.prim.draw_filled_rectangle(
					x as f32, y as f32,
					(x + POEM_WIDTH) as f32, (y + POEM_HEIGHT) as f32,
					Color::from_rgba(0, 0, 0, 128));
			for (i, spell_line_words) in map_data.spell.iter().enumerate()
			{
				let mut spell_line = String::new();
				for spell_word in spell_line_words
				{
					spell_line.push_str(spell_word);
					spell_line.push_str(" ");
				}
				if i + 1 == map_data.spell.len()
				{
					spell_line.push_str("¤");
				}
				else
				{
					spell_line.pop();
				}
				let i = i as i32;
				state.core.draw_text(&state.ui_font, Color::from_rgba(255, 255, 255, 255), ( mid_x - POEM_WIDTH / 2 + 4) as f32, (y + 4 + i * (HEIGHT + SPACE)) as f32, FontAlign::Left, &spell_line);
			}
			
			let x = mid_x - WIDTH - SPACE - WIDTH / 2;
			let y = mid_y - SPACE - HEIGHT + Y_OFFSET;
			state.newline_button.as_ref().unwrap().draw(x as f32, y as f32, state);
			let x = mid_x + SPACE + WIDTH / 2;
			state.backspace_button.as_ref().unwrap().draw(x as f32, y as f32, state);
			let x = mid_x - WIDTH / 2;
			state.blank_button.as_ref().unwrap().draw(x as f32, y as f32, state);
			state.core.draw_text(&state.ui_font, Color::from_rgba(255, 255, 255, 255), (x + WIDTH / 2) as f32, (y + 4) as f32, FontAlign::Centre, "Craft!");
			
			for (i, ref string) in map_data.inventory.iter().enumerate()
			{
				let (x, y) = get_inventory_xy(i);
				let x = x + mid_x;
				let y = y + mid_y + Y_OFFSET;
				state.blank_button.as_ref().unwrap().draw(x as f32, y as f32, state);
				state.core.draw_text(&state.ui_font, Color::from_rgba(255, 255, 255, 255), (x + WIDTH / 2) as f32, (y + 4) as f32, FontAlign::Centre, &*string);
			}
		}
		break;
	}
}

pub struct UIInput;

complex_behavior!
{
	UIInput[obj.is_map] |self, obj, objects, state|
	{
		let map_data = objects.get(state.current_map_id).map(|obj| obj.map_data.clone());
		if let Some(ref map_data) = map_data
		{
			let mut map_data = map_data.borrow_mut();
			if let (Some(mouse_x), Some(mouse_y)) = (state.mouse_x, state.mouse_y)
			{
				let mouse_x = mouse_x / SCALE as i32;
				let mouse_y = mouse_y / SCALE as i32;
				let button = state.mouse_button_down.unwrap_or(0);
				if dist(mouse_x, mouse_y, 16, 16) < 12
				{
					if button == 1
					{
						let map_id = state.current_map_id;
						state.remove_object(map_id);
						info!("Going back to menu!");
						let menu = create_main_menu(state);
						state.add_object(menu);
					}
					state.mouse_x = None;
					state.mouse_y = None;
					state.mouse_button_down = None;
				}
				else if dist(mouse_x, mouse_y, 16, state.buffer.get_height() - 16) < 12
				{
					if button == 1
					{
						map_data.our_turn = false;
						map_data.crafting = false;
					}
					state.mouse_x = None;
					state.mouse_y = None;
					state.mouse_button_down = None;
				}
				else if dist(mouse_x, mouse_y, state.buffer.get_width() - 16, 16) < 12
				{
					if button == 1
					{
						map_data.crafting = !map_data.crafting;
					}
					state.mouse_x = None;
					state.mouse_y = None;
					state.mouse_button_down = None;
				}
			}
		}
	}
}

pub struct CraftInput;

complex_behavior!
{
	CraftInput[obj.is_map] |self, obj, objects, state|
	{
		let map_data = objects.get(state.current_map_id).map(|obj| obj.map_data.clone());
		if let Some(ref map_data) = map_data
		{
			let mut map_data = map_data.borrow_mut();
			
			if !map_data.crafting
			{
				return;
			}
			
			if let (Some(mouse_x), Some(mouse_y)) = (state.mouse_x, state.mouse_y)
			{
				let mouse_x = mouse_x / SCALE as i32;
				let mouse_y = mouse_y / SCALE as i32;
				
				let mid_x = state.buffer.get_width() / 2;
				let mid_y = state.buffer.get_height() / 2;
				
				let button = state.mouse_button_down.unwrap_or(0);
				if button == 1
				{
					let mut spell_addition = None;
					for (i, ref string) in map_data.inventory.iter().enumerate()
					{
						let (x, y) = get_inventory_xy(i);
						let x = x + mid_x;
						let y = y + mid_y + Y_OFFSET;
						
						if mouse_x > x && mouse_x < x + WIDTH && mouse_y > y && mouse_y < y + HEIGHT
						{
							spell_addition = Some((*string).clone());
							break;
						}
					}
					if let Some(spell_addition) = spell_addition
					{
						if map_data.spell.last_mut().unwrap().len() < 5
						{
							map_data.spell.last_mut().unwrap().push(spell_addition.clone());
						}
					}
					
					let x = mid_x - WIDTH - SPACE - WIDTH / 2;
					let y = mid_y - SPACE - HEIGHT + Y_OFFSET;
					if mouse_x > x && mouse_x < x + WIDTH && mouse_y > y && mouse_y < y + HEIGHT
					{
						if map_data.spell.len() < 5
						{
							map_data.spell.push(vec![]);
						}
					}
					let x = mid_x + SPACE + WIDTH / 2;
					if mouse_x > x && mouse_x < x + WIDTH && mouse_y > y && mouse_y < y + HEIGHT
					{
						map_data.spell.last_mut().unwrap().pop();
						if map_data.spell.last().unwrap().is_empty() && map_data.spell.len() > 1
						{
							map_data.spell.pop();
						}
					}
					let x = mid_x - WIDTH / 2;
					if mouse_x > x && mouse_x < x + WIDTH && mouse_y > y && mouse_y < y + HEIGHT
					{
						let mut spell = String::new();
						let mut cost = 0;
						for line in &map_data.spell
						{
							for word in line
							{
								spell.push_str(word);
								spell.push_str(" ");
								cost += 1;
							}
							spell.pop();
							spell.push_str("\n");
						}
						spell.pop();
						let cost = cost * cost * cost / 36;
						
						let summon = state.spells.get(&spell).map(|s| s.clone());
						let announce_text = if let Some(summon) = summon
						{
							let mut wizard_x = 0;
							let mut wizard_y = 0;
							if let Some(wizard) = objects.get(map_data.wizard_id)
							{
								wizard_x = wizard.tile_x;
								wizard_y = wizard.tile_y;
							}
							
							let mut found_pos = None;
							for m in &[(-1, 0), (1, 0), (0, -1), (0, 1)]
							{
								let cand_x = wizard_x + m.0;
								let cand_y = wizard_y + m.1;
								if !map_tile_solid(cand_x, cand_y, &*map_data)
								{
									found_pos = Some((cand_x, cand_y));
								}
							}
							
							if let Some((x, y)) = found_pos
							{
								if cost <= map_data.mana
								{
									map_data.mana -= cost;								
									let mut obj = create_unit(state.current_map_id, &summon, true, state);
									obj.x = x as f32 * 32.0;
									obj.y = y as f32 * 32.0;
									obj.action_points = 0;
									let msg = format!("Summoned {}!", obj.name);
									let effect = create_spawn(state.current_map_id, obj.x, obj.y, state);
									state.add_object(obj);
									state.add_object(effect);
									msg
								}
								else
								{
									"Not enough mana!".to_string()
								}
							}
							else
							{
								"No space to summon!".to_string()
							}
						}
						else
						{
							"Gibberish...".to_string()
						};
						let announce = create_announcement(state.current_map_id, &announce_text, state);
						state.add_object(announce);
						
						map_data.crafting = false;
					}
				}
			}
			
			state.mouse_x = None;
			state.mouse_y = None;
			state.mouse_button_down = None;
		}
	}
}
