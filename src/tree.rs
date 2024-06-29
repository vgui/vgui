#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::vec::Vec;
use std::any::Any;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, Once, RwLock};
use std::{mem::MaybeUninit, thread};
use crate::arena::{Index, Arena};


const TREE_ARENA_CHUNK_SIZE : usize = 64;


pub trait Tree
{	
	fn index(&self) -> Index;
    fn remove(&mut self, childindex : Index) -> Index;
    fn insert(&mut self,  childindex : usize, child : Index);
	fn parent(&self) -> Option<Index>;
	fn child(&self, index : usize) -> Index;
	fn childindex(&self) -> usize;
	fn children_count(&self) -> usize;
	//fn data<T>(&self) -> Option<&T>;
}


pub struct TreeNode<'a>
{
	arena : &'a Arena<TreeNode<'a>>,
	index : Index,
	parent : Option<Index>,
	children : Vec<Index>,
	childindex : usize,
	data : Box<dyn Any + Send + Sync>,
}

impl TreeNode
{	
	fn arena(&self) -> &'a Arena<TreeNode<'a>>
	{
		self.arena
	}

	pub fn new(parent : Option<Index>,childindex : usize , data : Box<dyn Any + Send + Sync>) -> Index
	{
		let node = TreeNode 
		{
			index : Index::new(0,0,0),
			parent : None,
			children : Vec::new(),
			childindex : usize::MAX,
			data,
		};

		let index = selfarena.alloc(node);

		if parent.is_some()
		{			
			let mut childindex = childindex;

			if childindex == usize::MAX
			{
				childindex = parent.unwrap().children_count();	
			}

			let mut arena = TreeNode::get_arena().lock().unwrap();
			arena[index].parent = parent;
			arena[parent.unwrap()].children.insert(childindex, index);
			arena[parent.unwrap()].update_indexes(childindex);
			//println!("parent = {}, {}, {}", parent.unwrap().arena_id(), parent.unwrap().age(), parent.unwrap().index());
			//println!("childindex = {}, {}", childindex,arena.id());
		}		
	

		index
	}

	pub fn free(index : Index)
	{
		TreeNode::get_arena().lock().unwrap().free(index);
	}

    pub fn arena_id() -> usize
	{
		TreeNode::get_arena().lock().unwrap().id()
	}

    fn update_indexes(&mut self, start_index : usize)
    {
    	let mut i : usize = start_index;

        while i < self.children.len()
        {
        	let index = self.children[i];        	
        	TreeNode::get_arena().lock().unwrap()[index].childindex = i;
            
            i += 1
        }
    }
}

impl Drop for TreeNode
{
	fn drop(&mut self)
	{
		println!("Tree node dropped {},{}", self.index.age(), self.index.index());
	}
}

impl Tree for TreeNode
{	
    fn remove(&mut self, child : Index) -> Index
    {
    	//Check child index.
    	let child = &mut TreeNode::get_arena().lock().unwrap()[child];
		let childindex = child.childindex;

        if childindex >= self.children.len()
        {
            panic!("Too big index for removing.");
        }
      
		child.parent = None;
       	child.childindex = usize::MAX;

		self.children.remove(childindex);
        self.update_indexes(childindex);

        child.index()
    }

    fn insert(&mut self,  childindex : usize, child : Index)
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

        let child = &mut TreeNode::get_arena().lock().unwrap()[child];

        //If child have parent, remove child from parent using child index.
        if let Some(mut parent) = child.parent()
        {
			parent.remove(child.index());
        }

        //Set parent for child.
        child.parent = Some(self.index());

        //Insert child to the children and update indexes.
        self.children.insert(childindex, child.index());
        child.childindex = childindex;
        self.update_indexes(childindex+1);
    }

    fn index(&self) -> Index
	{
		self.index
	}

	fn parent(&self) -> Option<Index>
	{
		self.parent
	}

	fn child(&self, index : usize) -> Index
	{
		self.children[index]
	}

	fn childindex(&self) -> usize
	{
		self.childindex
	}

	fn children_count(&self) -> usize
	{
		self.children.len()
	}	
}

impl Tree for Index
{	
    fn remove(&mut self, index : Index) -> Index
    {
    	TreeNode::get_arena().lock().unwrap()[*self].remove(index)
    }

    fn insert(&mut self,  childindex : usize, child : Index)
    {
    	TreeNode::get_arena().lock().unwrap()[*self].insert(childindex, child)
    }

