/*
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::ops::DerefMut;
use std::vec::Vec;

pub struct TreeNode<'a, T>
{
	this : Option<&'a mut Self>,
	parent : Option<&'a mut Self>,
	children : Vec<&'a mut Self>,
	childindex : usize,
	data : T,
}

struct ArenaObj<'a, T>
{
	arena : Vec<Box<TreeNode<'a, T>>>,
}

trait Arena<'a, T>
{
	fn new() -> ArenaObj<'a, T>;
	fn alloc(&mut self, parent : Option<&'a mut Box<TreeNode<'a, T>>>, childindex : usize , data : T) -> &mut TreeNode<'a, T>;
	// fn free(&mut self, obj : &mut T);
}

impl<'a, T> Arena<'a, T> for ArenaObj<'a, T>
{	
	fn new() -> ArenaObj<'a, T>
	{
		ArenaObj
		{
			arena : Vec::new(),
		}
	}

	fn alloc(&mut self, parent : Option<&'a mut Box<TreeNode<'a, T>>>, childindex : usize , data : T) -> &mut TreeNode<'a, T>
	{
		self.arena.push
		(
			TreeNode::new(parent, childindex, data)
		);

		self.arena.last().unwrap().deref_mut()
	}
} 

impl<'a, T> TreeNode<'a, T>
{
    fn update_indexes(&mut self)
    {
    	let mut index : usize = 0;

        while index < self.children.len()
        {
            self.children[index].childindex = index;
            index += 1
        }
    }

	pub fn new(parent : Option<&'a mut Box<Self>>,childindex : usize , data : T) -> Box<Self>
	{
		let mut child = Box::new(
			Self
			{
				this : None,
				parent : None,
				children : Vec::new(),
				childindex : usize::MAX,
				data,
			});

		{
			child.this = Some(&mut child);
		}
		
		if parent.is_some()
		{			
			let mut parent = parent.unwrap();
			let mut childindex = childindex;

			if childindex == usize::MAX
			{
				childindex = parent.children.len();	
			}

			child.parent = Some(&mut parent);
			parent.children.insert(childindex, child.deref_mut());
			parent.update_indexes();
		}
		else 
		{
			assert_eq!(childindex == usize::MAX, true);
		}
		
		child
	}

    pub fn remove_child(&mut self, childindex : usize) -> &mut Self
    {
    	//Check child index.
        if childindex >= self.children.len()
        {
            panic!("Too big index for removing.");
        }
      
       	let mut child = self.children.remove(childindex);
       	child.parent = None;
       	child.childindex = usize::MAX;        
        self.update_indexes();

        child
    }

    pub fn insert_child(&mut self,  childindex : usize, child : &mut Self)
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
        	//let mut parent = unsafe { Box::from_raw(parent) };
	       	parent.remove_child(child.childindex);
        }

        //Set parent for child.
        child.parent = self.this;

        //Insert child to children and update indexes.
        self.children.insert(childindex, child);        
        self.update_indexes();
    }

	pub fn set_parent(&mut self, parent : Option<Box<Self>>, childindex : usize)
	{
	}

	pub fn parent(&self) -> Option<&'a mut Self>
	{
		self.parent
	}

/*	pub fn child(&self, index : usize) -> Option<&Box<Self>>
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
	}		*/
}
*/