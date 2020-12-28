// TODO clean up macro
#[macro_export]
macro_rules! test_instruction {
        (
            $(heap: $heap:expr,)?
            $(constants: [$($constant:expr),*],)?
            $(start_pc: $start_pc:expr,)?
            $(start_stack: [$($stack:expr),*],)?
            $(start_locals: {$($local_idx:expr => $local_value:expr),*},)?
            $(start_locals_long: {$($local_l_idx:expr => $local_l_value:expr),*},)?
            instruction: $instruction:expr $(;[$($operands:expr),*])?,
            $(final_pc: $final_pc:expr,)?
            $(final_stack: [$($expect_stack:expr),*],)?
            $(final_locals: {$($expect_local_idx:expr => $expected_local:expr),*},)?
            $(final_locals_long: {$($expect_local_l_idx:expr => $expected_local_l:expr),*},)?
        ) => {{
            let mut _constants = crate::class::constant::ConstantPool::new(2);
            $($(_constants.add($constant);)*)?

            let _class = crate::class::Class::from_constant_pool(_constants);
            let _code = crate::class::attribute::Code::new(10, 10, vec![], vec![], vec![
                crate::class::code::Instruction::new($instruction, vec![$($($operands),*)?])
            ]);
            let _method = crate::class::MethodInfo::from_code(_code);
            let mut frame = crate::vm::Frame::new(std::rc::Rc::new(_class), std::rc::Rc::new(_method));

            $(frame.pc = $start_pc;)?
            $(frame.set_operand_stack(vec![$($stack),*]);)?

            $($(frame.set_local($local_idx, $local_value);)*)?
            $($(frame.set_local_long($local_l_idx, $local_l_value);)*)?

            // Execute
            let mut _heap = crate::vm::heap::Heap::default();
            let _heap_ref = &mut _heap;
            $(let _heap_ref = &mut $heap;)?

            crate::vm::interpreter::interpret_instruction(
                &mut frame,
                _heap_ref,
                &crate::class::code::Instruction::new($instruction, vec![$($($operands),*)?])
            ).expect("Interpretation failed.");

            // Assert

            $(assert_eq!(frame.pc, $final_pc, "Expecting frame pc to be equal to final_pc.");)?

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
