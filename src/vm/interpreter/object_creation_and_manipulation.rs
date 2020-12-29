use crate::error::Result;
use crate::vm::data_type::Value::{Int, Reference};
use crate::vm::data_type::{IntType, ReferenceType};
use crate::vm::frame::Frame;
use crate::vm::heap::Heap;

pub fn new_array(frame: &mut Frame, heap: &mut Heap, operands: &[u8]) -> Result<()> {
    let len = frame.pop_operand().expect_int();
    let reference = match operands[0] {
        5 => heap.allocate_char_array(len),
        8 => heap.allocate_byte_array(len),
        10 => heap.allocate_int_array(len),
        a => runtime_error!("Unknown array type {}.", a),
    };

    frame.push_operand(Reference(reference as ReferenceType));
    Ok(())
}

pub fn array_length(frame: &mut Frame, heap: &Heap) -> Result<()> {
    let reference = frame.pop_operand().expect_reference();
    let array = heap.get(reference).expect_int_array();
    frame.push_operand(Int(array.len() as i32));
    Ok(())
}

pub fn new_object(frame: &mut Frame, heap: &mut Heap, operands: &[u8]) {
    let index = ((operands[0] as u16) << 8) | operands[1] as u16;
    let class = frame
        .class
        .constants
        .get_class_info_name(index as u16)
        .unwrap();
    let reference = heap.allocate_object(class);
    frame.push_operand(Reference(reference as ReferenceType));
}

pub fn byte_array_store(frame: &mut Frame, heap: &mut Heap) {
    let value: IntType = frame.pop_operand().expect_int();
    let index: IntType = frame.pop_operand().expect_int();
    let reference = frame.pop_operand().expect_reference();

    let array = heap.get_mut(reference).expect_mut_byte_array();
    array[index as usize] = value as u8;
}

pub fn char_array_store(frame: &mut Frame, heap: &mut Heap) {
    let value: IntType = frame.pop_operand().expect_int();
    let index: IntType = frame.pop_operand().expect_int();
    let reference = frame.pop_operand().expect_reference();

    let array = heap.get_mut(reference).expect_mut_char_array();
    array[index as usize] = value as u8 as char;
}

pub fn int_array_store(frame: &mut Frame, heap: &mut Heap) {
    let value: IntType = frame.pop_operand().expect_int();
    let index: IntType = frame.pop_operand().expect_int();
    let reference = frame.pop_operand().expect_reference();

    let array = heap.get_mut(reference).expect_mut_int_array();
    array[index as usize] = value;
}

pub fn byte_array_load(frame: &mut Frame, heap: &mut Heap) {
    let index: IntType = frame.pop_operand().expect_int();
    let reference = frame.pop_operand().expect_reference();

    let array = heap.get_mut(reference).expect_byte_array();
    frame.push_operand(Int(array[index as usize] as IntType));
}

pub fn char_array_load(frame: &mut Frame, heap: &mut Heap) {
    let index: IntType = frame.pop_operand().expect_int();
    let reference = frame.pop_operand().expect_reference();

    let array = heap.get_mut(reference).expect_char_array();
    frame.push_operand(Int(array[index as usize] as IntType));
}

pub fn int_array_load(frame: &mut Frame, heap: &mut Heap) {
    let index: IntType = frame.pop_operand().expect_int();
    let reference = frame.pop_operand().expect_reference();

    let array = heap.get_mut(reference).expect_mut_int_array();
    frame.push_operand(Int(array[index as usize]));
}

#[cfg(test)]
mod test {
    use crate::class::code::Opcode::{IaLoad, IaStore, NewArray};
    use crate::vm::data_type::Value::{Int, Reference};
    use crate::vm::heap::Heap;

    #[test]
    fn newarray_byte() {
        let mut heap = Heap::default();
        test_instruction!(
            heap: heap,
            start_stack: [Int(10)],
            instruction: NewArray; [0x08],
            final_stack: [Reference(0)],
        );

        let array = heap.get_mut(0).expect_byte_array();
        assert_eq!(array.len(), 10);
    }

    #[test]
    fn newarray_int() {
        let mut heap = Heap::default();
        test_instruction!(
            heap: heap,
            start_stack: [Int(10)],
            instruction: NewArray; [0x0a],
            final_stack: [Reference(0)],
        );

        let array = heap.get_mut(0).expect_int_array();
        assert_eq!(array.len(), 10);
    }

    #[test]
    fn iastore() {
        let mut heap = Heap::default();
        heap.allocate_int_array(10);

        test_instruction!(
            heap: heap,
            start_stack: [Reference(0), Int(1), Int(2)],
            instruction: IaStore; [],
            final_stack: [],
        );

        let array = heap.get_mut(0).expect_mut_int_array();
        assert_eq!(array[1], 2);
    }

    #[test]
    fn iaload() {
        let mut heap = Heap::default();
        heap.allocate_int_array(10);

        {
            let array = heap.get_mut(0).expect_mut_int_array();
            array[4] = 10;
        }

        test_instruction!(
            heap: heap,
            start_stack: [Reference(0), Int(4)],
            instruction: IaLoad; [],
            final_stack: [Int(10)],
        );
    }
}
