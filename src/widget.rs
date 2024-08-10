#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]


use std::any::Any;
use std::slice::{Iter, IterMut};
use druid_shell::{ Region, KeyEvent, MouseEvent };
use kurbo::{Line, Size, Point, Shape, Circle, BezPath, PathEl};
use piet_common::{RenderContext};
use crate::tree::{ RcRefCell, TreeNode, Tree };



pub struct WidgetBase
{
	tree : RcRefCell<TreeNode>,
    shape : BezPath,
}

pub struct Panel
{
    base : WidgetBase,
}

impl Panel
{
    pub fn new()
    {
        
    }
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
        self.base.tree.borrow_mut().remove(childindex)
    }

    fn insert(&mut self,  childindex : usize, child : &mut TreeNode)
    {
        self.base.tree.borrow_mut().insert(childindex, child)
    }

    fn set_parent(&mut self, newparent : &mut TreeNode, childindex : usize)
    {
        self.base.tree.borrow_mut().set_parent(newparent, childindex)
    }

    fn iter(&self) -> Iter<'_, RcRefCell<TreeNode>>
    {
        let i = self.base.tree.borrow();
        i.iter()
    }

    fn iter_mut(&mut self) -> IterMut<'_, RcRefCell<TreeNode>>
    {
        self.base.tree.borrow_mut().iter_mut()
    }

    fn parent(&self) -> Option<RcRefCell<TreeNode>>
    {
        self.base.tree.borrow().parent()
    }

    fn child(&self, index : usize) -> Option<RcRefCell<TreeNode>>
    {
        self.base.tree.borrow().child(index)
    }

    fn childindex(&self) -> usize
    {
        self.base.tree.borrow().childindex()
    }

    fn children_count(&self) -> usize
    {
        self.base.tree.borrow().children_count()
    }

    fn data(&self) -> &Box<dyn Any>
    {
        self.base.tree.borrow().data()
    }

    fn data_mut(&mut self) -> &mut Box<dyn Any>
    {
        let mut r = self.base.tree.borrow_mut();
        r.data_mut()
    }
    
}