    fn index(&self) -> Index
	{
		TreeNode::get_arena().lock().unwrap()[*self].index()
	}

	fn parent(&self) -> Option<Index>
	{
		TreeNode::get_arena().lock().unwrap()[*self].parent()
	}

	fn child(&self, index : usize) -> Index
	{
		TreeNode::get_arena().lock().unwrap()[*self].child(index)
	}

	fn childindex(&self) -> usize
	{
    	TreeNode::get_arena().lock().unwrap()[*self].childindex()
	}

	fn children_count(&self) -> usize
	{
		TreeNode::get_arena().lock().unwrap()[*self].children_count()
	}	
}

#[cfg(test)]
mod tests 
{
	use super::*;
    use crate::arena::{Index, Arena};
    use crate::tree::{TreeNode};

    struct Builder
    {
    	tree_arena : RwLock<Arena<TreeNode>>,
    }

    impl Builder
    {
    	pub fn new() -> Self
    	{
    		let builder = Self
    		{
    			tree_arena : RwLock::new(Arena::new()),
    		};    		

    		{
    			let mut arena = builder.tree_arena.write().unwrap();
    			arena.init(TREE_ARENA_CHUNK_SIZE);    		
    		}

    		builder
    	}

    	
    	pub fn new_tree_node(&self, parent : Option<Index>,childindex : usize , data : Box<dyn Any + Send + Sync>) -> Index
    	{
    		let mut arena = self.tree_arena.write().unwrap();

			let node = TreeNode 
			{
				index : Index::new(0,0,0),
				parent : None,
				children : Vec::new(),
				childindex : usize::MAX,
				data,
			};

			let index;
			{
				//let mut arena = TreeNode::get_arena().lock().unwrap();
				index = arena.alloc(node);
				arena[index].index = index;	
			}	

			if parent.is_some()
			{			
				let parent = parent.unwrap();
				let mut childindex = childindex;

				if childindex == usize::MAX
				{
					childindex = arena[parent].children_count();	
				}

				//let mut arena = TreeNode::get_arena().lock().unwrap();
				arena[index].parent = Some(parent);
				arena[parent].children.insert(childindex, index);//Insert if parent exist
				arena[parent].update_indexes(childindex);
				//println!("parent = {}, {}, {}", parent.unwrap().arena_id(), parent.unwrap().age(), parent.unwrap().index());
				//println!("childindex = {}, {}", childindex,arena.id());
    		}

    		index
    	}
    }

	struct WidgetObj
	{
		id : String,
	}

	impl WidgetObj
	{
		pub fn new(id : &str) -> Box<WidgetObj>
		{
			Box::new(WidgetObj { id : String::from(id), })
		}
	}

	pub trait Widget
	{	
	    fn paint(&mut self){}
    	fn size(&mut self) {}
	}

   #[test]
    fn tree_builder()
    {
    	let builder = Builder::new();
    	let root = builder.new_tree_node(None, usize::MAX, WidgetObj::new("root")); 
    	let child0 = builder.new_tree_node(Some(root), usize::MAX, WidgetObj::new("child0"));
    	let child1 = builder.new_tree_node(Some(root), usize::MAX, WidgetObj::new("child1"));
    	let child2 = builder.new_tree_node(Some(root), usize::MAX, WidgetObj::new("child2"));
    	let child3 = builder.new_tree_node(Some(root), usize::MAX, WidgetObj::new("child3"));
    	let child4 = builder.new_tree_node(Some(root), usize::MAX, WidgetObj::new("child4"));
 
		assert_eq!(child0.childindex(), 0);
		assert_eq!(child1.childindex(), 1);
		assert_eq!(child2.childindex(), 2);
		assert_eq!(child3.childindex(), 3);
		assert_eq!(child4.childindex(), 4);
		assert_eq!(root.children_count(), 5);

 	}

