#[macro_export]
macro_rules! convert {
    ($frame:ident, $from_type:path, $to_type:path, [$($inner_type:ty),*]) => {{
        let value = expect_type!($frame.pop_operand(), $from_type);
        $frame.push_operand($to_type(value $(as $inner_type)*));
    }};
}

#[cfg(test)]
mod test {
    use crate::class::code::Opcode::*;
    use crate::vm::data_type::Value::*;

    #[test]
    fn i2l() {
        test_instruction!(
            start_stack: [Int(100)],
            instruction: I2l,
            final_stack: [Long(100)],
        );
    }

    #[test]
    fn i2f() {
        test_instruction!(
            start_stack: [Int(100)],
            instruction: I2f,
            final_stack: [Float(100.0)],
        );
    }

    #[test]
    fn i2d() {
        test_instruction!(
            start_stack: [Int(100)],
            instruction: I2d,
            final_stack: [Double(100.0)],
        );
    }

    #[test]
    fn l2f() {
        test_instruction!(
            start_stack: [Long(1337)],
            instruction: L2f,
            final_stack: [Float(1337.0)],
        );
    }

    #[test]
    fn l2d() {
        test_instruction!(
            start_stack: [Long(13372)],
            instruction: L2d,
            final_stack: [Double(13372.0)],
        );
    }

    #[test]
    fn f2d() {
        test_instruction!(
            start_stack: [Float(1000.0)],
            instruction: F2d,
            final_stack: [Double(1000.0)],
        );
    }

    #[test]
    fn i2b() {
        test_instruction!(
            start_stack: [Int(0x101)],
            instruction: I2b,
            final_stack: [Int(1)],
        );
    }

    #[test]
    fn i2c() {
        test_instruction!(
            start_stack: [Int(65)],
            instruction: I2c,
            final_stack: [Int('A' as i32)],
        );
    }

    #[test]
    fn i2s() {
        test_instruction!(
            start_stack: [Int(0xffffff)],
            instruction: I2s,
            final_stack: [Int(-1)],
        );
    }

    #[test]
    fn l2i() {
        test_instruction!(
            start_stack: [Long(62)],
            instruction: L2i,
            final_stack: [Int(62)],
        );
    }

    #[test]
    fn f2i() {
        test_instruction!(
            start_stack: [Float(1234.58)],
            instruction: F2i,
            final_stack: [Int(1234)],
        );
    }

    #[test]
    fn d2i() {
        test_instruction!(
            start_stack: [Double(1234.58)],
            instruction: D2i,
            final_stack: [Int(1234)],
        );
    }

    #[test]
    fn d2l() {
        test_instruction!(
            start_stack: [Double(1234.58)],
            instruction: D2l,
            final_stack: [Long(1234)],
        );
    }

    #[test]
    fn d2f() {
        test_instruction!(
            start_stack: [Double(1234.58)],
            instruction: D2f,
            final_stack: [Float(1234.58)],
        );
    }
}
