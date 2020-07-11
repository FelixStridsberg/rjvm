use crate::vm::heap::HeapType::IntArray;

#[derive(Debug)]
pub enum HeapType {
    IntArray(Vec<i32>),
}

// TODO implement a real heap
#[derive(Debug)]
pub struct Heap {
    objects: Vec<HeapType>,
}

impl Heap {
    pub fn allocate_int_array(&mut self, size: i32) -> i32 {
        let index = self.objects.len() as i32;
        self.objects.push(IntArray(vec![0; size as usize]));
        index
    }

    pub fn get_int_array(&mut self, index: i32) -> &mut Vec<i32> {
        let object = self.objects.get_mut(index as usize).unwrap();
        match object {
            IntArray(a) => a,
            _ => panic!("Tried to pop {:?} as IntArray.", object),
        }
    }
}

impl Default for Heap {
    fn default() -> Self {
        Heap {
            objects: Vec::new(),
        }
    }
}
