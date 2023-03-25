#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::vec::Vec;
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::any::Any;


pub struct TreeNode<T>
{
	parent : Weak<RefCell<TreeNode<T>>>,
	children : Vec<Rc<RefCell<TreeNode<T>>>>,
	data : T,
}

impl<T> TreeNode<T>
{
	pub fn new_root(data : T) -> Rc<RefCell<Self>>
	{
		Rc::new(RefCell::new(
			Self
			{
				parent : Weak::new(),
				children : Vec::new(),
				data,
			}))		
	}

	pub fn new(parent : &mut Rc<RefCell<Self>>, data : T) -> Rc<RefCell<Self>>
	{
		let child = Rc::new(RefCell::new(
			Self
			{
				parent : Rc::downgrade(parent),
				children : Vec::new(),
				data,
			}));
				
		//let parentnode = parent.borrow_mut();
		parent.borrow_mut().add_child(Rc::clone(&child));					

		child
	}

	fn parent(&self) -> Option<Rc<RefCell<Self>>>
	{
		self.parent.upgrade()
	}

	fn set_parent(&mut self, parent : Option<Rc<RefCell<Self>>>)
	{
		self.parent = match parent
		{
			Some(parent) => Rc::downgrade(&parent),
			None => Weak::new()
		};
	}

	fn children(&self) -> &Vec<Rc<RefCell<TreeNode<T>>>>
	{
		&self.children
	}

	fn children_mut(&mut self) -> &mut Vec<Rc<RefCell<TreeNode<T>>>>
	{
		&mut self.children
	}	

	fn add_child(&mut self, child : Rc<RefCell<Self>>)
	{
		self.children.push(child);
	}

}

struct WidgetObj;

#[cfg(test)]
mod tests 
{
	use super::*;


	#[test]
    fn tree_new() 
    {
    	let mut root = TreeNode::new_root(WidgetObj);
    	let child1 = TreeNode::new(&mut root, WidgetObj);
    	let child2 = TreeNode::new(&mut root, WidgetObj);
    	let child3 = TreeNode::new(&mut root, WidgetObj);
 		
 		assert_eq!(root.borrow().children().len(), 3);
    }
}