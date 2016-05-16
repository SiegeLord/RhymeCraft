use std::collections::{HashMap, BinaryHeap};
use engine::id_map::{IdMap, HasId};
use map::map_tile_solid;
use game_state::*;
use effect::{create_slash, create_fire};
use util::l1_dist;

pub struct PathableInput;

pub fn order_unit(obj_id: usize, objects: &mut IdMap<Object>, goal_x: i32, goal_y: i32, map_data: &mut MapData)
{
	let mut attack_order = false;
	let order_ours = objects.get(obj_id).unwrap().is_ours;
	for obj in objects.elems_mut()
	{
		if obj.has_health && order_ours != obj.is_ours && obj.has_pos
		{
			if obj.tile_x == goal_x && obj.tile_y == goal_y
			{
				attack_order = true;
				break;
			}
		}
	}
	
	let obj = objects.get_mut(obj_id).unwrap();
	make_path(obj, goal_x, goal_y, &mut *map_data);
	let last_pos = obj.orders.last().map_or((obj.tile_x, obj.tile_y), |o| (o.x, o.y));
	if obj.orders.len() < obj.action_points as usize && attack_order && l1_dist(goal_x, goal_y, last_pos.0, last_pos.1) == 1
	{
		obj.orders.push(Order
		{
			order_type: OrderType::Attack,
			x: goal_x,
			y: goal_y,
		});
	}
}

fn make_path(obj: &mut Object, goal_x: i32, goal_y: i32, map_data: &mut MapData)
{
	obj.orders.clear();
	
	#[derive(Ord, PartialOrd, PartialEq, Eq)]
	struct Node
	{
		// Since this is a max heap, we'll store negative scores.
		neg_cost: i32,
		x: i32,
		y: i32,
	}
	#[derive(Ord, PartialOrd, PartialEq, Eq)]
	struct ApproxNode
	{
		// Since this is a max heap, we'll store negative scores.
		neg_approx_cost: i32,
		neg_true_cost: i32,
		x: i32,
		y: i32,
	}
	
	let mut open_set = BinaryHeap::new();
	let mut cost_map = HashMap::new();
	let mut approx_cost_heap = BinaryHeap::new();
	let mut came_from = HashMap::new();
	open_set.push(Node
	{
		neg_cost: 0,
		x: obj.tile_x,
		y: obj.tile_y,
	});
	approx_cost_heap.push(ApproxNode
	{
		neg_approx_cost: -l1_dist(obj.tile_x, obj.tile_y, goal_x, goal_y),
		neg_true_cost: 0,
		x: obj.tile_x,
		y: obj.tile_y,
	});
	cost_map.insert((obj.tile_x, obj.tile_y), 0);
	
	info!("Searching for {} {} from {} {}", goal_x, goal_y, obj.tile_x, obj.tile_y);
	let mut found = None;
	while !open_set.is_empty()
	{
		let best = open_set.pop().unwrap();
		let cost = *cost_map.get(&(best.x, best.y)).unwrap();
		if best.x == goal_x && best.y == goal_y
		{
			found = Some((best.x, best.y));
			break;
		}
		for m in &[(-1, 0), (1, 0), (0, -1), (0, 1)]
		{
			let cand_x = best.x + m.0;
			let cand_y = best.y + m.1;
			let cand_cost: i32 = cost + 1;
			info!("trying: {} {} at {}", cand_x, cand_y, cand_cost);
			if cost_map.get(&(cand_x, cand_y)).map_or(true, |&v| cand_cost < v) &&
				!map_tile_solid(cand_x, cand_y, &*map_data) && cand_cost <= obj.action_points
			{
				let approx_cost = l1_dist(goal_x, goal_y, cand_x, cand_y);
				open_set.push(Node
				{
					neg_cost: -(cand_cost + approx_cost),
					x: cand_x,
					y: cand_y,
				});
				approx_cost_heap.push(ApproxNode
				{
					neg_approx_cost: -approx_cost,
					neg_true_cost: -cand_cost,
					x: cand_x,
					y: cand_y,
				});
				cost_map.insert((cand_x, cand_y), cand_cost);
				came_from.insert((cand_x, cand_y), (best.x, best.y));
			}
		}
	}

	let mut cur;
	match found
	{
		Some(found) => cur = found,
		None =>
		{
			if let Some(approx_best) = approx_cost_heap.pop()
			{
				cur = (approx_best.x, approx_best.y);
				info!("Got {:?}", cur);
			}
			else
			{
				info!("Got nothing");
				return;
			}
		}
	}
	loop
	{
		if cur.0 == obj.tile_x && cur.1 == obj.tile_y
		{
			break;
		}
		obj.orders.push(Order{ x: cur.0, y: cur.1, order_type: OrderType::MoveTo });
		cur = *came_from.get(&cur).unwrap();
	}
	obj.orders.reverse();
	for &o in &obj.orders
	{
		info!("{:?}", o);
	}
	
	info!("Done: {}", obj.orders.len());
}

