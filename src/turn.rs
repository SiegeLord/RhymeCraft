use engine::id_map::HasId;
use path::order_unit;
use util::l1_dist;
use announce::create_announcement;

pub struct TurnLogic;

complex_behavior!
{
	TurnLogic[obj.is_map] |self, obj, objects, state|
	{
		let map_data = objects.get(state.current_map_id).map(|obj| obj.map_data.clone());
		if let Some(ref map_data) = map_data
		{
			let mut map_data = map_data.borrow_mut();
			if !map_data.our_turn && !map_data.executing_orders
			{
				let mut object_to_order = None;
				let mut order_location = None;
				for enemy_obj in objects.elems()
				{
					if !enemy_obj.is_ours && enemy_obj.can_act && enemy_obj.action_points > 0
					{
						for our_obj in objects.elems()
						{
							if our_obj.is_ours && our_obj.can_act
							{
								object_to_order = Some(enemy_obj.get_id());
								let cand_dist = l1_dist(our_obj.tile_x, our_obj.tile_y, enemy_obj.tile_x, enemy_obj.tile_y);
								if cand_dist < enemy_obj.sight_range
								{
									if let Some((x, y)) = order_location
									{
										if cand_dist < l1_dist(x, y, enemy_obj.tile_x, enemy_obj.tile_y)
										{
											order_location = Some((our_obj.tile_x, our_obj.tile_y));
										}
									}
									else
									{
										order_location = Some((our_obj.tile_x, our_obj.tile_y));
									}
								}
							}
						}
						if order_location.is_some() && object_to_order.is_some()
						{
							break;
						}
					}
				}
				
				if let (Some(object_to_order), Some((order_x, order_y))) = (object_to_order, order_location)
				{
					order_unit(object_to_order, objects, order_x, order_y, &mut *map_data);
					let mut obj = objects.get_mut(object_to_order).unwrap();
					obj.executing_orders = true;
					map_data.executing_orders = true;
				}
				else
				{
					for obj in objects.elems_mut()
					{
						if obj.can_act
						{
							obj.action_points = obj.max_action_points;
						}
					}
					map_data.our_turn = true;
					map_data.turn += 1;
					map_data.mana += 2 + map_data.num_circles_held;
					let announce = create_announcement(state.current_map_id, &format!("Turn {} begins!", map_data.turn), state);
					state.add_object(announce);
				}
			}
		}
	}
}

