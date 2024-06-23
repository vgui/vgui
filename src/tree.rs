#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::vec::Vec;
use std::rc::{Rc, Weak};
use std::any::Any;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, OnceLock};
use std::cell::{Cell, RefCell};
use crate::arena::{Index, Arena};

const TREE_ARENA_CHUNK_SIZE : usize = 64;

/*
fn get_tree_arena() -> &'static Arc<Mutex<Arena<TreeNode>>> 
{
    static ARENA: OnceLock<Arc<Mutex<Arena<TreeNode>>>> = OnceLock::new();
    ARENA.get_or_init(|| Arc::new(Mutex::new(Arena::new(TREE_ARENA_CHUNK_SIZE))))
}
*/

thread_local!
{
	static ARENA : RefCell<Arena<TreeNode>> = RefCell::new(<Arena<TreeNode>>::new(TREE_ARENA_CHUNK_SIZE));
}


pub struct TreeNode
{
	index : Index,
	parent : Option<Index>,
	children : Vec<Index>,
	childindex : usize,
	data : Box<dyn Any + Send + Sync>,
}

impl TreeNode
{	
	pub fn new(parent : Option<Index>,childindex : usize , data : Box<dyn Any + Send + Sync>) -> Index
	{
		let node = TreeNode 
		{
			index : Index::new(0,0,0),
			parent : parent,
			children : Vec::new(),
			childindex : childindex,
			data : data,
		};

/*		let index;
		{
			let mut arena = get_tree_arena().lock().unwrap();
			index = arena.alloc(node);
			arena.get(index).unwrap().index = index;
		}		
*/
		let mut index = Index::new(0,0,0);
		ARENA.with_borrow_mut(|arena| 
		{
			index = arena.alloc(node);
			arena[index].index = index;
		}		
		);

		index
	}

	pub fn free(&self)
	{
		//get_tree_arena().lock().unwrap().free(self.index());
		ARENA.with_borrow_mut(|arena| arena.free(self.index()))
	}

	pub fn index(&self) -> Index
	{
		self.index
	}

	pub fn parent(&self) -> Option<Index>
	{
		self.parent
	}

	pub fn child(&self, index : usize) -> Option<&Index>
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

	pub fn data<T: 'static>(&self) -> Option<&T>
	{
		self.data.downcast_ref::<T>()
	}	

}

impl Drop for TreeNode
{
	fn drop(&mut self)
	{
		println!("Tree node dropped {},{}", self.index.age(), self.index.index());
	}
}

#[cfg(test)]
mod tests 
{
	use super::*;
    use crate::arena::{Index, Arena};
    use crate::tree::{TreeNode};

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
    fn tree_new_free()
    {
    	let root = TreeNode::new(None, usize::MAX, WidgetObj::new("root"));    	
    	let w1 = TreeNode::new(Some(root), usize::MAX, WidgetObj::new("w1"));
    	let w2 = TreeNode::new(Some(root), usize::MAX, WidgetObj::new("w2"));
    	let w3 = TreeNode::new(Some(root), usize::MAX, WidgetObj::new("w3"));

    	//let mut arena = get_tree_arena().lock().unwrap();    	

    	ARENA.with_borrow_mut(|arena| -> ()
    	{
	    	let id = arena.id();    
    	 	
	    	assert_eq!(arena.used(), 4);
    		assert_eq!(arena[root].index(), Index::new(id,0,0));
	    	assert_eq!(arena[root].index(), Index::new(id,0,0));
    		assert_eq!(arena[root].parent(), None);

	    	assert_eq!(arena[w1].index(), Index::new(id,0,1));
    		assert_eq!(arena[w1].index(), Index::new(id,0,1));
    		assert_eq!(arena[w1].parent(), Some(root));

	    	assert_eq!(arena[w2].index(), Index::new(id,0,2));
    		assert_eq!(arena[w2].index(), Index::new(id,0,2));
    		assert_eq!(arena[w2].parent(), Some(root));

	    	assert_eq!(arena[w3].index(), Index::new(id,0,3));
    		assert_eq!(arena[w3].index(), Index::new(id,0,3));
    		assert_eq!(arena[w3].parent(), Some(root));

	    	arena.free(root);
	    	arena.free(w1);
	    	arena.free(w2);
	    	arena.free(w3);
    		// arena[w1].free();
    		// arena[w2].free();
    		// arena[w3].free();
    	});
    }

}//mod tests
