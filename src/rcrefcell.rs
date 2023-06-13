use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;


#[derive(Debug)]
pub struct RcRefCell<T>(Rc<RefCell<T>>);

impl<T> RcRefCell<T>
{
	pub fn new(arg : T) -> Self
	{
		RcRefCell
		{
			0 : Rc::new(RefCell::new(arg))
		}
	}
}

impl<T> Clone for RcRefCell<T>
{
    fn clone(&self) -> Self 
    {
    	let other = self.0.clone();

    	Self
    	{
        	0 : other, 	
        }
    }
}

impl<T> PartialEq for RcRefCell<T>
{
    fn eq(&self, other: &Self) -> bool 
    {
    	Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Eq for RcRefCell<T> {}

impl<T> Hash for RcRefCell<T>
{
    fn hash<H: Hasher>(&self, state: &mut H) 
    {
        std::ptr::hash(&**self, state)
    }
}

impl<T> Deref for RcRefCell<T> 
{
    type Target = Rc<RefCell<T>>;

    fn deref(&self) -> &Self::Target 
    {
        &self.0
    }
}
