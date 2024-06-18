#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::vec::Vec;
use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref, RefMut};
use std::any::Any;
use std::ops::{Deref, DerefMut};


type RcRefCell<T> = Rc<RefCell<T>>;
type WeakRefCell<T> = Weak<RefCell<T>>;
type TreePtr<T> = RcRefCell<TreeNode<T>>;


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
	pub fn new(parent : Option<RcRefCell<Self>>,childindex : usize , data : T) -> RcRefCell<Self>
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

    pub fn remove(&mut self, child : &mut Self) -> RcRefCell<TreeNode<T>>
    {
    	//Check child index.
		let childindex = child.childindex;

        if childindex >= self.children.len()
        {
            panic!("Too big index for removing.");
        }
      
		child.parent = Weak::new();
       	child.childindex = usize::MAX;

		self.children.remove(childindex);
        self.update_indexes(childindex);

        child.weak_self.upgrade().unwrap()
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
	       	//parent.borrow_mut().remove(child.childindex);
			self.remove(child);
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
			parent.borrow_mut().remove(self);
		}

		newparent.insert(childindex, self);		
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

	pub fn data(&self) -> &T
	{
		&self.data
	}	

	pub fn data_mut(&mut self) -> &mut T
	{
		&mut self.data
	}		
}


pub trait Tree<T>
{	
    fn remove(&mut self, childindex : usize) -> RcRefCell<TreeNode<T>>;
    fn insert(&mut self,  childindex : usize, child : &mut Self);
 	fn set_parent(&mut self, newparent : &mut Self, childindex : usize);
	fn parent(&self) -> Option<RcRefCell<Self>>;
	fn child(&self, index : usize) -> Option<&RcRefCell<Self>>;
	fn childindex(&self) -> usize;
	fn children_count(&self) -> usize;
	fn data(&self) -> &T;
	fn data_mut(&mut self) -> &mut T;
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
	pub fn tree_update_indexes()
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
		assert_eq!(root.borrow().children_count(), 0);
		assert_eq!(root.borrow().childindex(), usize::MAX);
		assert_eq!(root.borrow().data().id, "root");

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
		root.borrow_mut().update_indexes(0);

		assert_eq!(Rc::ptr_eq(&child0.borrow().weak_self.upgrade().unwrap(), &child0), true);		
		assert_eq!(child0.borrow().weak_self.strong_count(), 2);//1.root.children[0]; 2.child0
		assert_eq!(child0.borrow().weak_self.weak_count(), 1);
		assert_eq!(child0.borrow().parent.strong_count(), 1);
		assert_eq!(child0.borrow().parent.weak_count(), 2);//1.root.weak_self; 2.child0.parent;
		assert_eq!(child0.borrow().children_count(), 0);
		assert_eq!(child0.borrow().childindex(), 0);
		assert_eq!(child0.borrow().data().id, "child0");
		assert_eq!(root.borrow().children_count(), 1);		

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
		root.borrow_mut().update_indexes(0);

