#[macro_export]
macro_rules! test_command {
        (
            $(start_stack: [$($stack:expr),*],)?
            $(start_locals: [$($locals:expr),*],)?
            command: $command:expr $(;[$($operands:expr),*])?,
            final_stack: [$($expect:expr),*],
        ) => {{
            let constants = ConstantPool::new(2);
            let mut frame = Frame::new(10, 10, &constants);
            $(frame.set_operand_stack(vec![$($stack),*]);)?
            $(frame.set_locals(vec![$($locals),*]);)?
            interpret(&mut frame, &vec![Instruction::new($command, vec![$($($operands),*)?])]);
            assert_eq!(frame.operand_stack, vec![$($expect),*]);
        }}
    }
