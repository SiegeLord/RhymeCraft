use engine::id_map::HasId;
use game_state::*;
use allegro::*;
use allegro_font::*;
use map::create_map;

const WIDTH: i32 = 64;
const HEIGHT: i32 = 16;

pub fn create_main_menu(state: &mut GameState) -> Object
{
	let mut obj = Object::new(state.new_id());
	obj.is_main_menu = true;
	obj
}

simple_behavior!
{
	MainMenuDraw[obj.is_main_menu] |obj, state|
	{
		let mid_x = state.buffer.get_width() / 2;
		let mid_y = state.buffer.get_height() / 2;
		
		let bkg = state.menu_background.as_ref().unwrap();
		bkg.draw((mid_x - bkg.get_width() / 2) as f32, (mid_y - bkg.get_height() / 2) as f32, state);
		
		let x = mid_x - WIDTH * 2;
		let y = mid_y;
		state.blank_button.as_ref().unwrap().draw(x as f32, y as f32, state);
		state.core.draw_text(&state.ui_font, Color::from_rgba(192, 192, 255, 255), (x + WIDTH / 2) as f32, (y + 4) as f32, FontAlign::Centre, "Start");
		let x = mid_x + WIDTH * 1;
		state.blank_button.as_ref().unwrap().draw(x as f32, y as f32, state);
		state.core.draw_text(&state.ui_font, Color::from_rgba(255, 192, 192, 255), (x + WIDTH / 2) as f32, (y + 4) as f32, FontAlign::Centre, "Quit");

		break;
	}
}

simple_behavior!
{
	MainMenuInput[obj.is_main_menu] |obj, state|
	{
		if let (Some(mouse_x), Some(mouse_y)) = (state.mouse_x, state.mouse_y)
		{
			let mouse_x = mouse_x / SCALE as i32;
			let mouse_y = mouse_y / SCALE as i32;
			let button = state.mouse_button_down.unwrap_or(0);
			if button == 1
			{		
				let mid_x = state.buffer.get_width() / 2;
				let mid_y = state.buffer.get_height() / 2;
				let x = mid_x - WIDTH * 2;
				let y = mid_y;
				if mouse_x > x && mouse_x < x + WIDTH && mouse_y > y && mouse_y < y + HEIGHT
				{
					let map = create_map("data/map0.cfg", state);
					state.add_object(map);
					state.remove_object(obj.get_id());
				}
				let x = mid_x + WIDTH * 1;
				if mouse_x > x && mouse_x < x + WIDTH && mouse_y > y && mouse_y < y + HEIGHT
				{
					state.quit = true;
				}
			}
		}

		break;
	}
}
