#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::vec::Vec;
use std::sync::RwLock;

static ARENAS : RwLock<usize> = RwLock::new(0);


#[derive(Copy, Clone, Debug)]
pub struct Index 
{
	arena : usize,//Arena identifier from ARENAS
	age : usize,
	index : usize,
}

impl Index
{
	pub fn new(arena : usize, age : usize, index : usize) -> Self
	{
		Index
		{
			arena,
			age,
			index,
		}
	}

	//Arena ID must be unchangable.
	pub fn arena_id(&self) -> usize
	{
		self.index
	}

	pub fn age(&self) -> usize
	{
		self.index
	}

	pub fn index(&self) -> usize
	{
		self.index
	}

	pub fn set_age(&mut self, age : usize)
	{
		self.age = age
	}

	pub fn set_index(&mut self, index : usize)
	{
		self.index = index
	}

	pub fn set(&mut self, age : usize, index : usize)
	{
		self.age = age;
		self.index = index
	}	
}

impl PartialEq for Index 
{
    fn eq(&self, other: &Self) -> bool 
    {
        self.arena == other.arena && self.age == other.age && self.index == other.index
    }
}

pub struct Arena<T>
{
	initialized : bool,//For static instances, see Arena::new below
	id : usize,//Arena identifier from ARENAS
	chunk_size : usize,
	heap : Vec<Vec<Option<T>>>,
	freed : Vec<Index>,
	current_age : usize,
	next_index : usize,
}

impl<T> Arena<T> 
{	
	//For static instances of Arena we need const construcor. Use Arena::init for initialization.
    pub const fn new() -> Self 
    { 
    	Self 
		{
			initialized : false,
			id : 0,
			chunk_size : 0,
			heap : Vec::new(),
			freed : Vec::new(),
			current_age : 0,
			next_index : 0,			
		}
    }

	pub fn init(&mut self, chunk_size : usize) -> &mut Self 
	{	
		if self.initialized == false
		{	
			let mut arena_id = ARENAS.write().unwrap();
			*arena_id += 1;

			self.id = *arena_id;
			self.chunk_size = chunk_size;
			self.heap.push(Vec::new());//Initialized in Arena::new
			//self.freed;//Initialized in Arena::new
			self.current_age = 0;
			self.next_index = 0;
		}

		self		
	}

	pub fn initialized(&self) -> bool
	{
		self.initialized
	}

	pub fn id(&self) -> usize
	{
		self.id
	}

	pub fn chunk_size(&self) -> usize
	{
		self.chunk_size
	}

	pub fn ages(&self) -> usize
	{
		self.heap.len()
	}

	pub fn used(&self) -> usize
	{
		let mut used : usize = 0;
		
		for chunk in &self.heap
		{
			for i in chunk
			{
				if i.is_some()
				{
					used +=1;
				}
			}
		}
		
		used
	}

	pub fn freed(&self) -> usize
	{
		self.freed.len()
	}

	pub fn alloc(&mut self, obj : T) -> Index
	{		
		//Chunk is full, need to alloc new chunk.
		if self.next_index == self.chunk_size 
		{
			self.heap.push(Vec::new());		
			self.next_index = 0;
			self.current_age += 1;						
			self.heap[self.current_age].reserve(self.chunk_size);
		}		

		if self.freed.len() == 0  
		{
			self.heap[self.current_age].push(Some(obj));
			let index = Index::new(self.id(), self.current_age, self.next_index);
			self.next_index += 1;					
			index
		}
		else 
		{
			let index = self.freed.pop().unwrap();
			self.heap[index.age][index.index] = Some(obj);
			index
		}
	}

	fn check_index(&self, index : Index) -> bool
	{		
		if self.id == index.arena
			&& index.age < self.heap.len()
				&& index.index < self.heap[index.age].len()					
		{
			true
		}
		else
		{
			false
		}
	}	

	pub fn free(&mut self, index : Index) 
	{
		if self.check_index(index) == false && self.get(index).is_some() == false
		{
			panic!("Wrong Arena index for freeing !")
		}

		self.heap[index.age][index.index].take().unwrap();
		self.freed.push(index);

		println!("Free index {},{}", index.age, index.index);
	}	

	pub fn get(&mut self, index : Index) -> Option<&mut T>
	{
		self.heap[index.age][index.index].as_mut()
	}	

	pub fn get_mut(&mut self, age : usize, index : usize) -> Option<&mut T>
	{
		self.heap[age][index].as_mut()
	}

}

impl<T> std::ops::Index<Index> for Arena<T> 
{
    type Output = T;

    fn index(&self, index : Index) -> &Self::Output 
    {
		if self.check_index(index) == false
    	{
    		panic!("Invalid index for Arena !")
    	}

    	&self.heap[index.age][index.index].as_ref().unwrap()
    }
}

