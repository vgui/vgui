# arena.rs

**static ARENA_ID : Mutex<usize>**
Global identificator generator for all 'struct Arena\<T\>' instances.
TODO: Need to wrap to Arc for thread synchronization. Rename to ARENA_ID.

**pub struct Arena\<T\>**
'id' is just 'ARENA_ID + = 1' in a 'Arena<T>::new' method.
The "heap" field contains the vaalue 'None' placed in the 'freed' field,
the value 'Some' is the valid instance of 'Some(T)'.
The "freed" field contains the freed heap indexes.