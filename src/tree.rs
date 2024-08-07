#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::slice::{Iter, IterMut};
use std::vec::Vec;
use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref, RefMut};
use std::any::Any;
use std::ops::{Deref, DerefMut};


pub type RcRefCell<T> = Rc<RefCell<T>>;
pub type WeakRefCell<T> = Weak<RefCell<T>>;
//type TreePtr = RcRefCell<TreeNode>;


pub struct TreeNode
{
	weak_self : WeakRefCell<Self>,
	parent : WeakRefCell<Self>,
	children : Vec<RcRefCell<Self>>,
	childindex : usize,
	data : Box<dyn Any>,
}

impl TreeNode
{	
	pub fn new(parent : Option<RcRefCell<Self>>,childindex : usize , data : Box<dyn Any>) -> RcRefCell<Self>
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
			let mut childindex = childindex;

			if childindex == usize::MAX
			{
				childindex = parent.borrow().children.len();	
			}

			child.borrow_mut().parent = Rc::downgrade(&parent);
			parent.borrow_mut().children.insert(childindex, Rc::clone(&child));
			parent.borrow_mut().update_indexes(childindex);
		}
		
		child
	}

    fn update_indexes(&mut self, start_index : usize)
    {
    	let mut index : usize = start_index;

        while index < self.children.len()
        {
            self.children[index].borrow_mut().childindex = index;
            index += 1
        }
    }

    pub fn remove(&mut self, childindex : usize) -> RcRefCell<TreeNode>
    {
    	//Check child index.
		let mut childindex = childindex;

		if childindex == usize::MAX
		{
			childindex = self.children.len();	
		}

        if childindex > self.children.len()
        {
            panic!("Too big index for removing.");
        }

		let child = self.children.remove(childindex);
        self.update_indexes(childindex);
      
		child.borrow_mut().parent = Weak::new();
       	child.borrow_mut().childindex = usize::MAX;

        child
    }

    pub fn insert(&mut self,  childindex : usize, child : &mut Self)
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
	       	parent.borrow_mut().remove(child.childindex());
        }

        //Set parent for child.
        child.parent = Weak::clone(&self.weak_self);

        //Insert child to children and update indexes.
        self.children.insert(childindex, child.weak_self.upgrade().unwrap());
        child.childindex = childindex;
        self.update_indexes(childindex+1);
    }

 	pub fn set_parent(&mut self, newparent : &mut Self, childindex : usize)
	{
		if let Some(parent) = self.parent()
		{
			parent.borrow_mut().remove(self.childindex());
		}

		newparent.insert(childindex, self);		
	}

	pub fn iter(&self) -> Iter<'_, RcRefCell<Self>>
	{
		self.children.iter()
	}

	pub fn iter_mut(&mut self) -> IterMut<'_, RcRefCell<Self>>
	{
		self.children.iter_mut()
	}

	pub fn parent(&self) -> Option<RcRefCell<Self>>
	{
		self.parent.upgrade()
	}

	pub fn child(&self, index : usize) -> Option<&RcRefCell<Self>>
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

	pub fn data(&self) -> &Box<dyn Any>
	{
		&self.data
	}	

	pub fn data_mut(&mut self) -> &mut Box<dyn Any>
	{
		&mut self.data
	}		
}


pub trait Tree
{	
    fn remove(&mut self, childindex : usize) -> RcRefCell<TreeNode>;
    fn insert(&mut self,  childindex : usize, child : &mut TreeNode);
 	fn set_parent(&mut self, newparent : &mut TreeNode, childindex : usize);
	fn iter(&self) -> Iter<'_, RcRefCell<TreeNode>>;
	fn iter_mut(&mut self) -> IterMut<'_, RcRefCell<TreeNode>>; 	
	fn parent(&self) -> Option<RcRefCell<TreeNode>>;
	fn child(&self, index : usize) -> Option<&RcRefCell<TreeNode>>;
	fn childindex(&self) -> usize;
	fn children_count(&self) -> usize;
	fn data(&self) -> &Box<dyn Any>;
	fn data_mut(&mut self) -> &mut Box<dyn Any>;
}


