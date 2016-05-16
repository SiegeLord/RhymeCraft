use engine::id_map::HasId;
use game_state::*;
use allegro::*;
use allegro_font::*;

const START: f32 = 512.0;

pub fn create_announcement(parent: usize, text: &str, state: &mut GameState) -> Object
{
	let mut obj = Object::new(state.new_id());
	obj.is_announcement = true;
	obj.y = 0.0;
	obj.announcement = text.to_string();
	obj.start_fall_time = state.time + 2.0;
	obj.parent = parent;

	obj
}

simple_behavior!
{
	AnnounceDraw[obj.is_announcement] |obj, state|
	{
		let mid_x = state.buffer.get_width() / 2;
		let y = state.buffer.get_height() - 32;

		state.core.draw_text(&state.ui_font, Color::from_rgba(255, 255, 32, 255),
			mid_x as f32, (y + obj.y as i32) as f32, FontAlign::Centre, &obj.announcement);
	}
}

simple_behavior!
{
	AnnounceLogic[obj.is_announcement] |obj, state|
	{
		if state.time > obj.start_fall_time
		{
			obj.y += 64.0 * DT;
			if obj.y > START
			{
				state.remove_object(obj.get_id());
			}
		}
	}
}
