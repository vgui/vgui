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
	pub fn new_root(data : T) -> RcRefCell<Self>
	{
		let root = Rc::new(RefCell::new(
			Self
			{
				weak_self : Weak::new(),
				parent : Weak::new(),
				children : Vec::new(),
				childindex : usize::MAX,
				data,
			}));

		root.borrow_mut().weak_self = Rc::downgrade(&root);	
		root
	}

	pub fn new(parent : &mut RcRefCell<Self>, data : T) -> RcRefCell<Self>
	{
		let child = Rc::new(RefCell::new(
			Self
			{
				weak_self : Weak::new(),
				parent : Rc::downgrade(parent),
				children : Vec::new(),
				childindex : usize::MAX,
				data,
			}));
		
		child.borrow_mut().weak_self = Rc::downgrade(&child);
		let length = parent.borrow_mut().children.len();	
		parent.borrow_mut().insert_child(length, Rc::clone(&child));		
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

    fn _update_indexes(&mut self)
    {
    	let mut index : usize = 0;

        while index < self.children.len()
        {
            self.children[index].borrow_mut().childindex = index;
            index += 1
        }
    }

    fn remove_child(&mut self, childindex : usize) -> RcRefCell<Self>
    {
        if childindex >= self.children.len()
        {
            panic!("To big index for removing.");
        }
      
       	let child = self.children.remove(childindex);
       	child.borrow_mut().parent = Weak::new();
       	child.borrow_mut().childindex = usize::MAX;
        
        self._update_indexes();
        child
    }

    fn insert_child(&mut self,  childindex : usize, child : RcRefCell<Self>)
    {
        if let Some(parent) = child.borrow_mut().parent()
        {
        	parent.borrow_mut().remove_child(child.borrow_mut().childindex);
        }

        if childindex >= self.children.len()
        {
            panic!("Too big index for inserting.");
        }

        child.borrow_mut().parent = Weak::clone(&self.weak_self);
        self.children.insert(childindex, child);        
        self._update_indexes();
    }

}


#[cfg(test)]
mod tests 
{
	use super::*;

	struct WidgetObj;

	fn tree_new_widgetobj(parent : Option<&TreeNodePtr<WidgetObj>>) -> TreeNodePtr<WidgetObj>
	{
		let widgetobj = Rc::new(RefCell::new(
			TreeNode::<WidgetObj>
			{
				weak_self : Weak::new(),
				parent : match parent
				{
					Some(parent) => Rc::downgrade(&parent),
					None => Weak::new()
				},
				children : Vec::new(),
				childindex : usize::MAX,
				data : WidgetObj,
			}));

		widgetobj.borrow_mut().weak_self = Rc::downgrade(&widgetobj);
		widgetobj
	}

	fn tree_insert_child(parent : &TreeNodePtr<WidgetObj>, child :&TreeNodePtr<WidgetObj>)
	{
		parent.borrow_mut().children.push(Rc::clone(&child));
		child.borrow_mut().parent = Rc::downgrade(&parent);
	}

    #[test]
    fn tree_update_indexes()
    {
    	let root = TreeNode::new_root(WidgetObj);		

    	let child0 = tree_new_widgetobj(Some(&root));
    	let child1 = tree_new_widgetobj(Some(&root));
    	let child2 = tree_new_widgetobj(Some(&root));

    	tree_insert_child(&root, &child0);
    	tree_insert_child(&root, &child1);
    	tree_insert_child(&root, &child2);
    	root.borrow_mut()._update_indexes();

		assert_eq!(root.borrow().children().len(), 3);
 		assert_eq!(Rc::ptr_eq(&child0, root.borrow().child(0).unwrap()), true);
 		assert_eq!(Rc::ptr_eq(&child1, root.borrow().child(1).unwrap()), true);
 		assert_eq!(Rc::ptr_eq(&child2, root.borrow().child(2).unwrap()), true);
    }

    #[test]
    fn tree_remove_first()
    {
    	let root = TreeNode::new_root(WidgetObj);
    	let child0 = tree_new_widgetobj(Some(&root));
    	let child1 = tree_new_widgetobj(Some(&root));
    	let child2 = tree_new_widgetobj(Some(&root));
    	let child3 = tree_new_widgetobj(Some(&root));

    	tree_insert_child(&root, &child0);
    	tree_insert_child(&root, &child1);
    	tree_insert_child(&root, &child2);
    	tree_insert_child(&root, &child3);

		assert_eq!(root.borrow().children().len(), 4);

		//remove first
		let c0 = root.borrow_mut().remove_child(0);
		assert_eq!(Rc::ptr_eq(&c0, &child0), true);		
		assert_eq!(root.borrow().children().len(), 3);

		//test children
		assert_eq!(Rc::ptr_eq(&root.borrow().child(0).unwrap(), &child1), true);
		assert_eq!(Rc::ptr_eq(&root.borrow().child(1).unwrap(), &child2), true);
		assert_eq!(Rc::ptr_eq(&root.borrow().child(2).unwrap(), &child3), true);
    }

