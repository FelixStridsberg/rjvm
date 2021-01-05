use crate::error::Result;
use crate::vm::data_type::ReferenceType;
use crate::vm::data_type::Value::{Int, Reference};
use crate::vm::frame::Frame;
use crate::vm::heap::Heap;

#[macro_export]
macro_rules! array_load (
    ($frame:ident, $heap:ident, $array_type:path, $value_type:path, $inner_type:ty) => {{
        let index = $frame.pop_operand().expect_int();
        let reference = $frame.pop_operand().expect_reference().expect("Null pointer error"); // TODO
        let array = expect_type!($heap.get(reference), $array_type);

        if let Some(value) = array.get(index as usize) {
            $frame.push_operand($value_type(*value as $inner_type));
            Ok(Normal)
        } else {
            Ok(InternalException(
                "java/lang/ArrayIndexOutOfBoundsException".to_owned(),
            ))
        }
    }}
);

#[macro_export]
macro_rules! array_store (
    ($frame:ident, $heap:ident, $array_type:path, $value_type:path, [$($inner_type:ty),*]) => {{
        let value = expect_type!($frame.pop_operand(), $value_type);
        let index = $frame.pop_operand().expect_int();
        let reference = $frame.pop_operand().expect_reference().expect("Null pointer error"); // TODO

        let array = expect_type!($heap.get_mut(reference), $array_type);
        if let Some(elem) = array.get_mut(index as usize) {
            *elem = value $(as $inner_type)*;
            Ok(Normal)
        } else {
            Ok(InternalException(
                "java/lang/ArrayIndexOutOfBoundsException".to_owned(),
            ))
        }
    }}
);

pub fn new_array(frame: &mut Frame, heap: &mut Heap, operands: &[u8]) -> Result<()> {
    let len = frame.pop_operand().expect_int();
    let reference = match operands[0] {
        5 => heap.allocate_char_array(len),
        6 => heap.allocate_float_array(len),
        7 => heap.allocate_double_array(len),
        8 => heap.allocate_byte_array(len),
        9 => heap.allocate_short_array(len),
        10 => heap.allocate_int_array(len),
        11 => heap.allocate_long_array(len),
        a => return runtime_error!("Unknown array type {}.", a),
    };

    frame.push_operand(Reference(Some(reference as ReferenceType)));
    Ok(())
}

pub fn array_length(frame: &mut Frame, heap: &Heap) -> Result<()> {
    let reference = frame
        .pop_operand()
        .expect_reference()
        .expect("Null pointer error"); // TODO
    let array_len = heap.get(reference).array_length();
    frame.push_operand(Int(array_len as i32));
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
    frame.push_operand(Reference(Some(reference as ReferenceType)));
}

pub fn reference_array_store(frame: &mut Frame, heap: &mut Heap) {
    let value = frame
        .pop_operand()
        .expect_reference()
        .expect("Null pointer error"); // TODO
    let index = frame.pop_operand().expect_int();
    let reference = frame
        .pop_operand()
        .expect_reference()
        .expect("Null pointer error"); // TODO

    let object_type = heap.get(value).expect_instance().class.to_owned();
    let (array_type, array) = heap.get_mut(reference).expect_mut_reference_array();

    // TODO better type check, probably need to move to VM? Or can we get immutable access to all we need from here?
    if &object_type != array_type {
        unimplemented!("Better type checking for reference arrays.");
    }

    array[index as usize] = Some(value);
}

pub fn reference_array_load(frame: &mut Frame, heap: &mut Heap) {
    let index = frame.pop_operand().expect_int();
    let reference = frame
        .pop_operand()
        .expect_reference()
        .expect("Null pointer error"); // TODO

    let (_, array) = heap.get_mut(reference).expect_mut_reference_array();
    frame.push_operand(Reference(array[index as usize]));
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
            final_stack: [Reference(Some(0))],
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
            final_stack: [Reference(Some(0))],
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
            start_stack: [Reference(Some(0)), Int(1), Int(2)],
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
            start_stack: [Reference(Some(0)), Int(4)],
            instruction: IaLoad; [],
            final_stack: [Int(10)],
        );
    }
}
