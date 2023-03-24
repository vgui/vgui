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

    // pub unsafe fn get_mut_unchecked(this: &mut Self) -> &mut T {
    //     // We are careful to *not* create a reference covering the "count" fields, as
    //     // this would conflict with accesses to the reference counts (e.g. by `Weak`).
    //     unsafe { &mut (*this.ptr.as_ptr()).value }
    // }

	pub fn new(parent : &mut Rc<Self>, data : T) -> Rc<Self>
	{
		println!("Weak count {}", Rc::weak_count(&parent));
		
		let msg = format!("Strong count {}", Rc::strong_count(&parent));

		let child = Rc::new(
			Self
			{
				parent : Rc::downgrade(parent),
				children : Vec::new(),
				data,
			});

		{			
			//let parentnode = Rc::get_mut(parent).expect(msg.as_str());		
			//parentnode.add_child(Rc::clone(&child));			
			unsafe 
			{
				let ptr = Rc::as_ptr(&parent);
				(*(ptr as *mut Self)).children.push(Rc::clone(&child));// = Rc::downgrade(parent);
			}
		}

		child
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
		self.children.push(child);
	}

	fn update_lastchild_parent(&mut self)
	{
		self.parent = Rc::downgrade(self.children.last().unwrap());
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