impl<T> std::ops::IndexMut<Index> for Arena<T> 
{
    fn index_mut(&mut self, index : Index) -> &mut Self::Output 
    {
		if self.check_index(index) == false
    	{
    		panic!("Invalid index for Arena !")
    	}

    	self.heap[index.age][index.index].as_mut().unwrap()
    }
}


#[cfg(test)]
mod tests 
{
	use super::*;
    use crate::arena::{Index, Arena};

    const TEST_ARENA_CHUNK_SIZE : usize = 64;

    #[derive(Debug)]
    struct MyStruct
    {
    	x : usize,
    	y : String,
    }

    impl MyStruct
    {
    	pub fn new(x : usize, y : &str) -> Self
    	{
    		MyStruct
    		{
    			x : x,
    			y : y.to_string(), 
    		}
    	}
    }

	impl PartialEq for MyStruct 
	{
    	fn eq(&self, other: &Self) -> bool 
    	{
    		if self.x == other.x && self.y == other.y
    		{
    			true
    		}
    		else 
    		{
    			false
    		}
    	}
	}

    #[test]
    fn arena_new() 
    {
        let mut arena = Arena::<MyStruct>::new();
        arena.init(TEST_ARENA_CHUNK_SIZE);

        assert_eq!(arena.heap.len(), 1);
        assert_eq!(arena.freed.len(), 0);
        assert_eq!(arena.current_age, 0);
        assert_eq!(arena.next_index, 0);        
    }

   #[test]
    fn arena_alloc() 
    {
        let mut arena = Arena::<MyStruct>::new();
        arena.init(TEST_ARENA_CHUNK_SIZE);
        let index = arena.alloc(MyStruct::new(16838, "All is fine"));

        assert_eq!(arena.heap.len(), 1);
        assert_eq!(arena.heap[0].len(), 1);
        assert_eq!(arena.freed.len(), 0);
        assert_eq!(arena.current_age, 0);
        assert_eq!(arena.next_index, 1);  
        assert_eq!(index.age, 0);
        assert_eq!(index.index, 0);
    }     

    #[test]
    fn arena_alloc5() 
    {
        let mut arena = Arena::<MyStruct>::new();
        arena.init(TEST_ARENA_CHUNK_SIZE);
        let index0 = arena.alloc(MyStruct::new(0, "All is fine 0"));
        let index1 = arena.alloc(MyStruct::new(1, "All is fine 1"));
        let index2 = arena.alloc(MyStruct::new(2, "All is fine 2"));
        let index3 = arena.alloc(MyStruct::new(3, "All is fine 3"));
        let index4 = arena.alloc(MyStruct::new(4, "All is fine 4"));

        assert_eq!(arena.heap.len(), 1);
        assert_eq!(arena.heap[0].len(), 5);
        assert_eq!(arena.freed.len(), 0);
        assert_eq!(arena.current_age, 0);
        assert_eq!(arena.next_index, 5);

        assert_eq!(arena.get(index0) , Some(&mut MyStruct::new(0, "All is fine 0")));
        assert_eq!(index0.age, 0); assert_eq!(index0.index, 0);

        assert_eq!(arena.get(index1) , Some(&mut MyStruct::new(1, "All is fine 1")));
        assert_eq!(index1.age, 0); assert_eq!(index1.index, 1);
        
        assert_eq!(arena.get(index2) , Some(&mut MyStruct::new(2, "All is fine 2")));
        assert_eq!(index2.age, 0); assert_eq!(index2.index, 2);
        
        assert_eq!(arena.get(index3) , Some(&mut MyStruct::new(3, "All is fine 3")));
        assert_eq!(index3.age, 0); assert_eq!(index3.index, 3);
        
        assert_eq!(arena.get(index4) , Some(&mut MyStruct::new(4, "All is fine 4")));
        assert_eq!(index4.age, 0); assert_eq!(index4.index, 4);
	}         

	//Alloc 'n' objects in a new Arena
	//For more test accuracy need MyStruct::new(i,"All is fine")
	fn arena_alloc_n(n : usize) -> (Arena<MyStruct>, Vec<Index>)
	{
        let mut arena = Arena::<MyStruct>::new();
        arena.init(TEST_ARENA_CHUNK_SIZE);
        let mut indexs = Vec::new();

        for i in 0..n
        {
        	//For more test accuracy need MyStruct::new(i,"All is fine")
        	indexs.push(arena.alloc(MyStruct::new(i, "All is fine")));
        }

        (arena, indexs)
	}