#[cfg(test)]
mod tests 
{
	use super::*;

	struct MyWidget
	{
		id : String,
	}

	impl MyWidget
	{
		pub fn new(id : &str) -> MyWidget
		{
			MyWidget { id : String::from(id), }
		}
	}

	#[test]
	pub fn tree_update_indexes()
	{
		let root = Rc::new(RefCell::new(
		TreeNode
		{
			weak_self : Weak::new(),
			parent : Weak::new(),
			children : Vec::new(),
			childindex : usize::MAX,
			data : Box::new(MyWidget::new("root")),
		}));

		root.borrow_mut().weak_self = Rc::downgrade(&root);

		assert_eq!(Rc::ptr_eq(&root.borrow().weak_self.upgrade().unwrap(), &root), true);		
		assert_eq!(root.borrow().weak_self.strong_count(), 1);
		assert_eq!(root.borrow().weak_self.weak_count(), 1);
		assert_eq!(root.borrow().parent.strong_count(), 0);
		assert_eq!(root.borrow().parent.weak_count(), 0);
		assert_eq!(root.borrow().children_count(), 0);
		assert_eq!(root.borrow().childindex(), usize::MAX);
		assert_eq!(root.borrow().data.downcast_ref::<MyWidget>().unwrap().id, "root");

		let child0 = Rc::new(RefCell::new(
		TreeNode
		{
			weak_self : Weak::new(),
			parent : Rc::downgrade(&root),
			children : Vec::new(),
			childindex : usize::MAX,
			data : Box::new(MyWidget::new("child0")),
		}));		

		child0.borrow_mut().weak_self = Rc::downgrade(&child0);
		root.borrow_mut().children.insert(0, Rc::clone(&child0));
		root.borrow_mut().update_indexes(0);

		assert_eq!(Rc::ptr_eq(&child0.borrow().weak_self.upgrade().unwrap(), &child0), true);		
		assert_eq!(child0.borrow().weak_self.strong_count(), 2);//1.root.children[0]; 2.child0
		assert_eq!(child0.borrow().weak_self.weak_count(), 1);
		assert_eq!(child0.borrow().parent.strong_count(), 1);
		assert_eq!(child0.borrow().parent.weak_count(), 2);//1.root.weak_self; 2.child0.parent;
		assert_eq!(child0.borrow().children_count(), 0);
		assert_eq!(child0.borrow().childindex(), 0);
		assert_eq!(child0.borrow().data.downcast_ref::<MyWidget>().unwrap().id, "child0");
		assert_eq!(root.borrow().children_count(), 1);		

		let child1 = Rc::new(RefCell::new(
		TreeNode
		{
			weak_self : Weak::new(),
			parent : Rc::downgrade(&root),
			children : Vec::new(),
			childindex : usize::MAX,
			data : Box::new(MyWidget::new("child1")),
		}));

		child1.borrow_mut().weak_self = Rc::downgrade(&child1);
		root.borrow_mut().children.insert(0, Rc::clone(&child1));
		root.borrow_mut().update_indexes(0);

		assert_eq!(Rc::ptr_eq(&child1.borrow().weak_self.upgrade().unwrap(), &child1), true);		
		assert_eq!(child1.borrow().weak_self.strong_count(), 2);//1.root.children[0]; 2.child1
		assert_eq!(child1.borrow().weak_self.weak_count(), 1);
		assert_eq!(child1.borrow().parent.strong_count(), 1);
		assert_eq!(child1.borrow().parent.weak_count(), 3);//1.root.weak_self; 2.child0.parent;3.child1.parent
		assert_eq!(child1.borrow().children_count(), 0);
		assert_eq!(child1.borrow().childindex(), 0);
		assert_eq!(child0.borrow().childindex(), 1);
		assert_eq!(child1.borrow().data.downcast_ref::<MyWidget>().unwrap().id, "child1");
		assert_eq!(root.borrow().children_count(), 2);						
	}

