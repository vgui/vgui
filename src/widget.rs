#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use druid_shell::{ Region, KeyEvent, MouseEvent };
use piet_common::Piet;
use kurbo::Size;
use crate::tree2::{ TreeNode, Tree };


pub struct Panel
{
	tree : TreeNode,
}


pub trait Widget
{	
    fn on_idle(&mut self){}
    fn paint(&mut self, piet: &mut Piet<'_>, invalid: &Region){}
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



