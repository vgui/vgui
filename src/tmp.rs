#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::vec::Vec;

pub struct TreeNode<'a, T>
{
	this : Option<&'a Box<Self>>,
	parent : Option<&'a Box<Self>>,
	children : Vec<Box<Self>>,
	childindex : usize,
	data : T,
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

	pub fn new(parent : Option<Box<Self>>,childindex : usize , data : T) -> Box<Self>
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

		child.this = Some(&child);
		
		if parent.is_some()
		{			
			let mut parent = parent.unwrap();
			let mut childindex = childindex;

			if childindex == usize::MAX
			{
				childindex = parent.children.len();	
			}

			child.parent = Some(&parent);
			parent.children.insert(childindex, child);
			parent.update_indexes();
		}
		else 
		{
			assert_eq!(childindex == usize::MAX, true);
		}
		
		child
	}

    pub fn remove_child(&mut self, childindex : usize) -> Box<Self>
    {
    	//Check child index.
        if childindex >= self.children.len()
        {
            panic!("Too big index for removing.");
        }
      
       	let child = self.children.remove(childindex);
       	child.parent = None;
       	child.childindex = usize::MAX;        
        self.update_indexes();

        child
    }

    pub fn insert_child(&mut self,  childindex : usize, child : Box<Self>)
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

	pub fn parent(&self) -> Option<&'a Box<Self>>
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
