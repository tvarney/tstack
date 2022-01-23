
use tstack;

macro_rules! stack {
    ($($x:expr),*) => {{
        let v: Vec<u64> = vec![$($x),*];
        v
    }}
}

fn test_stack(bytecode: &[u16], expected: Vec<u64>) {
    let mut engine = tstack::Engine::new();
    let result = engine.run(bytecode);
    if let Err(e) = result {
        assert!(true, "Unexpected error: {}", e);
    }
    assert_eq!(engine.stack, expected);
}

#[test]
fn test_const_0() {
    test_stack(&[tstack::inst_stack!(CONST_0)], stack![0]);
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
fn test_const_u32() {
    test_stack(&[tstack::inst_stack!(CONST_U32), 0x1234, 0x5678], stack![0x12345678]);
}

#[test]
fn test_const_u64() {
    test_stack(&[tstack::inst_stack!(CONST_U64), 0x1234, 0x5678, 0x9ABC, 0xDEF0], stack![0x123456789ABCDEF0]);
}

#[test]
fn test_const_i16() {
    test_stack(&[tstack::inst_stack!(CONST_I16), 0xFFFD], stack![0xFFFFFFFFFFFFFFFD]);
}

#[test]
fn test_const_i32() {
    test_stack(&[tstack::inst_stack!(CONST_I32), 0x8000, 0x1234], stack!(0xFFFFFFFF80001234));
}