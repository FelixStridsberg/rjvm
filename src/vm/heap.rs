use crate::vm::data_type::ReferenceType;
use crate::vm::heap::HeapObject::{IntArray, Instance};
use crate::vm::Object;

#[derive(Debug)]
pub enum HeapObject {
    IntArray(Vec<i32>),
    Instance(Object)
}

impl HeapObject {
    pub fn expect_int_array(&mut self) -> &mut Vec<i32> {
        expect_type!(self, IntArray)
    }
}

// TODO implement a real heap
#[derive(Debug)]
pub struct Heap {
    objects: Vec<HeapObject>,
}

impl Heap {
    pub fn allocate_int_array(&mut self, size: i32) -> u32 {
        let index = self.objects.len() as u32;
        self.objects.push(IntArray(vec![0; size as usize]));
        index
    }

    pub fn allocate_object(&mut self, class: &str) -> u32 {
        let index = self.objects.len() as u32;
        self.objects.push(Instance(Object {
            class: class.to_owned()
        }));
        index
    }

    pub fn get(&mut self, reference: ReferenceType) -> &mut HeapObject {
        self.objects
            .get_mut(reference as usize)
            .expect("Tried to get non existing heap object.")
    }
}

impl Default for Heap {
    fn default() -> Self {
        Heap {
            objects: Vec::new(),
        }
    }
}
