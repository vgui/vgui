#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


use std::vec::Vec;
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::any::Any;


pub struct WidgetObj
{
	parent : Weak<Box<dyn Widget>>,
	children : Vec<Rc<Box<dyn Widget>>>,
	id : String,
}

impl WidgetObj
{
	pub fn new(parent : Option<Rc<Box<dyn Widget>>>, id : &str) -> Self
	{
		let parent = match parent
		{
			Some(parent) => Rc::downgrade(&parent),
			None => Weak::new()
		};

		WidgetObj
		{
			parent :  parent,
			children : Vec::new(),
			id : String::from(id),
		}
	}
}

pub trait Widget
{	
	fn parent(&self) -> Option<Rc<Box<dyn Widget>>>;
	fn set_parent(&mut self, parent : Option<Rc<Box<dyn Widget>>>);
	fn children(&self) -> &Vec<Rc<Box<dyn Widget>>>;
}


impl Widget for WidgetObj
{
	fn parent(&self) -> Option<Rc<Box<dyn Widget>>>
	{
		self.parent.upgrade()
	}

	fn set_parent(&mut self, parent : Option<Rc<Box<dyn Widget>>>)
	{
		self.parent = match parent
		{
			Some(parent) => Rc::downgrade(&parent),
			None => Weak::new()
		};
	}

	fn children(&self) -> &Vec<Rc<Box<dyn Widget>>>
	{
		&self.children
	}
}


#[cfg(test)]
mod tests 
{
	use super::*;
    use crate::arena::{ArenaIndex, Arena};



}