	#[test]
	pub fn tree_new()
	{
		let root = TreeNode::new(None,usize::MAX, Box::new(MyWidget::new("root")));

		assert_eq!(Rc::ptr_eq(&root.borrow().weak_self.upgrade().unwrap(), &root), true);		
		assert_eq!(root.borrow().weak_self.strong_count(), 1);
		assert_eq!(root.borrow().weak_self.weak_count(), 1);
		assert_eq!(root.borrow().parent.strong_count(), 0);
		assert_eq!(root.borrow().parent.weak_count(), 0);
		assert_eq!(root.borrow().children_count(), 0);
		assert_eq!(root.borrow().childindex(), usize::MAX);
		assert_eq!(root.borrow().data.downcast_ref::<MyWidget>().unwrap().id, "root");

		let child0 = TreeNode::new(Some(root.clone()),usize::MAX, Box::new(MyWidget::new("child0")));

		assert_eq!(Rc::ptr_eq(&child0.borrow().weak_self.upgrade().unwrap(), &child0), true);		
		assert_eq!(child0.borrow().weak_self.strong_count(), 2);//1.root.children[0]; 2.child0
		assert_eq!(child0.borrow().weak_self.weak_count(), 1);
		assert_eq!(child0.borrow().parent.strong_count(), 1);
		assert_eq!(child0.borrow().parent.weak_count(), 2);//1.root.weak_self; 2.child0.parent;
		assert_eq!(child0.borrow().children_count(), 0);
		assert_eq!(child0.borrow().childindex(), 0);
		assert_eq!(child0.borrow().data.downcast_ref::<MyWidget>().unwrap().id, "child0");
		assert_eq!(root.borrow().children_count(), 1);		

		let child1 = TreeNode::new(Some(root.clone()),usize::MAX, Box::new(MyWidget::new("child1")));

		assert_eq!(Rc::ptr_eq(&child1.borrow().weak_self.upgrade().unwrap(), &child1), true);		
		assert_eq!(child1.borrow().weak_self.strong_count(), 2);//1.root.children[1]; 2.child1
		assert_eq!(child1.borrow().weak_self.weak_count(), 1);
		assert_eq!(child1.borrow().parent.strong_count(), 1);
		assert_eq!(child1.borrow().parent.weak_count(), 3);//1.root.weak_self; 2.child0.parent; 3.child1.parent
		assert_eq!(child1.borrow().children_count(), 0);
		assert_eq!(child1.borrow().childindex(), 1);
		assert_eq!(child0.borrow().childindex(), 0);
		assert_eq!(child1.borrow().data.downcast_ref::<MyWidget>().unwrap().id, "child1");
		assert_eq!(root.borrow().children_count(), 2);						

		let child2 = TreeNode::new(Some(root.clone()),usize::MAX, Box::new(MyWidget::new("child2")));

		assert_eq!(Rc::ptr_eq(&child2.borrow().weak_self.upgrade().unwrap(), &child2), true);		
		assert_eq!(child2.borrow().weak_self.strong_count(), 2);//1.root.children[2]; 2.child2
		assert_eq!(child2.borrow().weak_self.weak_count(), 1);
		assert_eq!(child2.borrow().parent.strong_count(), 1);
		assert_eq!(child2.borrow().parent.weak_count(), 4);//1.root.weak_self; 2.child0.parent; 3.child1.parent; 4.child2.parent
		assert_eq!(child2.borrow().children_count(), 0);
		assert_eq!(child2.borrow().childindex(), 2);
		assert_eq!(child1.borrow().childindex(), 1);		
		assert_eq!(child0.borrow().childindex(), 0);
		assert_eq!(child2.borrow().data.downcast_ref::<MyWidget>().unwrap().id, "child2");
		assert_eq!(root.borrow().children_count(), 3);						
	}

