use crate::class::Class;
use crate::vm::data_type::ReferenceType;
use crate::vm::heap::HeapObject::{
    ByteArray, CharArray, DoubleArray, FloatArray, Instance, IntArray, LongArray, Null,
    ReferenceArray, ShortArray,
};
use crate::vm::Object;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum HeapObject {
    ByteArray(Vec<u8>),
    CharArray(Vec<char>),
    ShortArray(Vec<i16>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
    FloatArray(Vec<f32>),
    DoubleArray(Vec<f64>),
    ReferenceArray((String, Vec<Option<ReferenceType>>)),
    Instance(Object),
    Null,
}

impl HeapObject {
    pub fn expect_reference_array(&self) -> &(String, Vec<Option<ReferenceType>>) {
        expect_type!(self, ReferenceArray)
    }

    pub fn expect_mut_reference_array(&mut self) -> &mut (String, Vec<Option<ReferenceType>>) {
        expect_type!(self, ReferenceArray)
    }

    pub fn array_length(&self) -> usize {
        match self {
            ByteArray(a) => a.len(),
            CharArray(a) => a.len(),
            ShortArray(a) => a.len(),
            IntArray(a) => a.len(),
            LongArray(a) => a.len(),
            FloatArray(a) => a.len(),
            DoubleArray(a) => a.len(),
            ReferenceArray((_, a)) => a.len(),
            o => panic!("Tried to get array length of {:?}", o),
        }
    }

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

    pub fn expect_double_array(&self) -> &Vec<f64> {
        expect_type!(self, DoubleArray)
    }

    pub fn expect_mut_double_array(&mut self) -> &mut Vec<f64> {
        expect_type!(self, DoubleArray)
    }

    pub fn expect_int_array(&self) -> &Vec<i32> {
        expect_type!(self, IntArray)
    }

    pub fn expect_mut_int_array(&mut self) -> &mut Vec<i32> {
        expect_type!(self, IntArray)
    }

    pub fn expect_long_array(&self) -> &Vec<i64> {
        expect_type!(self, LongArray)
    }

    pub fn expect_mut_long_array(&mut self) -> &mut Vec<i64> {
        expect_type!(self, LongArray)
    }

    pub fn expect_short_array(&self) -> &Vec<i16> {
        expect_type!(self, ShortArray)
    }

    pub fn expect_mut_short_array(&mut self) -> &mut Vec<i16> {
        expect_type!(self, ShortArray)
    }

    pub fn expect_instance(&self) -> &Object {
        expect_type!(self, Instance)
    }
}

// TODO implement a real heap
#[derive(Debug)]
pub struct Heap {
    objects: Vec<HeapObject>,
    null: HeapObject,
}

// TODO DRY up and heapify
impl Heap {
    pub fn allocate_reference_array(&mut self, size: i32, class: Rc<Class>) -> u32 {
        let index = self.objects.len() as u32;
        self.objects.push(ReferenceArray((
            class.this_class.to_string(),
            vec![None; size as usize],
        )));
        index
    }

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

    pub fn allocate_double_array(&mut self, size: i32) -> u32 {
        let index = self.objects.len() as u32;
        self.objects.push(DoubleArray(vec![0.0; size as usize]));
        index
    }

    pub fn allocate_int_array(&mut self, size: i32) -> u32 {
        let index = self.objects.len() as u32;
        self.objects.push(IntArray(vec![0; size as usize]));
        index
    }

    pub fn allocate_long_array(&mut self, size: i32) -> u32 {
        let index = self.objects.len() as u32;
        self.objects.push(LongArray(vec![0; size as usize]));
        index
    }

    pub fn allocate_short_array(&mut self, size: i32) -> u32 {
        let index = self.objects.len() as u32;
        self.objects.push(ShortArray(vec![0; size as usize]));
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
        self.objects.get(reference as usize).unwrap_or(&self.null)
        //.expect("Tried to get non existing heap object.")
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
            null: Null,
        }
    }
}
