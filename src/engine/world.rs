// Copyright 2015 SiegeLord
//
// See LICENSE for terms.

use std::collections::HashSet;

use engine::id_map::{HasId, IdMap};

pub trait Behavior<O, S>
{
	fn check_object(&self, &O) -> bool
	{
		true
	}
	
	fn handle_objects(&mut self, objects: &mut IdMap<O>, state: &mut S);
}

pub trait WorldState<Object>
{
	fn get_new_objects(&mut self) -> &mut Vec<Object>;
	fn get_ids_to_remove(&mut self) -> &mut HashSet<usize>;
}

pub struct World<Object, State>
{
	objects: IdMap<Object>,
	logic_behaviors: Vec<Box<Behavior<Object, State>>>,
	input_behaviors: Vec<Box<Behavior<Object, State>>>,
	draw_behaviors: Vec<Box<Behavior<Object, State>>>,
	pub state: State,
}

impl<Object, State> World<Object, State> where Object: HasId, State: WorldState<Object>
{
	pub fn new(state: State) -> World<Object, State>
	{
		World
		{
			state: state,
			objects: IdMap::new(),
			logic_behaviors: vec![],
			input_behaviors: vec![],
			draw_behaviors: vec![],
		}
	}
	
	pub fn add_logic_behavior(&mut self, behavior: Box<Behavior<Object, State>>)
	{
		self.logic_behaviors.push(behavior);
	}
	
	pub fn logic(&mut self)
	{
		for behavior in &mut self.logic_behaviors
		{
			behavior.handle_objects(&mut self.objects, &mut self.state);
		}
		
		for obj in self.state.get_new_objects().drain(..)
		{
			self.objects.insert(obj);
		}
		
		for id in self.state.get_ids_to_remove().drain()
		{
			if self.objects.get(id).is_some()
			{
				self.objects.remove(id);
			}
		}
	}
	
	pub fn add_input_behavior(&mut self, behavior: Box<Behavior<Object, State>>)
	{
		self.input_behaviors.push(behavior);
	}
	
	pub fn input(&mut self)
	{
		for behavior in &mut self.input_behaviors
		{
			behavior.handle_objects(&mut self.objects, &mut self.state);
		}
	}
	
	pub fn add_draw_behavior(&mut self, behavior: Box<Behavior<Object, State>>)
	{
		self.draw_behaviors.push(behavior);
	}
	
	pub fn draw(&mut self)
	{
		for behavior in &mut self.draw_behaviors
		{
			behavior.handle_objects(&mut self.objects, &mut self.state);
		}
	}
}