	#[test]
	pub fn tree_remove()
	{
		let root = TreeNode::new(None,usize::MAX, Box::new(MyWidget::new("root")));
		let child0 = TreeNode::new(Some(root.clone()),usize::MAX, Box::new(MyWidget::new("child0")));
		let child1 = TreeNode::new(Some(root.clone()),usize::MAX, Box::new(MyWidget::new("child1")));
		let child2 = TreeNode::new(Some(root.clone()),usize::MAX, Box::new(MyWidget::new("child2")));
		let child3 = TreeNode::new(Some(root.clone()),usize::MAX, Box::new(MyWidget::new("child3")));
		let child4 = TreeNode::new(Some(root.clone()),usize::MAX, Box::new(MyWidget::new("child4")));

		assert_eq!(child0.borrow().childindex(), 0);
		assert_eq!(child1.borrow().childindex(), 1);
		assert_eq!(child2.borrow().childindex(), 2);
		assert_eq!(child3.borrow().childindex(), 3);
		assert_eq!(child4.borrow().childindex(), 4);
		assert_eq!(root.borrow().children_count(), 5);

		let childindex = child0.borrow().childindex();
		let c0 = root.borrow_mut().remove(childindex);
		assert_eq!(Rc::ptr_eq(&child0.borrow().weak_self.upgrade().unwrap(), &c0), true);				
		assert_eq!(c0.borrow_mut().childindex(), usize::MAX);
		assert_eq!(child1.borrow().childindex(), 0);
		assert_eq!(child2.borrow().childindex(), 1);		
		assert_eq!(child3.borrow().childindex(), 2);
		assert_eq!(child4.borrow().childindex(), 3);
		assert_eq!(root.borrow().children_count(), 4);

		let childindex = child3.borrow().childindex();
		let c3 = root.borrow_mut().remove(childindex);
		assert_eq!(Rc::ptr_eq(&child3.borrow().weak_self.upgrade().unwrap(), &c3), true);				
		assert_eq!(c3.borrow_mut().childindex(), usize::MAX);
		assert_eq!(child1.borrow().childindex(), 0);
		assert_eq!(child2.borrow().childindex(), 1);		
		assert_eq!(child4.borrow().childindex(), 2);
		assert_eq!(root.borrow().children_count(), 3);

		let childindex = child4.borrow().childindex();
		let c4 = root.borrow_mut().remove(childindex);
		assert_eq!(Rc::ptr_eq(&child4.borrow().weak_self.upgrade().unwrap(), &c4), true);				
		assert_eq!(c4.borrow_mut().childindex(), usize::MAX);
		assert_eq!(child1.borrow().childindex(), 0);
		assert_eq!(child2.borrow().childindex(), 1);
		assert_eq!(root.borrow().children_count(), 2);	

		let childindex = child1.borrow().childindex();
		let c1 = root.borrow_mut().remove(childindex);
		assert_eq!(Rc::ptr_eq(&child1.borrow().weak_self.upgrade().unwrap(), &c1), true);				
		assert_eq!(c1.borrow_mut().childindex(), usize::MAX);
		assert_eq!(child2.borrow().childindex(), 0);
		assert_eq!(root.borrow().children_count(), 1);	

		let childindex = child2.borrow().childindex();
		let c2 = root.borrow_mut().remove(childindex);
		assert_eq!(Rc::ptr_eq(&child2.borrow().weak_self.upgrade().unwrap(), &c2), true);				
		assert_eq!(c2.borrow_mut().childindex(), usize::MAX);
		assert_eq!(root.borrow().children_count(), 0);	
	}

