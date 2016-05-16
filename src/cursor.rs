use game_state::*;
use std::cmp::{min, max};

pub fn create_cursor(parent: usize, state: &mut GameState) -> Object
{
	let mut obj = Object::new(state.new_id());
	obj.is_cursor = true;
	obj.parent = parent;
	obj
}

pub struct CursorInput;

complex_behavior!
{
	CursorInput[obj.is_cursor] |self, obj, objects, state|
	{
		if let (Some(mouse_x), Some(mouse_y)) = (state.mouse_x, state.mouse_y)
		{
			let map_data = objects.get(state.current_map_id).map(|obj| obj.map_data.clone());
			if let Some(ref map_data) = map_data
			{
				let map_data = map_data.borrow();
				for obj in objects.elems_mut()
				{
					if self.check_object(obj)
					{
						state.cursor_x = (mouse_x / SCALE as i32 + map_data.camera_x as i32 + 16) / 32;
						state.cursor_y = (mouse_y / SCALE as i32 + map_data.camera_y as i32 + 16) / 32;
						
						state.cursor_x = max(state.cursor_x, 1);
						state.cursor_y = max(state.cursor_y, 1);
						state.cursor_x = min(state.cursor_x, map_data.width - 2);
						state.cursor_y = min(state.cursor_y, map_data.height - 2);

						break;
					}
				}
			}
		}
	}
}

simple_behavior!
{
	CursorDraw[obj.is_cursor] |obj, state|
	{
		state.cursor_select.as_ref().unwrap().draw(state.cursor_x as f32 * 32.0 - 16.0, state.cursor_y as f32 * 32.0 - 16.0, state);
		break;
	}
}

