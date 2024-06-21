#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::vec::Vec;
use std::rc::{Rc, Weak};
use std::any::Any;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, OnceLock};
use crate::arena::{Index, Arena};

const TREE_ARENA_CHUNK_SIZE : usize = 64;

fn get_tree_arena() -> &'static Arc<Mutex<Arena<TreeNode>>> 
{
    static ARENA: OnceLock<Arc<Mutex<Arena<TreeNode>>>> = OnceLock::new();
    ARENA.get_or_init(|| Arc::new(Mutex::new(Arena::new(TREE_ARENA_CHUNK_SIZE))))
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
		let arena = get_tree_arena();

		let node = TreeNode 
		{
			index : Index::new(0,0,0),
			parent : parent,
			children : Vec::new(),
			childindex : childindex,
			data : data,
		};

		let index;
		{
			let mut arena = arena.lock().unwrap();
			index = arena.alloc(node);
			arena.get_by_index(index).unwrap().index = index;
		}		

		index
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
    fn tree_new()
    {
    	let root = TreeNode::new(None, usize::MAX, WidgetObj::new("w1"));
    	let w1 = TreeNode::new(Some(root), usize::MAX, WidgetObj::new("w2"));
    	let w2 = TreeNode::new(Some(root), usize::MAX, WidgetObj::new("w3"));
    	let w2 = TreeNode::new(Some(root), usize::MAX, WidgetObj::new("w4"));

    	// let arena = get_tree_arena().lock().unwrap();
    	// assert_eq!(arena.used(), 4);
    	// assert_eq!(arena.get(0, 1).unwrap(), Index::new(arena.id(),0,0));
    }

}//mod tests