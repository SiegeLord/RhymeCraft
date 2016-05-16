use engine::id_map::HasId;
use game_state::{GameState, Object};
use animation::Animation;

pub fn create_slash(parent: usize, x: f32, y: f32, state: &mut GameState) -> Object
{
	let mut obj = Object::new(state.new_id());
	obj.parent = parent;
	obj.x = x;
	obj.y = y;
	obj.has_pos = true;
	obj.has_sprite = true;
	obj.sprite = Some(Animation::new("data/slash.cfg", true, state));
	obj.is_effect = true;
	obj.effect_death_time = state.time + 1.0;
	obj
}

pub fn create_fire(parent: usize, x: f32, y: f32, state: &mut GameState) -> Object
{
	let mut obj = Object::new(state.new_id());
	obj.parent = parent;
	obj.x = x;
	obj.y = y;
	obj.has_pos = true;
	obj.has_sprite = true;
	obj.sprite = Some(Animation::new("data/fire.cfg", true, state));
	obj.is_effect = true;
	obj.effect_death_time = state.time + 1.0;
	obj
}

pub fn create_death(parent: usize, x: f32, y: f32, state: &mut GameState) -> Object
{
	let mut obj = Object::new(state.new_id());
	obj.parent = parent;
	obj.x = x;
	obj.y = y;
	obj.has_pos = true;
	obj.has_sprite = true;
	obj.sprite = Some(Animation::new("data/death.cfg", true, state));
	obj.is_effect = true;
	obj.effect_death_time = state.time + 1.0;
	obj
}

pub fn create_spawn(parent: usize, x: f32, y: f32, state: &mut GameState) -> Object
{
	let mut obj = Object::new(state.new_id());
	obj.parent = parent;
	obj.x = x;
	obj.y = y;
	obj.has_pos = true;
	obj.has_sprite = true;
	obj.sprite = Some(Animation::new("data/spawn.cfg", true, state));
	obj.is_effect = true;
	obj.effect_death_time = state.time + 1.0;
	obj
}

simple_behavior!
{
	EffectDraw[obj.has_sprite && obj.has_pos && obj.is_effect] |obj, state|
	{
		obj.sprite.as_ref().unwrap().draw(obj.x - 16.0, obj.y - 16.0, state);
	}
}

simple_behavior!
{
	EffectLogic[obj.is_effect] |obj, state|
	{
		if state.time > obj.effect_death_time
		{
			state.remove_object(obj.get_id());
		}
	}
}
