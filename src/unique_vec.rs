#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::hash::Hash;
use std::cmp::{PartialEq, Eq};
use std::collections::HashSet;
use std::ops::Index;
use std::vec::Vec;
use crate::rcrefcell;


struct UniqueVec<'a, T> 
{
    vec: Vec<T>,
    set: &'a mut HashSet<T>,
}

impl<'a, T: Clone + PartialEq + Eq + Hash + 'a> UniqueVec<'a, T> 
{
    pub fn new(set: &'a mut HashSet<T>) -> Self 
    {
        UniqueVec {
            vec: Vec::new(),
            set: set,
        }
    }

    pub fn find(&mut self, value: T) -> usize 
    {
        self.vec.iter().position(|v| v == &value).unwrap()
    }

    pub fn insert(&mut self, index: usize, value : &T) 
    {
        if self.set.insert(value.clone()) 
        {
            self.vec.insert(index, value.clone());
        }
    }
}

impl<'a, T> Index<usize> for UniqueVec<'a, T> 
{
    type Output = T;

    fn index(&self, i : usize) -> &Self::Output 
    {
    	&self.vec[i]
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use crate::rcrefcell::RcRefCell;


    #[derive(Debug)]
	struct WidgetObj 
    {
        id: String,
    }

    impl WidgetObj 
    {
        pub fn new(id: &str) -> Self 
        {
            WidgetObj 
            {
                id: String::from(id),
            }
        }
    }

    #[test]
    pub fn unique_vec_find() 
    {
    	let w0 = RcRefCell::new(WidgetObj::new("widget0"));
        let w1 = RcRefCell::new(WidgetObj::new("widget1"));
        let w2 = RcRefCell::new(WidgetObj::new("widget2"));

        let mut set = HashSet::<RcRefCell<WidgetObj>>::new();
        let mut vec = UniqueVec::<RcRefCell<WidgetObj>>::new(&mut set);

        vec.insert(0, &w0);
        vec.insert(1, &w1);
        vec.insert(2, &w2);

        assert_eq!(&vec[0], &w0);
        //println!("vec - {} , widget - {}", &vec[0], &w0);
        assert_eq!(&vec[1], &w1);
        assert_eq!(&vec[2], &w2);
    }
}
