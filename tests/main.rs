use tstack;

use std::collections::HashMap;
use std::rc::Rc;

macro_rules! stack {
    ($($x:expr),*) => {{
        let v: Vec<u64> = vec![$($x),*];
        v
    }}
}

fn test_stack(bytecode: &[u16], expected: Vec<u64>) {
    let mut engine = tstack::Engine::new();
    let r = engine.add_module(Rc::new(tstack::module::Module {
        name: String::from("testmain"),
        strings: vec![String::from("main")],
        data: vec![],
        local_symbols: vec![tstack::module::LocalSymbol { name_id: 0, code_offset: 0 }],
        external_symbols: vec![],
        bytecode: bytecode.to_vec(),
        symbol_lookup: HashMap::new(),
    }));
    if let Err(e) = r {
        assert!(false, "Unexpected error adding test module: {}", e);
    }
    if let Err(e) = engine.run(0, 0) {
        assert!(false, "Unexpected error: {}", e);
    }
    assert_eq!(engine.stack, expected);
}

fn test_fail(
    init: Option<fn(&mut tstack::Engine)>,
    errcheck: Option<fn(tstack::errors::BytecodeError) -> bool>,
    bytecode: &[u16],
) {
    let mut engine = tstack::Engine::new();
    if let Some(initfn) = init {
        initfn(&mut engine);
    }
    let r = engine.add_module(Rc::new(tstack::module::Module {
        name: String::from("testmain"),
        strings: vec![String::from("main")],
        data: vec![],
        local_symbols: vec![tstack::module::LocalSymbol { name_id: 0, code_offset: 0 }],
        external_symbols: vec![],
        bytecode: bytecode.to_vec(),
        symbol_lookup: HashMap::new(),
    }));
    if let Err(e) = r {
        assert!(false, "Unexpected error adding test module: {}", e);
    }
    if let Err(e) = engine.run(0, 0) {
        if let Some(errfn) = errcheck {
            assert!(errfn(e), "incorrect error");
        }
        return;
    }
    assert!(false, "error expected");
}

#[test]
fn test_const_0() {
    test_stack(&[tstack::inst_stack!(CONST_0)], stack![0]);
}

#[test]
fn test_const_0_stack_overflow() {
    test_fail(
        Some(|engine| engine.maxstack = 2),
        Some(|bce| bce.is_stack_overflow()),
        &[tstack::inst_stack!(CONST_0), tstack::inst_stack!(CONST_0), tstack::inst_stack!(CONST_0)],
    );
}

#[test]
fn test_const_1() {
    test_stack(&[tstack::inst_stack!(CONST_1)], stack![1]);
}

#[test]
fn test_const_2() {
    test_stack(&[tstack::inst_stack!(CONST_2)], stack![2]);
}

#[test]
fn test_const_3() {
    test_stack(&[tstack::inst_stack!(CONST_3)], stack![3]);
}

#[test]
fn test_const_4() {
    test_stack(&[tstack::inst_stack!(CONST_4)], stack![4]);
}

#[test]
fn test_const_8() {
    test_stack(&[tstack::inst_stack!(CONST_8)], stack![8]);
}

#[test]
fn test_const_16() {
    test_stack(&[tstack::inst_stack!(CONST_16)], stack![16]);
}

#[test]
fn test_const_32() {
    test_stack(&[tstack::inst_stack!(CONST_32)], stack![32]);
}

#[test]
fn test_const_64() {
    test_stack(&[tstack::inst_stack!(CONST_64)], stack![64]);
}

#[test]
fn test_const_128() {
    test_stack(&[tstack::inst_stack!(CONST_128)], stack![128]);
}

#[test]
fn test_const_n1() {
    test_stack(&[tstack::inst_stack!(CONST_N1)], stack![0xFFFFFFFFFFFFFFFF]);
}

#[test]
fn test_const_u16() {
    test_stack(&[tstack::inst_stack!(CONST_U16), 0x1234], stack![0x1234]);
}