		assert_eq!(Rc::ptr_eq(&child1.borrow().weak_self.upgrade().unwrap(), &child1), true);		
		assert_eq!(child1.borrow().weak_self.strong_count(), 2);//1.root.children[0]; 2.child1
		assert_eq!(child1.borrow().weak_self.weak_count(), 1);
		assert_eq!(child1.borrow().parent.strong_count(), 1);
		assert_eq!(child1.borrow().parent.weak_count(), 3);//1.root.weak_self; 2.child0.parent;3.child1.parent
		assert_eq!(child1.borrow().children_count(), 0);
		assert_eq!(child1.borrow().childindex(), 0);
		assert_eq!(child0.borrow().childindex(), 1);
		assert_eq!(child1.borrow().data().id, "child1");
		assert_eq!(root.borrow().children_count(), 2);						
	}

	#[test]
	pub fn tree_new()
	{
		let root = TreeNode::new(None,usize::MAX, WidgetObj::new("root"));

		assert_eq!(Rc::ptr_eq(&root.borrow().weak_self.upgrade().unwrap(), &root), true);		
		assert_eq!(root.borrow().weak_self.strong_count(), 1);
		assert_eq!(root.borrow().weak_self.weak_count(), 1);
		assert_eq!(root.borrow().parent.strong_count(), 0);
		assert_eq!(root.borrow().parent.weak_count(), 0);
		assert_eq!(root.borrow().children_count(), 0);
		assert_eq!(root.borrow().childindex(), usize::MAX);
		assert_eq!(root.borrow().data().id, "root");

		let child0 = TreeNode::new(Some(root.clone()),usize::MAX, WidgetObj::new("child0"));

		assert_eq!(Rc::ptr_eq(&child0.borrow().weak_self.upgrade().unwrap(), &child0), true);		
		assert_eq!(child0.borrow().weak_self.strong_count(), 2);//1.root.children[0]; 2.child0
		assert_eq!(child0.borrow().weak_self.weak_count(), 1);
		assert_eq!(child0.borrow().parent.strong_count(), 1);
		assert_eq!(child0.borrow().parent.weak_count(), 2);//1.root.weak_self; 2.child0.parent;
		assert_eq!(child0.borrow().children_count(), 0);
		assert_eq!(child0.borrow().childindex(), 0);
		assert_eq!(child0.borrow().data().id, "child0");
		assert_eq!(root.borrow().children_count(), 1);		

		let child1 = TreeNode::new(Some(root.clone()),usize::MAX, WidgetObj::new("child1"));

		assert_eq!(Rc::ptr_eq(&child1.borrow().weak_self.upgrade().unwrap(), &child1), true);		
		assert_eq!(child1.borrow().weak_self.strong_count(), 2);//1.root.children[1]; 2.child1
		assert_eq!(child1.borrow().weak_self.weak_count(), 1);
		assert_eq!(child1.borrow().parent.strong_count(), 1);
		assert_eq!(child1.borrow().parent.weak_count(), 3);//1.root.weak_self; 2.child0.parent; 3.child1.parent
		assert_eq!(child1.borrow().children_count(), 0);
		assert_eq!(child1.borrow().childindex(), 1);
		assert_eq!(child0.borrow().childindex(), 0);
		assert_eq!(child1.borrow().data().id, "child1");
		assert_eq!(root.borrow().children_count(), 2);						

		let child2 = TreeNode::new(Some(root.clone()),usize::MAX, WidgetObj::new("child2"));

		assert_eq!(Rc::ptr_eq(&child2.borrow().weak_self.upgrade().unwrap(), &child2), true);		
		assert_eq!(child2.borrow().weak_self.strong_count(), 2);//1.root.children[2]; 2.child2
		assert_eq!(child2.borrow().weak_self.weak_count(), 1);
		assert_eq!(child2.borrow().parent.strong_count(), 1);
		assert_eq!(child2.borrow().parent.weak_count(), 4);//1.root.weak_self; 2.child0.parent; 3.child1.parent; 4.child2.parent
		assert_eq!(child2.borrow().children_count(), 0);
		assert_eq!(child2.borrow().childindex(), 2);
		assert_eq!(child1.borrow().childindex(), 1);		
		assert_eq!(child0.borrow().childindex(), 0);
		assert_eq!(child2.borrow().data().id, "child2");
		assert_eq!(root.borrow().children_count(), 3);						
	}

	#[test]
	pub fn tree_remove()
	{
		let root = TreeNode::new(None,usize::MAX, WidgetObj::new("root"));
		let child0 = TreeNode::new(Some(root.clone()),usize::MAX, WidgetObj::new("child0"));
		let child1 = TreeNode::new(Some(root.clone()),usize::MAX, WidgetObj::new("child1"));
		let child2 = TreeNode::new(Some(root.clone()),usize::MAX, WidgetObj::new("child2"));
		let child3 = TreeNode::new(Some(root.clone()),usize::MAX, WidgetObj::new("child3"));
		let child4 = TreeNode::new(Some(root.clone()),usize::MAX, WidgetObj::new("child4"));

		assert_eq!(child0.borrow().childindex(), 0);
		assert_eq!(child1.borrow().childindex(), 1);
		assert_eq!(child2.borrow().childindex(), 2);
		assert_eq!(child3.borrow().childindex(), 3);
		assert_eq!(child4.borrow().childindex(), 4);
		assert_eq!(root.borrow().children_count(), 5);

		let c0 = root.borrow_mut().remove(&mut child0.borrow_mut());
		assert_eq!(Rc::ptr_eq(&child0.borrow().weak_self.upgrade().unwrap(), &c0), true);				
		assert_eq!(c0.borrow_mut().childindex(), usize::MAX);
		assert_eq!(child1.borrow().childindex(), 0);
		assert_eq!(child2.borrow().childindex(), 1);		
		assert_eq!(child3.borrow().childindex(), 2);
		assert_eq!(child4.borrow().childindex(), 3);
		assert_eq!(root.borrow().children_count(), 4);

		let c3 = root.borrow_mut().remove(&mut child3.borrow_mut());
		assert_eq!(Rc::ptr_eq(&child3.borrow().weak_self.upgrade().unwrap(), &c3), true);				
		assert_eq!(c3.borrow_mut().childindex(), usize::MAX);
		assert_eq!(child1.borrow().childindex(), 0);
		assert_eq!(child2.borrow().childindex(), 1);		
		assert_eq!(child4.borrow().childindex(), 2);
		assert_eq!(root.borrow().children_count(), 3);

		let c4 = root.borrow_mut().remove(&mut child4.borrow_mut());
		assert_eq!(Rc::ptr_eq(&child4.borrow().weak_self.upgrade().unwrap(), &c4), true);				
		assert_eq!(c4.borrow_mut().childindex(), usize::MAX);
		assert_eq!(child1.borrow().childindex(), 0);
		assert_eq!(child2.borrow().childindex(), 1);
		assert_eq!(root.borrow().children_count(), 2);	

		let c1 = root.borrow_mut().remove(&mut child1.borrow_mut());
		assert_eq!(Rc::ptr_eq(&child1.borrow().weak_self.upgrade().unwrap(), &c1), true);				
		assert_eq!(c1.borrow_mut().childindex(), usize::MAX);
		assert_eq!(child2.borrow().childindex(), 0);
		assert_eq!(root.borrow().children_count(), 1);	

		let c2 = root.borrow_mut().remove(&mut child2.borrow_mut());
		assert_eq!(Rc::ptr_eq(&child2.borrow().weak_self.upgrade().unwrap(), &c2), true);				
		assert_eq!(c2.borrow_mut().childindex(), usize::MAX);
		assert_eq!(root.borrow().children_count(), 0);	
	}

	#[test]
	pub fn tree_insert()
	{
		let root = TreeNode::new(None, usize::MAX, WidgetObj::new("root"));
		let child0 = TreeNode::new(None, usize::MAX, WidgetObj::new("child0"));
		let child1 = TreeNode::new(None, usize::MAX, WidgetObj::new("child1"));
		let child2 = TreeNode::new(None, usize::MAX, WidgetObj::new("child2"));
		let child3 = TreeNode::new(None, usize::MAX, WidgetObj::new("child3"));
		let child4 = TreeNode::new(None, usize::MAX, WidgetObj::new("child4"));

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
		let root = TreeNode::new(None, usize::MAX, WidgetObj::new("root"));
		let child0 = TreeNode::new(None, usize::MAX, WidgetObj::new("child0"));
		let child1 = TreeNode::new(None, usize::MAX, WidgetObj::new("child1"));
		let child2 = TreeNode::new(None, usize::MAX, WidgetObj::new("child2"));
		let child3 = TreeNode::new(None, usize::MAX, WidgetObj::new("child3"));
		let child4 = TreeNode::new(None, usize::MAX, WidgetObj::new("child4"));

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


	pub trait Widget
	{	
		type TreeType;
		fn tree(&mut self) -> Self::TreeType;
    	fn paint(&mut self){}
		fn size(&mut self) {}
    	fn mouse_move(&mut self) {}
    	fn destroy(&mut self) {}
	}

	struct NullContainer;

	impl NullContainer
	{
		pub fn new() ->NullContainer
		{
			NullContainer{}
		}
	}

	struct Container
	{
		tree : RcRefCell<TreeNode<RcRefCell<Box<dyn Widget>>>>,
		id : String,		
	}

	impl Widget for NullContainer
	{
		type TreeType = RcRefCell<TreeNode<RcRefCell<dyn Widget>>>;
		fn tree(&mut self) -> RcRefCell<TreeNode<RcRefCell<dyn Widget>>>{  }
    	fn paint(&mut self){}
	    fn size(&mut self) {}
    	fn mouse_move(&mut self) {}
    	fn destroy(&mut self) {}
	}

	impl Widget for Container
	{	
		type TreeType = RcRefCell<TreeNode<RcRefCell<dyn Widget>>>;
		fn tree(&mut self) -> RcRefCell<TreeNode<RcRefCell<dyn Widget>>>{ self.tree }
    	fn paint(&mut self){}
	    fn size(&mut self) {}
    	fn mouse_move(&mut self) {}
    	fn destroy(&mut self) {}
	}

	impl Container
	{
		pub fn new(parent : Option<RcRefCell<TreeNode<RcRefCell<dyn Widget>>>>, id : &str) -> RcRefCell<dyn Widget>
		{
			let container : RcRefCell<dyn Widget> = Rc::new(RefCell::new(Container
			{
				tree : TreeNode::new(parent, 0, Rc::new(RefCell::new(NullContainer::new()))),
				id : String::from(id), 
			}));

			container.borrow_mut().tree.borrow_mut().data = Rc::clone(&container);
			container
		}
	}

	#[test]
	pub fn container_new()
	{
		let root = Container::new(None, "root");

		assert_eq!(Rc::ptr_eq(&root.tree.borrow().weak_self.upgrade().unwrap(), &root.tree), true);		
		assert_eq!(root.tree.borrow().weak_self.strong_count(), 1);
		assert_eq!(root.tree.borrow().weak_self.weak_count(), 1);
		assert_eq!(root.tree.borrow().parent.strong_count(), 0);
		assert_eq!(root.tree.borrow().parent.weak_count(), 0);
		assert_eq!(root.tree.borrow().children_count(), 0);
		assert_eq!(root.tree.borrow().childindex(), usize::MAX);
		assert_eq!(root.id, "root");

		let child0 = Container::new(Some(Rc::clone(&root.tree)), "child0");
		root.tree.borrow_mut().insert(0, &mut child0.tree.borrow_mut());
		   //root.borrow_mut().insert(0, &mut child0.borrow_mut());

		assert_eq!(Rc::ptr_eq(&child0.tree.borrow().weak_self.upgrade().unwrap(), &child0.tree), true);
		assert_eq!(child0.tree.borrow().weak_self.strong_count(), 2);//1.root.children[0]; 2.child0
		assert_eq!(child0.tree.borrow().weak_self.weak_count(), 1);
		assert_eq!(child0.tree.borrow().parent.strong_count(), 1);
		assert_eq!(child0.tree.borrow().parent.weak_count(), 2);//1.root.weak_self; 2.child0.parent;
		assert_eq!(child0.tree.borrow().children_count(), 0);
		assert_eq!(child0.tree.borrow().childindex(), 0);
		assert_eq!(child0.id, "child0");
		assert_eq!(root.tree.borrow().children_count(), 1);		

		let child1 = Container::new(Some(Rc::clone(&root.tree)), "child1");
		root.tree.borrow_mut().insert(0, &mut child1.tree.borrow_mut());

		assert_eq!(Rc::ptr_eq(&child1.tree.borrow().weak_self.upgrade().unwrap(), &child1.tree), true);
		assert_eq!(child1.tree.borrow().weak_self.strong_count(), 2);//1.root.children[0]; 2.child1
		assert_eq!(child1.tree.borrow().weak_self.weak_count(), 1);
		assert_eq!(child1.tree.borrow().parent.strong_count(), 1);
		assert_eq!(child1.tree.borrow().parent.weak_count(), 3);//1.root.weak_self; 2.child0.parent;3.child1.parent
		assert_eq!(child1.tree.borrow().children_count(), 0);
		assert_eq!(child1.tree.borrow().childindex(), 0);
		assert_eq!(child1.id, "child1");
		assert_eq!(root.tree.borrow().children_count(), 2);						
	}
}	
