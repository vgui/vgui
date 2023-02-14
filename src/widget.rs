#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

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
	pub fn new(parent : Option<&Rc<Box<dyn Widget>>>, id : &str) -> Rc<Box<dyn Widget>>
	{
		let weak_parent = match parent
		{
			Some(parent) => 
			{
				let parent = Rc::clone(&parent);
				Rc::downgrade(&parent)
			},
			None => Weak::new()
		};

		let mut result : Rc<Box<dyn Widget>> = Rc::new(Box::new(
		WidgetObj
		{
			parent :  weak_parent,
			children : Vec::new(),
			id : String::from(id),
		}));

		// if parent.is_some()
		// {
		// 	parent.unwrap().children().push(Rc::clone(&mut result));
		// }

		result
	}
}

pub trait Widget
{	
	fn parent(&self) -> Option<Rc<Box<dyn Widget>>>;
	fn set_parent(&mut self, parent : Option<Rc<Box<dyn Widget>>>);
	fn children(&mut self) -> &mut Vec<Rc<Box<dyn Widget>>>;
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

	fn children(&mut self) -> &mut Vec<Rc<Box<dyn Widget>>>
	{
		&mut self.children
	}
}




#[cfg(test)]
mod tests 
{
	use super::*;
    //use crate::w::{ArenaIndex, Arena};


	#[test]
    fn widget_new() 
    {
    	let root = WidgetObj::new(None, "root");
    	let child1 = WidgetObj::new(Some(&root), "child1");
    	let child2 = WidgetObj::new(Some(&root), "child2");
    	let child3 = WidgetObj::new(Some(&root), "child3");
 		
 		assert_eq!(root.parent().is_none(), true);
 		assert_eq!(child1.parent().is_some(), true);
 		//assert_eq!(child1.parent().unwrap(), root);
 		//assert_eq!(root.children().len(), 3);
 	}

}