#[test]
fn test_const_u16_insufficient() {
    test_fail(None, Some(|e| e.is_code_data()), &[tstack::inst_stack!(CONST_U16)]);
}

#[test]
fn test_const_u32() {
    test_stack(&[tstack::inst_stack!(CONST_U32), 0x1234, 0x5678], stack![0x12345678]);
}

#[test]
fn test_const_u32_insufficient() {
    test_fail(None, Some(|e| e.is_code_data()), &[tstack::inst_stack!(CONST_U32), 0x1010]);
}

#[test]
fn test_const_u64() {
    test_stack(
        &[tstack::inst_stack!(CONST_U64), 0x1234, 0x5678, 0x9ABC, 0xDEF0],
        stack![0x123456789ABCDEF0],
    );
}

#[test]
fn test_const_u64_insufficient() {
    test_fail(
        None,
        Some(|e| e.is_code_data()),
        &[tstack::inst_stack!(CONST_U64), 0x0101, 0x0010, 0x0101],
    )
}

#[test]
fn test_const_i16() {
    test_stack(&[tstack::inst_stack!(CONST_I16), 0xFFFD], stack![0xFFFFFFFFFFFFFFFD]);
}

#[test]
fn test_const_i16_insufficient() {
    test_fail(None, Some(|e| e.is_code_data()), &[tstack::inst_stack!(CONST_I16)]);
}

#[test]
fn test_const_i32() {
    test_stack(&[tstack::inst_stack!(CONST_I32), 0x8000, 0x1234], stack!(0xFFFFFFFF80001234));
}

#[test]
fn test_const_i32_insufficient() {
    test_fail(None, Some(|e| e.is_code_data()), &[tstack::inst_stack!(CONST_I32), 0x0101])
}

#[test]
fn test_dupe() {
    test_stack(
        &[
            tstack::inst_stack!(CONST_1),
            tstack::inst_stack!(CONST_128),
            tstack::inst_stack!(CONST_N1),
            tstack::inst_stack!(CONST_I32),
            0x8018,
            0x0230,
            tstack::inst_stack!(CONST_3),
            tstack::inst_stack!(DUPE),
        ],
        stack![
            1,
            128,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFF80180230,
            128,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFF80180230
        ],
    )
}

#[test]
fn test_dupe_empty() {
    // Fail on empty stack
    test_fail(None, Some(|e| e.is_stack_underflow()), &[tstack::inst_stack!(DUPE)]);
}

#[test]
fn test_dupe_insufficient_args() {
    // Fail on insufficient arguments
    test_fail(
        None,
        Some(|e| e.is_stack_underflow()),
        &[tstack::inst_stack!(CONST_16), tstack::inst_stack!(CONST_2), tstack::inst_stack!(DUPE)],
    );
}

#[test]
fn test_dupe_1() {
    test_stack(
        &[
            tstack::inst_stack!(CONST_128),
            tstack::inst_stack!(CONST_3),
            tstack::inst_stack!(DUPE_1),
        ],
        stack![128, 3, 3],
    );
}

#[test]
fn test_dupe_1_insufficient_args() {
    test_fail(None, Some(|e| e.is_stack_underflow()), &[tstack::inst_stack!(DUPE_1)]);
}

#[test]
fn test_dupe_c() {
    test_stack(
        &[
            tstack::inst_stack!(CONST_0),
            tstack::inst_stack!(CONST_128),
            tstack::inst_stack!(CONST_8),
            tstack::inst_stack!(DUPE_C),
            2,
        ],
        stack![0, 128, 8, 128, 8],
    );
}

#[test]
fn test_dupe_c_no_count() {
    test_fail(
        None,
        Some(|e| e.is_code_data()),
        &[tstack::inst_stack!(CONST_0), tstack::inst_stack!(CONST_2), tstack::inst_stack!(DUPE_C)],
    );
}

#[test]
fn test_dupe_c_insufficient_args() {
    test_fail(
        None,
        Some(|e| e.is_stack_underflow()),
        &[tstack::inst_stack!(CONST_8), tstack::inst_stack!(DUPE_C), 4],
    );
}