 /*   #[test]
    fn tree_new_free()
    {
    	let root = TreeNode::new(None, usize::MAX, WidgetObj::new("root"));    	
    	let w1 = TreeNode::new(Some(root), usize::MAX, WidgetObj::new("w1"));
    	let w2 = TreeNode::new(Some(root), usize::MAX, WidgetObj::new("w2"));
    	let w3 = TreeNode::new(Some(root), usize::MAX, WidgetObj::new("w3"));
    	
    	let arena_id = TreeNode::arena_id();    	
   		println!("root = {}, {}, {}", root.arena_id(), root.age(), root.index());

   		assert_eq!(root.arena_id(), arena_id);
   		assert_eq!(w1.arena_id(), arena_id);
   		assert_eq!(w2.arena_id(), arena_id);
   		assert_eq!(w3.arena_id(), arena_id);

   		assert_eq!(root.index(), 0);
   		assert_eq!(root.parent(), None);
    	assert_eq!(root.childindex(), usize::MAX);
	   		
   		assert_eq!(w1.index(), 1);
   		assert_eq!(w1.parent(), Some(root));
    	assert_eq!(w1.childindex(),  usize::MAX);

   		assert_eq!(w2.index(), 2);
   		assert_eq!(w2.parent(), Some(root));
    	assert_eq!(w2.childindex(),  usize::MAX);

   		assert_eq!(w3.index(), 3);
   		assert_eq!(w3.parent(), Some(root));
    	assert_eq!(w3.childindex(),  usize::MAX);    	

    	TreeNode::free(root);
    	TreeNode::free(w1);
    	TreeNode::free(w2);
    	TreeNode::free(w3);
    }

#[test]
	pub fn tree_remove()
	{
		let mut root = TreeNode::new(None,usize::MAX, Box::new(WidgetObj::new("root")));
		let child0 = TreeNode::new(Some(root),usize::MAX, Box::new(WidgetObj::new("child0")));
		let child1 = TreeNode::new(Some(root),usize::MAX, Box::new(WidgetObj::new("child1")));
		let child2 = TreeNode::new(Some(root),usize::MAX, Box::new(WidgetObj::new("child2")));
		let child3 = TreeNode::new(Some(root),usize::MAX, Box::new(WidgetObj::new("child3")));
		let child4 = TreeNode::new(Some(root),usize::MAX, Box::new(WidgetObj::new("child4")));

		assert_eq!(child0.childindex(), 0);
		assert_eq!(child1.childindex(), 1);
		assert_eq!(child2.childindex(), 2);
		assert_eq!(child3.childindex(), 3);
		assert_eq!(child4.childindex(), 4);
		assert_eq!(root.children_count(), 5);

		let c0 = root.remove(child0);
		assert_eq!(c0.childindex(), usize::MAX);
		assert_eq!(child1.childindex(), 0);
		assert_eq!(child2.childindex(), 1);		
		assert_eq!(child3.childindex(), 2);
		assert_eq!(child4.childindex(), 3);
		assert_eq!(root.children_count(), 4);

		let c3 = root.remove(child3);
		assert_eq!(c3.childindex(), usize::MAX);
		assert_eq!(child1.childindex(), 0);
		assert_eq!(child2.childindex(), 1);		
		assert_eq!(child4.childindex(), 2);
		assert_eq!(root.children_count(), 3);

		let c4 = root.remove(child4);
		assert_eq!(c4.childindex(), usize::MAX);
		assert_eq!(child1.childindex(), 0);
		assert_eq!(child2.childindex(), 1);
		assert_eq!(root.children_count(), 2);	

		let c1 = root.remove(child1);
		assert_eq!(c1.childindex(), usize::MAX);
		assert_eq!(child2.childindex(), 0);
		assert_eq!(root.children_count(), 1);	

		let c2 = root.remove(child2);
		assert_eq!(c2.childindex(), usize::MAX);
		assert_eq!(root.children_count(), 0);	
	}


	#[test]
	pub fn tree_insert()
	{
		let mut root = TreeNode::new(None, usize::MAX, Box::new(WidgetObj::new("root")));
		let child0 = TreeNode::new(None, usize::MAX, Box::new(WidgetObj::new("child0")));
		let child1 = TreeNode::new(None, usize::MAX, Box::new(WidgetObj::new("child1")));
		let child2 = TreeNode::new(None, usize::MAX, Box::new(WidgetObj::new("child2")));
		let child3 = TreeNode::new(None, usize::MAX, Box::new(WidgetObj::new("child3")));
		let child4 = TreeNode::new(None, usize::MAX, Box::new(WidgetObj::new("child4")));

		root.insert(0, child4);
		root.insert(0, child3);
		root.insert(0, child2);
		root.insert(0, child1);
		root.insert(0, child0);

		assert_eq!(child0.childindex(), 0);
		assert_eq!(child1.childindex(), 1);
		assert_eq!(child2.childindex(), 2);
		assert_eq!(child3.childindex(), 3);
		assert_eq!(child4.childindex(), 4);
		assert_eq!(root.children_count(), 5);
	}*/
}//mod tests
