#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::vec::Vec;
use std::sync::atomic::{AtomicUsize, Ordering};


// Arena identifier counter, increments in Arena::new .
static ARENA_ID : AtomicUsize = AtomicUsize::new(0);


// Arena is array of arrayes of objects and array of indexes of freed objects. 
// Index of Arena is an intermediate entity, to present real object.
// Chunk size is a size of array in array. Index of this chunk is age. And index of index is real object.
// Somthing like Arena[age][index] -> accsess to object.
// And Arena.freed[] - freed objects.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Index 
{
	arena_id : usize,// Arena identifier. Is being created in Arena::new by incrementing ARENA_ID.
	age : usize,
	index : usize,
}

// Index is persistent object, it cannot be changed after creation.
impl Index
{
	pub fn new(arena_id : usize, age : usize, index : usize) -> Self
	{
		Index
		{
			arena_id,
			age,
			index,
		}
	}

	//Arena ID must be unchangable.
	pub fn arena_id(&self) -> usize
	{
		self.arena_id
	}

	pub fn age(&self) -> usize
	{
		self.age
	}

	pub fn index(&self) -> usize
	{
		self.index
	}
}

#[derive(Debug, PartialEq)]
pub enum IndexStatus
{
	Freed,
	Allocated,
	NonUsed,
	Invalid,
}


pub struct Arena<T>
{
	id : usize,// Arena identifier. Is being created in Arena::new by incrementing ARENA_ID.
	chunk_size : usize,
	heap : Vec<Vec<Option<T>>>,
	last_freed : Vec<Index>,
	current_index : usize,
}


impl<T> Arena<T> 
{	
	pub fn new(chunk_size : usize) -> Self 
	{			
    	let mut arena = Self 
		{
			id : ARENA_ID.load(Ordering::SeqCst),
			chunk_size : chunk_size,
			heap : Vec::new(),
			last_freed : Vec::new(),
			current_index : 0,			
		};

		arena.heap.push(Vec::new());
		arena.heap[0].reserve(arena.chunk_size);
		ARENA_ID.fetch_add(1, Ordering::SeqCst);	

		arena		
	}

	pub fn id(&self) -> usize
	{
		self.id
	}

	pub fn index_status(&self, index : Index) -> IndexStatus
	{
		// It is not necessery introduce 'status : Vec<Vec<ObjectStatus>>' field 
		// to Arena for fast index status access, because next condition must be.
		if index.arena_id == self.id && index.age < self.heap.len() && index.index < self.chunk_size
		{
			if index.index >= self.current_index
			{
				IndexStatus::NonUsed
			}
			else
			if self.heap[index.age][index.index].is_some()
			{
				IndexStatus::Allocated
			}
			else
			{
				IndexStatus::Freed
			}
		}
		else
		{
			IndexStatus::Invalid
		}
	}

	pub fn status_string(status : IndexStatus) -> String
	{
		match status
		{
			IndexStatus::Invalid => "Invalid".to_string(),
			IndexStatus::Freed => "Freed".to_string(),
			IndexStatus::Allocated => "Allocated".to_string(),
			IndexStatus::NonUsed => "Nonused".to_string(),
		}
	}

	pub fn alloc(&mut self, obj : T) -> Index
	{
		//Chunk is full, need to alloc new chunk.
		if self.current_index == self.chunk_size 
		{
			self.heap.push(Vec::new());
			let age = self.heap.len() - 1;
			self.heap[age].reserve(self.chunk_size);
			self.current_index = 0;
		}				

		if self.last_freed.len() == 0
		{
			let age = self.heap.len() - 1;			
			self.heap[age].push(Some(obj));
			let index = Index::new(self.id, age, self.current_index);
			self.current_index += 1;
			index
		}
		else		
		{
			let index = self.last_freed.pop().unwrap();
			self.heap[index.age][index.index] = Some(obj);
			index			
		}

	}

	pub fn free(&mut self, index : Index) 
	{
		let status = self.index_status(index);

		if status == IndexStatus::Allocated
		{
			self.heap[index.age][index.index].take().unwrap();
			self.last_freed.push(index);
		}
		else
		{
			panic!("Unable to delete object. Index status: {} .", Self::status_string(status));
		}
	}	

	pub fn get(&self, index : Index) -> Option<&T>
	{
		self.heap[index.age][index.index].as_ref()
	}	

	pub fn get_mut(&mut self, index : Index) -> Option<&mut T>
	{
		self.heap[index.age][index.index].as_mut()
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
        let arena = Arena::<MyStruct>::new(TEST_ARENA_CHUNK_SIZE);

        assert!(arena.id < ARENA_ID.load(Ordering::SeqCst));
        assert_eq!(arena.chunk_size, TEST_ARENA_CHUNK_SIZE);
        assert_eq!(arena.heap.len(), 1);
        assert_eq!(arena.heap[0].len(), 0);
        assert_eq!(arena.last_freed.len(), 0);
        assert_eq!(arena.current_index, 0);        
    }

   #[test]
    fn arena_alloc() 
    {
        let mut arena = Arena::<MyStruct>::new(TEST_ARENA_CHUNK_SIZE);
        let index = arena.alloc(MyStruct::new(16838, "All is fine"));

        assert_eq!(arena.heap.len(), 1);
        assert_eq!(arena.heap[0].len(), 1);
        assert_eq!(arena.last_freed.len(), 0);
        assert_eq!(arena.current_index, 1);  
        assert_eq!(index.age, 0);
        assert_eq!(index.index, 0);
    }     