complex_behavior!
{
	PathableInput[obj.is_selectable && obj.has_pos && obj.selected && obj.can_act && obj.is_ours] |self, obj, objects, state|
	{
		let button = state.mouse_button_down.unwrap_or(0);
		if button != 2
		{
			return;
		}
		let map_data = objects.get(state.current_map_id).map(|obj| obj.map_data.clone());
		if let Some(ref map_data) = map_data
		{
			let mut map_data = map_data.borrow_mut();
			
			if map_data.executing_orders || !map_data.our_turn
			{
				return;
			}
			let mut executing_orders = None;
			let mut adding_orders = None;
			for obj in objects.elems_mut()
			{
				if !self.check_object(obj) || obj.action_points == 0
				{
					continue;
				}
				
				if let Some(last_order) = obj.orders.last()
				{
					if last_order.x == state.cursor_x && last_order.y == state.cursor_y
					{
						obj.executing_orders = true;
						executing_orders = Some(obj.get_id())
					}
				}
				if executing_orders.is_none() && (obj.tile_x != state.cursor_x || obj.tile_y != state.cursor_y)
				{
					adding_orders = Some(obj.get_id());
				}
				break;
			}
			if let Some(orders_id) = executing_orders
			{
				map_data.executing_orders = true;
				for obj in objects.elems_mut()
				{
					if obj.get_id() != orders_id
					{
						obj.orders.clear();
					}
				}
			}
			else if let Some(orders_id) = adding_orders
			{
				order_unit(orders_id, objects, state.cursor_x, state.cursor_y, &mut *map_data);
			}
		}
	}
}

pub struct OrdersLogic;

complex_behavior!
{
	OrdersLogic[obj.has_pos && obj.can_act && obj.has_pos] |self, obj, objects, state|
	{
		let map_data = objects.get(state.current_map_id).map(|obj| obj.map_data.clone());
		if let Some(ref map_data) = map_data
		{
			let mut map_data = map_data.borrow_mut();
			let mut damage_this = None;
			for obj in objects.elems_mut()
			{
				if !self.check_object(obj) || !obj.executing_orders
				{
					continue;
				}
				if let Some(&order) = obj.orders.first()
				{
					let dest_x = order.x as f32 * 32.0;
					let dest_y = order.y as f32 * 32.0;
					match order.order_type
					{
						OrderType::MoveTo =>
						{
							const SPEED: f32 = 256.0;
							if (obj.x - dest_x).abs() < SPEED * DT * 1.1 && (obj.y - dest_y).abs() < SPEED * DT * 1.1
							{
								obj.x = dest_x;
								obj.y = dest_y;
								obj.orders.remove(0);
								obj.action_points -= 1;
							}
							if obj.x < dest_x
							{
								obj.x += SPEED * DT;
							}
							if obj.x > dest_x
							{
								obj.x -= SPEED * DT;
							}
							if obj.y < dest_y
							{
								obj.y += SPEED * DT;
							}
							if obj.y > dest_y
							{
								obj.y -= SPEED * DT;
							}
						},
						OrderType::Attack =>
						{
							if obj.fire
							{
								let slash = create_fire(state.current_map_id, dest_x, dest_y, state);
								state.add_object(slash);
							}
							else
							{
								let slash = create_slash(state.current_map_id, dest_x, dest_y, state);
								state.add_object(slash);
							}
							obj.orders.remove(0);
							obj.action_points = 0;
							damage_this = Some((order.x, order.y, -obj.damage));
						}
					}
				}
				else
				{
					obj.executing_orders = false;
					if !obj.is_ours
					{
						// HACK!
						obj.action_points = 0;
					}
					map_data.executing_orders = false;
				}
			}
			if let Some((x, y, change)) = damage_this
			{
				for obj in objects.elems_mut()
				{
					if obj.has_health && obj.tile_x == x && obj.tile_y == y
					{
						obj.health += change;
					}
				}
			}
		}
	}
}

simple_behavior!
{
	PathableDraw[obj.has_pos && obj.can_act && obj.is_selectable] |obj, state|
	{
		if !obj.selected
		{
			continue;
		}
		let path_len = obj.orders.len();
		for (count, &order) in obj.orders.iter().enumerate()
		{
			if obj.executing_orders && count == 0
			{
				continue;
			}
			let ani = match order.order_type
			{
				OrderType::MoveTo =>
					if count == path_len - 1
					{
						state.path_end.as_ref().unwrap()
					}
					else
					{
						state.path_marker.as_ref().unwrap()
					},
				OrderType::Attack =>
				{
					state.path_attack.as_ref().unwrap()
				}
			};
			ani.draw(order.x as f32 * 32.0 - 16.0, order.y as f32 * 32.0 - 16.0, state);
		}
	}
}
