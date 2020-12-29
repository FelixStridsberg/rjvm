use crate::vm::data_type::ReferenceType;
use crate::vm::heap::HeapObject::{ByteArray, CharArray, FloatArray, Instance, IntArray};
use crate::vm::Object;
use std::collections::HashMap;

#[derive(Debug)]
pub enum HeapObject {
    ByteArray(Vec<u8>),
    CharArray(Vec<char>),
    IntArray(Vec<i32>),
    FloatArray(Vec<f32>),
    Instance(Object),
}

impl HeapObject {
    pub fn expect_byte_array(&self) -> &Vec<u8> {
        expect_type!(self, ByteArray)
    }

    pub fn expect_mut_byte_array(&mut self) -> &mut Vec<u8> {
        expect_type!(self, ByteArray)
    }

    pub fn expect_char_array(&self) -> &Vec<char> {
        expect_type!(self, CharArray)
    }

    pub fn expect_mut_char_array(&mut self) -> &mut Vec<char> {
        expect_type!(self, CharArray)
    }

    pub fn expect_float_array(&self) -> &Vec<f32> {
        expect_type!(self, FloatArray)
    }

    pub fn expect_mut_float_array(&mut self) -> &mut Vec<f32> {
        expect_type!(self, FloatArray)
    }

    pub fn expect_int_array(&self) -> &Vec<i32> {
        expect_type!(self, IntArray)
    }

    pub fn expect_mut_int_array(&mut self) -> &mut Vec<i32> {
        expect_type!(self, IntArray)
    }

    pub fn expect_instance(&self) -> &Object {
        expect_type!(self, Instance)
    }
}

// TODO implement a real heap
#[derive(Debug)]
pub struct Heap {
    objects: Vec<HeapObject>,
}

// TODO DRY up
impl Heap {
    pub fn allocate_byte_array(&mut self, size: i32) -> u32 {
        let index = self.objects.len() as u32;
        self.objects.push(ByteArray(vec![0; size as usize]));
        index
    }

    pub fn allocate_char_array(&mut self, size: i32) -> u32 {
        let index = self.objects.len() as u32;
        self.objects.push(CharArray(vec!['\0'; size as usize]));
        index
    }

    pub fn allocate_float_array(&mut self, size: i32) -> u32 {
        let index = self.objects.len() as u32;
        self.objects.push(FloatArray(vec![0.0; size as usize]));
        index
    }

    pub fn allocate_int_array(&mut self, size: i32) -> u32 {
        let index = self.objects.len() as u32;
        self.objects.push(IntArray(vec![0; size as usize]));
        index
    }

    pub fn allocate_object(&mut self, class: &str) -> u32 {
        let index = self.objects.len() as u32;
        self.objects.push(Instance(Object {
            class: class.to_owned(),
            fields: HashMap::new(),
        }));
        index
    }

    pub fn get(&self, reference: ReferenceType) -> &HeapObject {
        self.objects
            .get(reference as usize)
            .expect("Tried to get non existing heap object.")
    }

    pub fn get_mut(&mut self, reference: ReferenceType) -> &mut HeapObject {
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