    #[test]
    fn arena_alloc5() 
    {
        let mut arena = Arena::<MyStruct>::new(TEST_ARENA_CHUNK_SIZE);

        let index0 = arena.alloc(MyStruct::new(0, "All is fine 0"));
        let index1 = arena.alloc(MyStruct::new(1, "All is fine 1"));
        let index2 = arena.alloc(MyStruct::new(2, "All is fine 2"));
        let index3 = arena.alloc(MyStruct::new(3, "All is fine 3"));
        let index4 = arena.alloc(MyStruct::new(4, "All is fine 4"));

        assert_eq!(arena.heap.len(), 1);
        assert_eq!(arena.heap[0].len(), 5);
        assert_eq!(arena.last_freed.len(), 0);
        assert_eq!(arena.current_index, 5);

        assert_eq!(arena.get(index0) , Some(&MyStruct::new(0, "All is fine 0")));
        assert_eq!(index0.age, 0); assert_eq!(index0.index, 0);

        assert_eq!(arena.get(index1) , Some(&MyStruct::new(1, "All is fine 1")));
        assert_eq!(index1.age, 0); assert_eq!(index1.index, 1);
        
        assert_eq!(arena.get(index2) , Some(&MyStruct::new(2, "All is fine 2")));
        assert_eq!(index2.age, 0); assert_eq!(index2.index, 2);
        
        assert_eq!(arena.get(index3) , Some(&MyStruct::new(3, "All is fine 3")));
        assert_eq!(index3.age, 0); assert_eq!(index3.index, 3);
        
        assert_eq!(arena.get(index4) , Some(&MyStruct::new(4, "All is fine 4")));
        assert_eq!(index4.age, 0); assert_eq!(index4.index, 4);
	}         

	//Alloc 'n' objects in a new Arena
	//For more test accuracy need MyStruct::new(i,"All is fine")
	fn arena_alloc_n(n : usize) -> (Arena<MyStruct>, Vec<Index>)
	{
        let mut arena = Arena::<MyStruct>::new(TEST_ARENA_CHUNK_SIZE);
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
        assert_eq!(arena.last_freed.len(), 0);
        assert_eq!(arena.current_index, 1);
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
        assert_eq!(arena.last_freed.len(), 0);
        assert_eq!(arena.current_index, 1);  

        let first0 = Index{arena_id : arena.id(), age : 0, index : 0};
        let last0 = Index{arena_id : arena.id(), age : 0, index : TEST_ARENA_CHUNK_SIZE - 1};
        let after_last0 = Index{arena_id : arena.id(), age : 0, index : TEST_ARENA_CHUNK_SIZE};

        let first1 = Index{arena_id : arena.id(), age : 1, index : 0};
        let last1 = Index{arena_id : arena.id(), age : 1, index : 0};
        let after_last1 = Index{arena_id : arena.id(), age : 1, index : 1};

        let fake_index = Index{arena_id : 33, age : 0, index : 0};

        assert_eq!(arena.index_status(first0), IndexStatus::Allocated);
        assert_eq!(arena.index_status(last0), IndexStatus::NonUsed);
        assert_eq!(arena.index_status(after_last0), IndexStatus::Invalid);

        assert_eq!(arena.index_status(first1), IndexStatus::Allocated);
        assert_eq!(arena.index_status(last1), IndexStatus::NonUsed);
        assert_eq!(arena.index_status(after_last1), IndexStatus::Invalid);        

        assert_eq!(arena.index_status(fake_index), IndexStatus::Invalid);

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
        assert_eq!(arena.last_freed.len(), 0);
        //assert_eq!(arena.current_age, 100);
        assert_eq!(arena.current_index, 1);

		let index1 = Index{arena_id : arena.id(), age : 13, index : 13};
		arena.free(index1);
		assert_eq!(arena.last_freed.len(), 1);
		assert_eq!(arena.last_freed[0], index1);		

		assert_eq!(arena.heap[13][12], Some(MyStruct::new(13*TEST_ARENA_CHUNK_SIZE+12, "All is fine")));
		assert_eq!(arena.get(index1), None);
		assert_eq!(arena.heap[13][14], Some(MyStruct::new(13*TEST_ARENA_CHUNK_SIZE+14, "All is fine")));

		let index2 = Index{arena_id : arena.id(), age : 100, index : 0};
		arena.free(index2);
		assert_eq!(arena.last_freed.len(), 2);
		assert_eq!(arena.last_freed[1], index2);		

		assert_eq!(arena.heap[99][TEST_ARENA_CHUNK_SIZE - 1], Some(MyStruct::new(99*TEST_ARENA_CHUNK_SIZE+63, "All is fine")));
		assert_eq!(arena.get(index2), None);

		//alloc after free
		let new_index1 = arena.alloc(MyStruct::new(777, "All is fine"));
		assert_eq!(index2, new_index1);
		assert_eq!(arena.get(index2), Some(&MyStruct::new(777, "All is fine")));
		assert_eq!(arena.last_freed.len(), 1);

		let new_index2 = arena.alloc(MyStruct::new(888, "All is fine"));
		assert_eq!(index1, new_index2);
		assert_eq!(arena.get(index1), Some(&MyStruct::new(888, "All is fine")));		
		assert_eq!(arena.last_freed.len(), 0);
    }         
}//mod tests
