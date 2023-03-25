#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::vec::Vec;
use std::any::Any;


pub struct Node<'a, T>
{
	parent : Option<&'a Node<'a, T>>,
	children : Vec<Box<Node<'a, T>>>,
	data : T,	
}

impl<'a, T> Node<'a, T>
{
	pub fn new_root(data : T) -> Box<Self>
	{
		Box::new(
			Self
			{
				parent : None,
				children : Vec::new(),
				data,
			})		
	}

	pub fn new(p : &'static mut Self, data : T) -> &Box<Self>
	{
		let child = Box::new(
			Self
			{
				parent : Some(&p),
				children : Vec::new(),
				data,
			});

		p.children.push(child);
    p.children.last().unwrap()
	}

	fn parent(&self) -> Option<&Node<T>>
	{
		self.parent
	}

	fn set_parent(&'a mut self, parent : Option<&Node<T>>)
	{
		self.parent = parent
	}

	fn children(&'a self) -> &Vec<Box<Node<T>>>
	{
		&self.children
	}

	fn children_mut(&mut self) -> &mut Vec<Box<Self>>
	{
		&mut self.children
	}	

	fn add_child(&mut self, mut child : Self)
	{
    child.parent = Some(self);
		self.children.push(Box::new(child))
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
    	let child1 = Node::new(&mut root, WidgetObj);
    	let child2 = Node::new(&mut root, WidgetObj);
    	let child3 = Node::new(&mut root, WidgetObj);
 		
 		assert_eq!(root.children().len(), 3);
    }
}