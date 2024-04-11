#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::vec::Vec;
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::any::Any;


pub struct Tree<'a, T>
{
	weak_self : Option<&'a mut Box<Self>>,
	parent : Option<&'a mut Self>,
	children : Vec<Box<Self>>,
	childindex : usize,
	data : T,
}

impl<'a, T> Tree<'a, T>
{
    fn update_indexes(&mut self, start_index : usize)
    {
    	let mut index : usize = start_index;

        while index < self.children.len()
        {
            self.children[index].childindex = index;
            index += 1
        }
    }

	pub fn new(parent : Option<&'a mut Self>,childindex : usize , data : T) -> Box<Self>
	{
		let child = Box::new(
			Self
			{
				weak_self : None,
				parent : None,
				children : Vec::new(),
				childindex : usize::MAX,
				data,
			});
		
		//child.weak_self = Some(&child);

		if parent.is_some()
		{			
			let mut childindex = childindex;

			if childindex == usize::MAX
			{
				childindex = parent.unwrap().children.len();	
			}

			parent.unwrap().children.insert(childindex, child);
			parent.unwrap().update_indexes(childindex);
			child.parent = parent;
		}
		
		child
	}

    pub fn remove(&mut self, childindex : usize) -> Box<Self>
    {
    	//Check child index.
        if childindex >= self.children.len()
        {
            panic!("Too big index for removing.");
        }
      
       	let child = self.children.remove(childindex);
       	child.parent = None;
       	child.childindex = usize::MAX;        
        self.update_indexes(childindex);

        child
    }

    pub fn insert(&mut self,  childindex : usize, mut child : Box<Self>)
    {
    	//Check child index.
		let mut childindex = childindex;

		if childindex == usize::MAX
		{
			childindex = self.children.len();	
		}

        if childindex > self.children.len()
        {
            panic!("Too big index for inserting.");
        }

        //If child have parent, remove child from parent using child index.
        if let Some(parent) = child.parent()
        {
	       	parent.remove(child.childindex);
        }

        //Set parent for child.
        child.parent = Some(&mut self);

        //Insert child to children and update indexes.
        self.children.insert(childindex, child);        
        self.update_indexes(childindex);
    }

	pub fn set_parent(&mut self, newparent : Option<&Box<Self>>, childindex : usize)
	{
		if let Some(parent) = self.parent()
		{
			parent.remove(self.childindex());
		}

		if let Some(newparent) = newparent
		{
			newparent.insert(childindex, *self.weak_self.unwrap());			
		}
	}

	pub fn parent(&self) -> Option<&'a mut Self>
	{
		self.parent
	}

	pub fn child(&'a self, index : usize) -> Option<&'a Box<Self>>
	{
		self.children.get(index)
	}

	pub fn childindex(&self) -> usize
	{
		self.childindex
	}

	pub fn children_count(&self) -> usize
	{
		self.children.len()
	}	

	pub fn data(&self) -> &T
	{
		&self.data
	}	

	pub fn data_mut(&mut self) -> &mut T
	{
		&mut self.data
	}		
}
