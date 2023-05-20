#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::vec::Vec;
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::any::Any;


type RcRefCell<T> = Rc<RefCell<T>>;
type WeakRefCell<T> = Weak<RefCell<T>>;
type TreeNodePtr<T> = RcRefCell<TreeNode::<T>>;


pub struct TreeNode<T>
{
	weak_self : WeakRefCell<Self>,
	parent : WeakRefCell<Self>,
	children : Vec<RcRefCell<Self>>,
	childindex : usize,
	data : T,
}

impl<T> TreeNode<T>
{
    fn update_indexes(&mut self)
    {
    	let mut index : usize = 0;

        while index < self.children.len()
        {
            self.children[index].borrow_mut().childindex = index;
            index += 1
        }
    }

	pub fn new(parent : Option<RcRefCell<Self>>, data : T) -> RcRefCell<Self>
	{
		let child = Rc::new(RefCell::new(
			Self
			{
				weak_self : Weak::new(),
				parent : Weak::new(),
				children : Vec::new(),
				childindex : usize::MAX,
				data,
			}));
		
		child.borrow_mut().weak_self = Rc::downgrade(&child);

		if parent.is_some()
		{
			let parent = parent.unwrap().clone();
			child.borrow_mut().parent = Rc::downgrade(&parent);

			let newchildindex = parent.borrow().children.len();	
			parent.borrow_mut().children.insert(newchildindex, Rc::clone(&child));
			parent.borrow_mut().update_indexes();
		}
		
		child
	}

	fn parent(&self) -> Option<RcRefCell<Self>>
	{
		self.parent.upgrade()
	}

	fn set_parent(&mut self, parent : Option<RcRefCell<Self>>)
	{
		self.parent = match parent
		{
			Some(parent) => Rc::downgrade(&parent),
			None => Weak::new()
		};
	}

	fn child(&self, index : usize) -> Option<&RcRefCell<Self>>
	{
		self.children.get(index)
	}

	fn children(&self) -> &Vec<RcRefCell<Self>>
	{
		&self.children
	}

	fn children_mut(&mut self) -> &mut Vec<RcRefCell<Self>>
	{
		&mut self.children
	}	

    fn remove_child(&mut self, childindex : usize) -> RcRefCell<Self>
    {
    	//Check child index.
        if childindex > self.children.len()
        {
            panic!("Too big index for removing.");
        }
      
       	let child = self.children.remove(childindex);
       	child.borrow_mut().parent = Weak::new();
       	child.borrow_mut().childindex = usize::MAX;
        
        self.update_indexes();
        child
    }

    fn insert_child(&mut self,  childindex : usize, child : RcRefCell<Self>)
    {
    	//Check child index.
        if childindex > self.children.len()
        {
            panic!("Too big index for inserting.");
        }

        //If child have parent, remove child from parent using child index.
        if let Some(parent) = child.borrow_mut().parent()
        {
        	parent.borrow_mut().remove_child(child.borrow_mut().childindex);
        }

        //Set parent for child.
        child.borrow_mut().parent = Weak::clone(&self.weak_self);

        //Insert child to children and update indexes.
        self.children.insert(childindex, child);        
        self.update_indexes();
    }

}


#[cfg(test)]
mod tests 
{
	use super::*;

	struct WidgetObj
	{
		id : String,
	}

	impl WidgetObj
	{
		pub fn new(id : &str) -> WidgetObj
		{
			WidgetObj { id : String::from(id), }
		}
	}

