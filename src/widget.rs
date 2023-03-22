#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::vec::Vec;
use std::rc::{Rc, Weak};
use crate::tree;


pub struct WidgetObj
{
	tree : Tree<Box<dyn Widget>,
}

impl WidgetObj
{
	pub fn new(parent : Option<&Rc<RefCell<Box<dyn Widget>>>>, id : &str) -> Rc<RefCell<Box<dyn Widget>>>
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

		let mut result : Rc<RefCell<Box<dyn Widget>>> = Rc::new(RefCell::new(Box::new(
		WidgetObj
		{
			parent :  weak_parent,
			children : Vec::new(),
			id : String::from(id),
		})));

		if parent.is_some()
		{
			parent.unwrap().borrow_mut().children().push(Rc::clone(&mut result));
		}

		result
	}

	pub fn id(&self) -> &String
	{
		&self.id
	}
}

trait CmpWidget
{
	fn as_any(&self) -> &dyn Any;
	fn equals(&self, other : &dyn Widget) -> bool;
}

//impl<S: 'static + PartialEq> CmpWidget for S 
/*impl CmpWidget for WidgetObj 
{
    fn as_any(&self) -> &dyn Any 
    {
        self
    }

    fn equals(&self, other: &dyn Widget) -> bool 
    {
        other.as_any()
            .downcast_ref::<WidgetObj>()
            .map_or(false, |a| self == a)
    }
}*/

pub trait Widget
{	
	fn parent(&self) -> Option<Rc<RefCell<Box<dyn Widget>>>>;
	fn set_parent(&mut self, parent : Option<Rc<RefCell<Box<dyn Widget>>>>);
	fn children(&mut self) -> &mut Vec<Rc<RefCell<Box<dyn Widget>>>>;
	fn as_any(&self) -> &dyn Any;
}


impl Widget for WidgetObj
{
	fn parent(&self) -> Option<Rc<RefCell<Box<dyn Widget>>>>
	{
		self.parent.upgrade()
	}

	fn set_parent(&mut self, parent : Option<Rc<RefCell<Box<dyn Widget>>>>)
	{
		self.parent = match parent
		{
			Some(parent) => Rc::downgrade(&parent),
			None => Weak::new()
		};
	}

	fn children(&mut self) -> &mut Vec<Rc<RefCell<Box<dyn Widget>>>>
	{
		&mut self.children
	}

    fn as_any(&self) -> &dyn Any 
    {
        self
    }	
}



fn type_of<T>(_: &T) 
{
    println!("{}", std::any::type_name::<T>())
}
/*
fn eq<T: Any + Eq, Q: Any + Eq>(a: &T, b: &Q) -> bool 
{
    if TypeId::of::<T>() == TypeId::of::<Q>() 
    {
        let b_as_t = &b as &dyn Any;
        // safe to unwrap, we matched the type already
        a == b_as_t.downcast_ref::<T>().unwrap()
    } else 
    {
        false    
    }
}*/

/*
#[cfg(test)]
mod tests 
{
	use super::*;
    //use crate::w::{ArenaIndex, Arena};

	#[test]
    fn widget_new() 
    {
    	let mut root = WidgetObj::new(None, "root");
    	let mut child1 = WidgetObj::new(Some(&root), "child1");
    	let mut child2 = WidgetObj::new(Some(&root), "child2");
    	let mut child3 = WidgetObj::new(Some(&root), "child3");
 		

 		
    	{
 		assert_eq!(root.borrow_mut().parent().is_none(), true);
 		assert_eq!(child1.borrow_mut().parent().is_some(), true);
 		}
 		//assert_eq!(child1.borrow_mut().parent().unwrap().borrow().as_any(), root.borrow().as_any());

 		// let e = child1.borrow_mut().parent().unwrap().get_mut().downcast::<WidgetObj>() ==
 		// 		root.get_mut().downcast::<WidgetObj>();
 		// assert_eq!(e, true);

 		// let b1 = child1.borrow_mut();
 		// let b2 = b1.parent().unwrap();
 		// let b3 = b2.borrow_mut();
 		// let c1 = &b3.deref().deref();
 		// println!("{}", root.borrow_mut().children().len());

 		//let binding = root.borrow_mut();
   		//let r = &binding.deref().deref();
 		//type_of(c1);
 		//type_of(r);
 		

 		//assert_eq!(root.borrow_mut().children().len(), 3);
 	}

}*/
