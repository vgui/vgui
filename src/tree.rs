#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


use core::marker::PhantomData;
use std::vec::Vec;
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::any::Any;


pub struct Node<T>
{
	parent : Weak<Node<T>>,
	children : Vec<Rc<Node<T>>>,
	data : T,
	//_marker : PhantomData<T>,
}

impl<T> Node<T>
{
	pub fn new_root(data : T) -> Rc<Self>
	{
		Rc::new(
			Self
			{
				parent : Weak::new(),
				children : Vec::new(),
				data,
			})		
	}

	pub fn new(mut parent : Rc<Self>, data : T) -> Rc<Self>
	{
		println!("Strong count {}", Rc::strong_count(&parent));
		
		let msg = format!("Strong count {}", Rc::strong_count(&parent));
		let weakparent = Rc::downgrade(&parent);
		let parentnode =Rc::get_mut(&mut parent).expect(msg.as_str());
		
		let child =Rc::new(
			Self
			{
				parent : weakparent,
				children : Vec::new(),
				data,
			});

		parentnode.add_child(Rc::clone(&child));

		Rc::clone(&child)	
	}

	fn parent(&self) -> Option<Rc<Self>>
	{
		self.parent.upgrade()
	}

	fn set_parent(&mut self, parent : Option<Rc<Self>>)
	{
		self.parent = match parent
		{
			Some(parent) => Rc::downgrade(&parent),
			None => Weak::new()
		};
	}

	fn children(&self) -> &Vec<Rc<Node<T>>>
	{
		&self.children
	}

	fn children_mut(&mut self) -> &mut Vec<Rc<Node<T>>>
	{
		&mut self.children
	}	

	fn add_child(&mut self, child : Rc<Self>)
	{
		self.children.push(child)
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
    	let mut root = Node::new_root(WidgetObj);
    	let child1 = Node::new(Rc::clone(&root), WidgetObj);
    	let child2 = Node::new(Rc::clone(&root), WidgetObj);
    	let child3 = Node::new(Rc::clone(&root), WidgetObj);
 		
 		assert_eq!(root.children().len(), 3);
    }
}