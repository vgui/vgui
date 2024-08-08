#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]


use std::any::Any;
use std::slice::{Iter, IterMut};
use druid_shell::{ Region, KeyEvent, MouseEvent };
use ::kurbo::{Size, Shape};
use piet_common::{RenderContext};
use crate::tree::{ RcRefCell, TreeNode, Tree };



pub struct WidgetBase
{
	tree : TreeNode,
    //shape : Box<dyn Shape>,
}

pub struct Panel
{
    tree : TreeNode,
}

pub trait Widget : Tree
{	
    fn on_idle(&mut self){}
    fn paint(&mut self, piet: &mut piet_common::Piet, invalid: &Region)
    {
        for  i in self.iter_mut()
        {
            println!("{}", i.borrow().children_count());
        }
    }
    fn size(&mut self, size: Size) {}
    fn key_down(&mut self, event: KeyEvent) -> bool { false }
    fn key_up(&mut self, event: KeyEvent) {}
    fn wheel(&mut self, event: &MouseEvent) {}
    fn mouse_move(&mut self, event: &MouseEvent) {}
    fn mouse_down(&mut self, event: &MouseEvent) {}
    fn mouse_up(&mut self, event: &MouseEvent) {}
    fn got_focus(&mut self) {}
    fn lost_focus(&mut self) {}
    fn destroy(&mut self) {}

}


impl Tree for Panel
{
    fn remove(&mut self, childindex : usize) -> RcRefCell<TreeNode>
    {
        self.tree.remove(childindex)
    }

    fn insert(&mut self,  childindex : usize, child : &mut TreeNode)
    {
        self.tree.insert(childindex, child)
    }

    fn set_parent(&mut self, newparent : &mut TreeNode, childindex : usize)
    {
        self.tree.set_parent(newparent, childindex)
    }

    fn iter(&self) -> Iter<'_, RcRefCell<TreeNode>>
    {
        self.tree.iter()
    }

    fn iter_mut(&mut self) -> IterMut<'_, RcRefCell<TreeNode>>
    {
        self.tree.iter_mut()
    }

    fn parent(&self) -> Option<RcRefCell<TreeNode>>
    {
        self.tree.parent()
    }

    fn child(&self, index : usize) -> Option<&RcRefCell<TreeNode>>
    {
        self.tree.child(index)
    }

    fn childindex(&self) -> usize
    {
        self.tree.childindex()
    }

    fn children_count(&self) -> usize
    {
        self.tree.children_count()
    }

    fn data(&self) -> &Box<dyn Any>
    {
        self.tree.data()
    }

    fn data_mut(&mut self) -> &mut Box<dyn Any>
    {
        self.tree.data_mut()
    }
    
}
