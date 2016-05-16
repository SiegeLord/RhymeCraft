// Copyright 2015 SiegeLord
//
// See LICENSE for terms.

use engine::world::WorldState;
use engine::id_map::HasId;

pub struct ParentLogic;

impl ::engine::world::Behavior<::game_state::Object, ::game_state::GameState> for ParentLogic
{
	fn check_object(&self, obj: &::game_state::Object) -> bool
	{
		obj.parent > 0
	}
	
	fn handle_objects(&mut self, objects: &mut ::engine::id_map::IdMap<::game_state::Object>, state: &mut ::game_state::GameState)
	{
		let mut old_ids_to_remove = vec![];
		old_ids_to_remove.extend(state.get_ids_to_remove().iter());
		let mut new_ids_to_remove = vec![];
		while !old_ids_to_remove.is_empty()
		{
			new_ids_to_remove.clear();
			for obj in objects.elems()
			{
				for &dead_id in &old_ids_to_remove
				{
					if obj.parent == dead_id
					{
						state.remove_object(obj.get_id());
						new_ids_to_remove.push(obj.get_id());
					}
				}
			}
			old_ids_to_remove.clear();
			old_ids_to_remove.extend(&new_ids_to_remove);
		}
	}
}
