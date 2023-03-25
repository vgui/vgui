#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::vec::Vec;
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::any::Any;


type TreeNodePtr<T> = Rc<RefCell<TreeNode::<T>>>;

pub struct TreeNode<T>
{
	parent : Weak<RefCell<TreeNode<T>>>,
	children : Vec<Rc<RefCell<TreeNode<T>>>>,
	childindex : usize,
	data : T,
}

impl<T> TreeNode<T>
{
	pub fn new_root(data : T) -> Rc<RefCell<Self>>
	{
		Rc::new(RefCell::new(
			Self
			{
				parent : Weak::new(),
				children : Vec::new(),
				childindex : usize::MAX,
				data,
			}))		
	}

	pub fn new(parent : &mut Rc<RefCell<Self>>, data : T) -> Rc<RefCell<Self>>
	{
		let child = Rc::new(RefCell::new(
			Self
			{
				parent : Rc::downgrade(parent),
				children : Vec::new(),
				childindex : usize::MAX,
				data,
			}));
				
		parent.borrow_mut().add_child(Rc::clone(&child));					
		child
	}


	fn parent(&self) -> Option<Rc<RefCell<Self>>>
	{
		self.parent.upgrade()
	}

	fn set_parent(&mut self, parent : Option<Rc<RefCell<Self>>>)
	{
		self.parent = match parent
		{
			Some(parent) => Rc::downgrade(&parent),
			None => Weak::new()
		};
	}

	fn child(&self, index : usize) -> Option<&Rc<RefCell<Self>>>
	{
		self.children.get(index)
	}

	fn children(&self) -> &Vec<Rc<RefCell<TreeNode<T>>>>
	{
		&self.children
	}

	fn children_mut(&mut self) -> &mut Vec<Rc<RefCell<TreeNode<T>>>>
	{
		&mut self.children
	}	

	fn add_child(&mut self, child : Rc<RefCell<Self>>)
	{
		self.children.push(child);
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

    fn remove_child(&mut self, childindex : usize) -> Rc<RefCell<Self>>
    {
        if childindex >= self.children.len()
        {
            panic!("Removing child with too large index.");
        }
      
       	let child = self.children.remove(childindex);
       	child.borrow_mut().parent = Weak::new();
       	child.borrow_mut().childindex = usize::MAX;
        
        self._update_indexes();
        child
    }

}


#[cfg(test)]
mod tests 
{
	use super::*;

	struct WidgetObj;

	fn tree_new_widgetobj(parent : Option<&TreeNodePtr<WidgetObj>>) -> TreeNodePtr<WidgetObj>
	{
		if parent.is_some()
		{
			Rc::new(RefCell::new(
				TreeNode::<WidgetObj>
				{
					parent : Rc::downgrade(&parent.unwrap()),
					children : Vec::new(),
					childindex : usize::MAX,
					data : WidgetObj,
				}))
		}
		else
		{
			Rc::new(RefCell::new(
				TreeNode::<WidgetObj>
				{
					parent : Weak::new(),
					children : Vec::new(),
					childindex : usize::MAX,
					data : WidgetObj,
				}))
		}
	}

	fn tree_insert_child(parent : &TreeNodePtr<WidgetObj>, child :&TreeNodePtr<WidgetObj>)
	{
		parent.borrow_mut().children.push(Rc::clone(&child));
	}

	#[test]
    fn tree_new() 
    {
    	let mut root = TreeNode::new_root(WidgetObj);

    	let child1 = TreeNode::new(&mut root, WidgetObj);
    	let child2 = TreeNode::new(&mut root, WidgetObj);
    	let child3 = TreeNode::new(&mut root, WidgetObj);
 		
 		assert_eq!(root.borrow().children().len(), 3);
 		assert_eq!(Rc::ptr_eq(&root, &child1.borrow().parent().unwrap()), true);
 		assert_eq!(Rc::ptr_eq(&root, &child2.borrow().parent().unwrap()), true);
 		assert_eq!(Rc::ptr_eq(&root, &child3.borrow().parent().unwrap()), true);
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
    	let child4 = tree_new_widgetobj(Some(&root));
    	let child5 = tree_new_widgetobj(Some(&root));
    	let child6 = tree_new_widgetobj(Some(&root));

    	tree_insert_child(&root, &child0);
    	tree_insert_child(&root, &child1);
    	tree_insert_child(&root, &child2);
    	tree_insert_child(&root, &child3);
    	tree_insert_child(&root, &child4);
    	tree_insert_child(&root, &child5);
    	tree_insert_child(&root, &child6);

		assert_eq!(root.borrow().children().len(), 7);

		//remove first
		let c0 = root.borrow_mut().remove_child(0);
		assert_eq!(Rc::ptr_eq(&c0, &child0), true);		
		assert_eq!(root.borrow().children().len(), 6);

		//test children
		assert_eq!(Rc::ptr_eq(&root.borrow().child(0).unwrap(), &child1), true);
		assert_eq!(Rc::ptr_eq(&root.borrow().child(1).unwrap(), &child2), true);
		assert_eq!(Rc::ptr_eq(&root.borrow().child(2).unwrap(), &child3), true);
		assert_eq!(Rc::ptr_eq(&root.borrow().child(3).unwrap(), &child4), true);
		assert_eq!(Rc::ptr_eq(&root.borrow().child(4).unwrap(), &child5), true);
		assert_eq!(Rc::ptr_eq(&root.borrow().child(5).unwrap(), &child6), true);

    }

}