	#[test]
	pub fn tree_insert()
	{
		let root = TreeNode::new(None, usize::MAX, Box::new(MyWidget::new("root")));
		let child0 = TreeNode::new(None, usize::MAX, Box::new(MyWidget::new("child0")));
		let child1 = TreeNode::new(None, usize::MAX, Box::new(MyWidget::new("child1")));
		let child2 = TreeNode::new(None, usize::MAX, Box::new(MyWidget::new("child2")));
		let child3 = TreeNode::new(None, usize::MAX, Box::new(MyWidget::new("child3")));
		let child4 = TreeNode::new(None, usize::MAX, Box::new(MyWidget::new("child4")));

		root.borrow_mut().insert(0, &mut child4.borrow_mut());
		root.borrow_mut().insert(0, &mut child3.borrow_mut());
		root.borrow_mut().insert(0, &mut child2.borrow_mut());
		root.borrow_mut().insert(0, &mut child1.borrow_mut());
		root.borrow_mut().insert(0, &mut child0.borrow_mut());

		assert_eq!(child0.borrow().childindex(), 0);
		assert_eq!(Rc::ptr_eq(&child0.borrow().parent().unwrap(), &root), true);				
		assert_eq!(child1.borrow().childindex(), 1);
		assert_eq!(Rc::ptr_eq(&child1.borrow().parent().unwrap(), &root), true);				
		assert_eq!(child2.borrow().childindex(), 2);
		assert_eq!(Rc::ptr_eq(&child2.borrow().parent().unwrap(), &root), true);				
		assert_eq!(child3.borrow().childindex(), 3);
		assert_eq!(Rc::ptr_eq(&child3.borrow().parent().unwrap(), &root), true);				
		assert_eq!(child4.borrow().childindex(), 4);
		assert_eq!(Rc::ptr_eq(&child4.borrow().parent().unwrap(), &root), true);				
		assert_eq!(root.borrow().children_count(), 5);
	}

	#[test]
	pub fn tree_set_parent()
	{
		let root = TreeNode::new(None, usize::MAX, Box::new(MyWidget::new("root")));
		let child0 = TreeNode::new(None, usize::MAX, Box::new(MyWidget::new("child0")));
		let child1 = TreeNode::new(None, usize::MAX, Box::new(MyWidget::new("child1")));
		let child2 = TreeNode::new(None, usize::MAX, Box::new(MyWidget::new("child2")));
		let child3 = TreeNode::new(None, usize::MAX, Box::new(MyWidget::new("child3")));
		let child4 = TreeNode::new(None, usize::MAX, Box::new(MyWidget::new("child4")));

		child0.borrow_mut().set_parent(&mut root.borrow_mut(), 0);
		child1.borrow_mut().set_parent(&mut root.borrow_mut(), 0);
		child2.borrow_mut().set_parent(&mut root.borrow_mut(), 0);
		child3.borrow_mut().set_parent(&mut root.borrow_mut(), 0);
		child4.borrow_mut().set_parent(&mut root.borrow_mut(), 0);

		assert_eq!(child0.borrow().childindex(), 4);
		assert_eq!(Rc::ptr_eq(&child0.borrow().parent().unwrap(), &root), true);				
		assert_eq!(child1.borrow().childindex(), 3);
		assert_eq!(Rc::ptr_eq(&child1.borrow().parent().unwrap(), &root), true);				
		assert_eq!(child2.borrow().childindex(), 2);
		assert_eq!(Rc::ptr_eq(&child2.borrow().parent().unwrap(), &root), true);				
		assert_eq!(child3.borrow().childindex(), 1);
		assert_eq!(Rc::ptr_eq(&child3.borrow().parent().unwrap(), &root), true);				
		assert_eq!(child4.borrow().childindex(), 0);
		assert_eq!(Rc::ptr_eq(&child4.borrow().parent().unwrap(), &root), true);				
		assert_eq!(root.borrow().children_count(), 5);
	}
}	
