#[macro_export]
macro_rules! test_command {
        (
            start_stack [$($stack:expr),*],
            command $command:expr,
            final_stack [$($expect:expr),*]
        ) => {{
            let constants = ConstantPool::new(2);
            let mut frame = Frame::new(10, 10, &constants);
            frame.set_operand_stack(vec![$($stack),*]);
            interpret(&mut frame, &vec![Instruction::new($command, vec![])]);
            assert_eq!(frame.operand_stack, vec![$($expect),*]);
        }}
    }
