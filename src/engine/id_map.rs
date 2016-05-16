// Copyright 2015 SiegeLord
//
// See LICENSE for terms.

use std::collections::HashMap;

// Wrapper type to prevent giving the same id to multiple objects
pub struct UniqueId(usize);

impl UniqueId
{
	pub fn empty() -> UniqueId
	{
		UniqueId(0)
	}

	pub fn get(&self) -> usize
	{
		self.0
	}
}

pub trait HasId
{
	fn get_id(&self) -> usize;
}

pub struct IdMint
{
	next_id: usize,
}

impl IdMint
{
	pub fn new() -> IdMint
	{
		IdMint
		{
			next_id: 1,
		}
	}

	// Yeah, you could just make a new IdMint and screw everything up... don't do it.
	pub fn new_id(&mut self) -> UniqueId
	{
		let ret = self.next_id;
		self.next_id += 1;
		UniqueId(ret)
	}
}

pub struct IdMap<T>
{
	elems: Vec<T>,
	id_to_idx: HashMap<usize, usize>,
}

impl<T: HasId> IdMap<T>
{
	pub fn new() -> IdMap<T>
	{
		IdMap
		{
			elems: vec![],
			id_to_idx: HashMap::new(),
		}
	}

	pub fn insert(&mut self, e: T)
	{
		assert!(e.get_id() > 0);
		assert!(!self.id_to_idx.contains_key(&e.get_id()));
		self.id_to_idx.insert(e.get_id(), self.elems.len());
		self.elems.push(e);
	}

	pub fn remove(&mut self, id: usize)
	{
		let idx = self.id_to_idx[&id];
		// This element will be moved to the idx.
		*self.id_to_idx.get_mut(&self.elems.last().unwrap().get_id()).unwrap() = idx;
		self.elems.swap_remove(idx);
		self.id_to_idx.remove(&id);
	}

	pub fn get(&self, id: usize) -> Option<&T>
	{
		self.id_to_idx.get(&id).map(|&idx| &self.elems[idx])
	}

	pub fn get_mut(&mut self, id: usize) -> Option<&mut T>
	{
		match self.id_to_idx.get(&id)
		{
			Some(&idx) => Some(&mut self.elems[idx]),
			None => None
		}
	}

	pub fn len(&self) -> usize
	{
		self.elems.len()
	}

	pub fn elems(&self) -> &[T]
	{
		&self.elems
	}

	// XXX: Err... don't call swap on this.
	pub fn elems_mut(&mut self) -> &mut [T]
	{
		&mut self.elems
	}
}

#[test]
fn basic()
{
	impl HasId for i32
	{
		fn get_id(&self) -> usize
		{
			*self as usize
		}
	}

	let mut map = IdMap::<i32>::new();
	map.insert(1);
	map.insert(2);
	assert_eq!(1, *map.get(1).unwrap());
	assert_eq!(2, *map.get(2).unwrap());
	assert_eq!(2, map.len());
	map.remove(1);
	assert_eq!(2, *map.get(2).unwrap());
	assert_eq!(1, map.len());
	assert_eq!(2, *map.get(2).unwrap());
	map.insert(3);
	assert_eq!(3, *map.get(3).unwrap());
	assert_eq!(2, map.len());
}
