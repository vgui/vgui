#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::hash::Hash;
use std::vec::Vec;
use std::collections::HashSet;
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::any::Any;
use std::ops::Deref;


struct UniqueVec<'a, T>
{
	vec : Vec<T>,
	set : &'a mut HashSet<T>
}


impl<'a, T : Eq + PartialEq + Hash + Clone + 'a> UniqueVec<'a, T> 
{
	pub fn new(set : &'a mut HashSet<T>) -> Self
	{
		UniqueVec
		{
			vec : Vec::new(),
			set : set,
		}
	}

	pub fn find(&mut self, value : T) -> usize
	{
		self.vec.iter().position(|&v| &v == value).unwrap()
	}

	pub fn insert(&mut self, index : usize, value : T)
	{
		if self.set.insert(value.clone())
		{
			self.vec.insert(index, value.clone());
		}
	}
 }

#[cfg(test)]
mod tests 
{	
	use super::*;

	#[derive(Clone, Eq, Hash, PartialEq)]
	struct RcRefCell<T>(Rc<RefCell<T>>);

	impl<T> RcRefCell<T>
	{
		pub fn new(arg : T) -> Self
		{
			RcRefCell
			{
				0 : Rc::new(RefCell::new(arg))
			}
		}
	}

	impl<T> Deref for RcRefCell<T> 
	{
	    type Target = Rc<RefCell<T>>;

	    fn deref(&self) -> &Self::Target 
	    {
	        &self.0
	    }
	}

	struct WidgetObj
	{
		id : String,
	}

	impl WidgetObj
	{
		pub fn new(id : &str) -> Self
		{
			WidgetObj { id : String::from(id), }
		}
	}

	#[test]
	pub fn unique_vec_find()
	{
		let w1 = RcRefCell::new(WidgetObj::new("widget1"));
		let w1 = RcRefCell::new(WidgetObj::new("widget1"));


		let mut set = HashSet::<RcRefCell<WidgetObj>>::new();
		let vec = UniqueVec::<RcRefCell<WidgetObj>>::new(&mut set);		
	}
}