#[macro_export]
macro_rules! test_command {
        (
            $(constants: [$($constant:expr),*],)?
            $(start_stack: [$($stack:expr),*],)?
            $(start_locals: {$($local_idx:expr => $local_value:expr),*},)?
            $(start_locals_long: {$($local_l_idx:expr => $local_l_value:expr),*},)?
            command: $command:expr $(;[$($operands:expr),*])?,
            $(final_stack: [$($expect_stack:expr),*],)?
            $(final_locals: {$($expect_local_idx:expr => $expected_local:expr),*},)?
            $(final_locals_long: {$($expect_local_l_idx:expr => $expected_local_l:expr),*},)?
        ) => {{
            // Setup
            let mut _constants = ConstantPool::new(2);
            $($(_constants.add($constant);)*)?

            let mut frame = Frame::new(10, 10, &_constants);
            $(frame.set_operand_stack(vec![$($stack),*]);)?

            $($(frame.set_local($local_idx, $local_value);)*)?
            $($(frame.set_local_long($local_l_idx, $local_l_value);)*)?

            // Execute
            interpret(&mut frame, &vec![Instruction::new($command, vec![$($($operands),*)?])]);

            // Assert
            $(
                assert_eq!(
                    frame.operand_stack,
                    vec![$($expect_stack),*],
                    "Expecting frame stack to be equal to final_stack."
                );
            )?

            $(
                $(
                    assert_eq!(
                        frame.get_local($expect_local_idx),
                        $expected_local,
                        "Expecting local {} to contain {}", $expect_local_idx, $expected_local
                    )
                )*
            )?

            $(
                $(
                    assert_eq!(
                        frame.get_local_long($expect_local_l_idx),
                        $expected_local_l,
                        "Expecting long local {} to contain {}", $expect_local_l_idx, $expected_local_l
                    )
                )*
            )?
        }}
    }

// $(assert_eq!(frame.local_variables, vec![$($expect_locals),*]);)?
