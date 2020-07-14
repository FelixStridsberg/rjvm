use crate::vm::data_type::Value::{Int, Reference};
use crate::vm::data_type::{IntType, ReferenceType};
use crate::vm::frame::Frame;
use crate::vm::heap::Heap;

pub fn new_array(frame: &mut Frame, heap: &mut Heap, operands: &[u8]) {
    let len = frame.pop_operand().expect_int();
    let reference = match operands[0] {
        10 => heap.allocate_int_array(len),
        a => panic!("Unknown array type {}.", a),
    };

    frame.push_operand(Reference(reference as ReferenceType));
}

pub fn int_array_store(frame: &mut Frame, heap: &mut Heap) {
    let value: IntType = frame.pop_operand().expect_int();
    let index: IntType = frame.pop_operand().expect_int();
    let reference = frame.pop_operand().expect_reference();

    let array = heap.get(reference).expect_int_array();
    array[index as usize] = value;
}

pub fn int_array_load(frame: &mut Frame, heap: &mut Heap) {
    let index: IntType = frame.pop_operand().expect_int();
    let reference = frame.pop_operand().expect_reference();

    let array = heap.get(reference).expect_int_array();
    frame.push_operand(Int(array[index as usize]));
}

#[cfg(test)]
mod test {
    use crate::class::code::Opcode::{Iaload, Iastore, NewArray};
    use crate::vm::data_type::Value::{Int, Reference};
    use crate::vm::heap::Heap;

    #[test]
    fn newarray() {
        let mut heap = Heap::default();
        test_command!(
            heap: heap,
            start_stack: [Int(10)],
            command: NewArray; [0x0a],
            final_stack: [Reference(0)],
        );

        let array = heap.get(0).expect_int_array();
        assert_eq!(array.len(), 10);
    }

    #[test]
    fn iastore() {
        let mut heap = Heap::default();
        heap.allocate_int_array(10);

        test_command!(
            heap: heap,
            start_stack: [Reference(0), Int(1), Int(2)],
            command: Iastore; [],
            final_stack: [],
        );

        let array = heap.get(0).expect_int_array();
        assert_eq!(array[1], 2);
    }

    #[test]
    fn iaload() {
        let mut heap = Heap::default();
        heap.allocate_int_array(10);

        {
            let array = heap.get(0).expect_int_array();
            array[4] = 10;
        }

        test_command!(
            heap: heap,
            start_stack: [Reference(0), Int(4)],
            command: Iaload; [],
            final_stack: [Int(10)],
        );
    }
}