	#[test]
	pub fn treenode_update_indexes()
	{
		let root = Rc::new(RefCell::new(
		TreeNode
		{
			weak_self : Weak::new(),
			parent : Weak::new(),
			children : Vec::new(),
			childindex : usize::MAX,
			data : WidgetObj::new("root"),
		}));

		root.borrow_mut().weak_self = Rc::downgrade(&root);

		assert_eq!(Rc::ptr_eq(&root.borrow().weak_self.upgrade().unwrap(), &root), true);		
		assert_eq!(root.borrow().weak_self.strong_count(), 1);
		assert_eq!(root.borrow().weak_self.weak_count(), 1);
		assert_eq!(root.borrow().parent.strong_count(), 0);
		assert_eq!(root.borrow().parent.weak_count(), 0);
		assert_eq!(root.borrow().children.len(), 0);
		assert_eq!(root.borrow().childindex, usize::MAX);
		assert_eq!(root.borrow().data.id, "root");

		let child0 = Rc::new(RefCell::new(
		TreeNode
		{
			weak_self : Weak::new(),
			parent : Rc::downgrade(&root),
			children : Vec::new(),
			childindex : usize::MAX,
			data : WidgetObj::new("child0"),
		}));

		child0.borrow_mut().weak_self = Rc::downgrade(&child0);
		root.borrow_mut().children.insert(0, Rc::clone(&child0));
		root.borrow_mut().update_indexes();

		assert_eq!(Rc::ptr_eq(&child0.borrow().weak_self.upgrade().unwrap(), &child0), true);		
		assert_eq!(child0.borrow().weak_self.strong_count(), 2);//1.root.children[0]; 2.child0
		assert_eq!(child0.borrow().weak_self.weak_count(), 1);
		assert_eq!(child0.borrow().parent.strong_count(), 1);
		assert_eq!(child0.borrow().parent.weak_count(), 2);//1.root.weak_self; 2.child0.parent;
		assert_eq!(child0.borrow().children.len(), 0);
		assert_eq!(child0.borrow().childindex, 0);
		assert_eq!(child0.borrow().data.id, "child0");
		assert_eq!(root.borrow().children.len(), 1);		

		let child1 = Rc::new(RefCell::new(
		TreeNode
		{
			weak_self : Weak::new(),
			parent : Rc::downgrade(&root),
			children : Vec::new(),
			childindex : usize::MAX,
			data : WidgetObj::new("child1"),
		}));

		child1.borrow_mut().weak_self = Rc::downgrade(&child1);
		root.borrow_mut().children.insert(0, Rc::clone(&child1));
		root.borrow_mut().update_indexes();

		assert_eq!(Rc::ptr_eq(&child1.borrow().weak_self.upgrade().unwrap(), &child1), true);		
		assert_eq!(child1.borrow().weak_self.strong_count(), 2);//1.root.children[0]; 2.child1
		assert_eq!(child1.borrow().weak_self.weak_count(), 1);
		assert_eq!(child1.borrow().parent.strong_count(), 1);
		assert_eq!(child1.borrow().parent.weak_count(), 03);//1.root.weak_self; 2.child0.parent;3.child1.parent
		assert_eq!(child1.borrow().children.len(), 0);
		assert_eq!(child1.borrow().childindex, 0);
		assert_eq!(child0.borrow().childindex, 1);
		assert_eq!(child1.borrow().data.id, "child1");
		assert_eq!(root.borrow().children.len(), 2);				
	}

	#[test]
	pub fn treenode_new()
	{
		let root = TreeNode::new(None,WidgetObj::new("root"));

		assert_eq!(Rc::ptr_eq(&root.borrow().weak_self.upgrade().unwrap(), &root), true);		
		assert_eq!(root.borrow().weak_self.strong_count(), 1);
		assert_eq!(root.borrow().weak_self.weak_count(), 1);
		assert_eq!(root.borrow().parent.strong_count(), 0);
		assert_eq!(root.borrow().parent.weak_count(), 0);
		assert_eq!(root.borrow().children.len(), 0);
		assert_eq!(root.borrow().childindex, usize::MAX);
		assert_eq!(root.borrow().data.id, "root");

		let child0 = TreeNode::new(Some(root.clone()), WidgetObj::new("child0"));

		assert_eq!(Rc::ptr_eq(&child0.borrow().weak_self.upgrade().unwrap(), &child0), true);		
		assert_eq!(child0.borrow().weak_self.strong_count(), 2);//1.root.children[0]; 2.child0
		assert_eq!(child0.borrow().weak_self.weak_count(), 1);
		assert_eq!(child0.borrow().parent.strong_count(), 1);
		assert_eq!(child0.borrow().parent.weak_count(), 2);//1.root.weak_self; 2.child0.parent;
		assert_eq!(child0.borrow().children.len(), 0);
		assert_eq!(child0.borrow().childindex, 0);
		assert_eq!(child0.borrow().data.id, "child0");
		assert_eq!(root.borrow().children.len(), 1);

		let child1 = TreeNode::new(Some(root.clone()), WidgetObj::new("child1"));

		assert_eq!(Rc::ptr_eq(&child1.borrow().weak_self.upgrade().unwrap(), &child1), true);		
		assert_eq!(child1.borrow().weak_self.strong_count(), 2);//1.root.children[1]; 2.child1
		assert_eq!(child1.borrow().weak_self.weak_count(), 1);
		assert_eq!(child1.borrow().parent.strong_count(), 1);
		assert_eq!(child1.borrow().parent.weak_count(), 3);//1.root.weak_self; 2.child0.parent; 3.child1.parent
		assert_eq!(child1.borrow().children.len(), 0);
		assert_eq!(child1.borrow().childindex, 1);
		assert_eq!(child1.borrow().data.id, "child1");
		assert_eq!(root.borrow().children.len(), 2);

		let child2 = TreeNode::new(Some(root.clone()), WidgetObj::new("child2"));

		assert_eq!(Rc::ptr_eq(&child2.borrow().weak_self.upgrade().unwrap(), &child2), true);		
		assert_eq!(child2.borrow().weak_self.strong_count(), 2);//1.root.children[2]; 2.child2
		assert_eq!(child2.borrow().weak_self.weak_count(), 1);
		assert_eq!(child2.borrow().parent.strong_count(), 1);
		assert_eq!(child2.borrow().parent.weak_count(), 4);//1.root.weak_self; 2.child0.parent; 3.child1.parent; 4.child2.parent
		assert_eq!(child2.borrow().children.len(), 0);
		assert_eq!(child2.borrow().childindex, 2);
		assert_eq!(child2.borrow().data.id, "child2");
		assert_eq!(root.borrow().children.len(), 3);

	}

}