    #[test]
    fn arena_alloc_chunk_size() 
    {
    	//We force to alloc new chunk
        let (arena, indexs) = arena_alloc_n(TEST_ARENA_CHUNK_SIZE + 1);

        assert_eq!(arena.heap.len(), 2);//Two chunks in a heap
        assert_eq!(arena.heap[0].len(), TEST_ARENA_CHUNK_SIZE);
        assert_eq!(arena.heap[1].len(), 1);
        assert_eq!(arena.freed.len(), 0);
        assert_eq!(arena.current_age, 1);
        assert_eq!(arena.next_index, 1);
        assert_eq!(indexs.len(), TEST_ARENA_CHUNK_SIZE + 1);  
        assert_eq!(indexs[TEST_ARENA_CHUNK_SIZE - 1].age , 0);
        assert_eq!(indexs[TEST_ARENA_CHUNK_SIZE - 1].index , TEST_ARENA_CHUNK_SIZE - 1);
        assert_eq!(indexs[TEST_ARENA_CHUNK_SIZE].age , 1);
        assert_eq!(indexs[TEST_ARENA_CHUNK_SIZE].index , 0);
    }             

    #[test]
    fn arena_alloc_check_index() 
    {
    	//We force to alloc new chunk
        let (arena, indexs) = arena_alloc_n(TEST_ARENA_CHUNK_SIZE + 1);

        assert_eq!(arena.heap.len(), 2);//Two chunks in a heap
        assert_eq!(arena.heap[0].len(), TEST_ARENA_CHUNK_SIZE);
        assert_eq!(arena.heap[1].len(), 1);
        assert_eq!(arena.freed.len(), 0);
        assert_eq!(arena.current_age, 1);
        assert_eq!(arena.next_index, 1);  

        let first0 = Index{arena : arena.id(), age : 0, index : 0};
        let last0 = Index{arena : arena.id(), age : 0, index : TEST_ARENA_CHUNK_SIZE - 1};
        let after_last0 = Index{arena : arena.id(), age : 0, index : TEST_ARENA_CHUNK_SIZE};

        let first1 = Index{arena : arena.id(), age : 1, index : 0};
        let last1 = Index{arena : arena.id(), age : 1, index : 0};
        let after_last1 = Index{arena : arena.id(), age : 1, index : 1};

        let fake_index = Index{arena : 33, age : 0, index : 0};

        assert_eq!(arena.check_index(first0), true);
        assert_eq!(arena.check_index(last0), true);
        assert_eq!(arena.check_index(after_last0), false);

        assert_eq!(arena.check_index(first1), true);
        assert_eq!(arena.check_index(last1), true);
        assert_eq!(arena.check_index(after_last1), false);        

        assert_eq!(arena.check_index(fake_index), false);

        let mut age = 0;
        let mut index = 0;
        for i in 0..TEST_ARENA_CHUNK_SIZE+1
        {
        	assert_eq!(indexs[i].age, age);
        	assert_eq!(indexs[i].index, index);

        	index += 1;
			if index == TEST_ARENA_CHUNK_SIZE 
			{ 
				age +=1;
				index = 0;
			}
        }        
    }             

    #[test]
    fn arena_free_and_alloc_after_free() 
    {
		let (mut arena, indexs) = arena_alloc_n(100 * TEST_ARENA_CHUNK_SIZE + 1);

        assert_eq!(arena.heap.len(), 101);
        assert_eq!(arena.heap[0].len(), TEST_ARENA_CHUNK_SIZE);
        assert_eq!(arena.freed.len(), 0);
        assert_eq!(arena.current_age, 100);
        assert_eq!(arena.next_index, 1);

		let index1 = Index{arena : arena.id(), age : 13, index : 13};
		arena.free(index1);
		assert_eq!(arena.freed.len(), 1);
		assert_eq!(arena.freed[0], index1);		

		assert_eq!(arena.heap[13][12], Some(MyStruct::new(13*TEST_ARENA_CHUNK_SIZE+12, "All is fine")));
		assert_eq!(arena.get(index1), None);
		assert_eq!(arena.heap[13][14], Some(MyStruct::new(13*TEST_ARENA_CHUNK_SIZE+14, "All is fine")));

		let index2 = Index{arena : arena.id(), age : 100, index : 0};
		arena.free(index2);
		assert_eq!(arena.freed.len(), 2);
		assert_eq!(arena.freed[1], index2);		

		assert_eq!(arena.heap[99][TEST_ARENA_CHUNK_SIZE - 1], Some(MyStruct::new(99*TEST_ARENA_CHUNK_SIZE+63, "All is fine")));
		assert_eq!(arena.get(index2), None);

		//alloc after free
		let new_index1 = arena.alloc(MyStruct::new(777, "All is fine"));
		assert_eq!(index2, new_index1);
		assert_eq!(arena.get(index2), Some(&mut MyStruct::new(777, "All is fine")));
		assert_eq!(arena.freed.len(), 1);

		let new_index2 = arena.alloc(MyStruct::new(888, "All is fine"));
		assert_eq!(index1, new_index2);
		assert_eq!(arena.get(index1), Some(&mut MyStruct::new(888, "All is fine")));		
		assert_eq!(arena.freed.len(), 0);
    }         
}//mod tests