    #[test]
    fn tree_remove_last()
    {
    	let root = TreeNode::new_root(WidgetObj);
    	let child0 = tree_new_widgetobj(Some(&root));
    	let child1 = tree_new_widgetobj(Some(&root));
    	let child2 = tree_new_widgetobj(Some(&root));
    	let child3 = tree_new_widgetobj(Some(&root));

    	tree_insert_child(&root, &child0);
    	tree_insert_child(&root, &child1);
    	tree_insert_child(&root, &child2);
    	tree_insert_child(&root, &child3);

		assert_eq!(root.borrow().children().len(), 4);

		//remove last
		let c3 = root.borrow_mut().remove_child(3);
		assert_eq!(Rc::ptr_eq(&c3, &child3), true);		
		assert_eq!(root.borrow().children().len(), 3);

		//test children
		assert_eq!(Rc::ptr_eq(&root.borrow().child(0).unwrap(), &child0), true);
		assert_eq!(Rc::ptr_eq(&root.borrow().child(1).unwrap(), &child1), true);
		assert_eq!(Rc::ptr_eq(&root.borrow().child(2).unwrap(), &child2), true);
    }

    #[test]
    fn tree_remove_middle()
    {
    	let root = TreeNode::new_root(WidgetObj);
    	let child0 = tree_new_widgetobj(Some(&root));
    	let child1 = tree_new_widgetobj(Some(&root));
    	let child2 = tree_new_widgetobj(Some(&root));
    	let child3 = tree_new_widgetobj(Some(&root));
    	let child4 = tree_new_widgetobj(Some(&root));

    	tree_insert_child(&root, &child0);
    	tree_insert_child(&root, &child1);
    	tree_insert_child(&root, &child2);
    	tree_insert_child(&root, &child3);
    	tree_insert_child(&root, &child4);

		assert_eq!(root.borrow().children().len(), 5);

		//remove middle
		let c2 = root.borrow_mut().remove_child(2);
		assert_eq!(Rc::ptr_eq(&c2, &child2), true);		
		assert_eq!(root.borrow().children().len(), 4);

		//test children
		assert_eq!(Rc::ptr_eq(&root.borrow().child(0).unwrap(), &child0), true);
		assert_eq!(Rc::ptr_eq(&root.borrow().child(1).unwrap(), &child1), true);
		assert_eq!(Rc::ptr_eq(&root.borrow().child(2).unwrap(), &child3), true);
		assert_eq!(Rc::ptr_eq(&root.borrow().child(3).unwrap(), &child4), true);
    }

    #[test]
    fn tree_remove_all()
    {
    	let root = TreeNode::new_root(WidgetObj);
    	let child0 = tree_new_widgetobj(Some(&root));
    	let child1 = tree_new_widgetobj(Some(&root));
    	let child2 = tree_new_widgetobj(Some(&root));
    	let child3 = tree_new_widgetobj(Some(&root));
    	let child4 = tree_new_widgetobj(Some(&root));

    	tree_insert_child(&root, &child0);
    	tree_insert_child(&root, &child1);
    	tree_insert_child(&root, &child2);
    	tree_insert_child(&root, &child3);
    	tree_insert_child(&root, &child4);

		assert_eq!(root.borrow().children().len(), 5);

		//remove middle
		let c2 = root.borrow_mut().remove_child(2);
		assert_eq!(Rc::ptr_eq(&c2, &child2), true);		
		assert_eq!(root.borrow().children().len(), 4);

		//remove first
		let c0 = root.borrow_mut().remove_child(0);
		assert_eq!(Rc::ptr_eq(&c0, &child0), true);		
		assert_eq!(root.borrow().children().len(), 3);

		//remove last
		let c4 = root.borrow_mut().remove_child(2);
		assert_eq!(Rc::ptr_eq(&c4, &child4), true);		
		assert_eq!(root.borrow().children().len(), 2);

		//remove first
		let c1 = root.borrow_mut().remove_child(0);
		assert_eq!(Rc::ptr_eq(&c1, &child1), true);		
		assert_eq!(root.borrow().children().len(), 1);

		//remove last
		let c3 = root.borrow_mut().remove_child(0);
		assert_eq!(Rc::ptr_eq(&c3, &child3), true);		
		assert_eq!(root.borrow().children().len(), 0);
    }

    #[test]
	pub fn tree_new_root()
	{
		let root = TreeNode::new_root(WidgetObj);

		assert_eq!(root.borrow().parent.strong_count(), 0);
		assert_eq!(root.borrow().parent.weak_count(), 0);
		assert_eq!(root.borrow().children.len(), 0);
		assert_eq!(root.borrow().childindex, usize::MAX);
	}
/*
	#[test]
	pub fn tree_new()
	{
		let mut root = TreeNode::new_root(WidgetObj);
		let child = TreeNode::new(&mut root, WidgetObj);

		assert_eq!(root.borrow().parent.strong_count(), 0);
		assert_eq!(root.borrow().parent.weak_count(), 0);
		assert_eq!(root.borrow().children.len(), 1);
		assert_eq!(root.borrow().childindex, usize::MAX);

		assert_eq!(child.borrow().parent.strong_count(), 1);
		assert_eq!(child.borrow().parent.weak_count(), 1);
		assert_eq!(Rc::ptr_eq(&child.borrow().parent.upgrade().unwrap(), &root), true);		
		assert_eq!(child.borrow().children.len(), 0);
		assert_eq!(child.borrow().childindex, 0);
	}*/

}