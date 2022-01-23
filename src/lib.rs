#[allow(dead_code)]
pub mod bytecode;
pub mod errors;

use errors::{BytecodeError, StackError, ValueError};
use std::num::Wrapping;

/// The virtual machine engine
pub struct Engine {
    // The operand stack, used to hold dynamic arguments to instructions
    pub stack: Vec<u64>,
}

macro_rules! popsingle {
    ($engine:expr, $opcode:expr, $name:literal, $count:literal) => {{
        let value: u64;
        let res = $engine.stack.pop();
        match res {
            None => {
                return Err(BytecodeError::StackError(StackError::new(
                    $opcode, $name, $count,
                )))
            }
            Some(v) => value = v,
        }
        value
    }};
}

macro_rules! popstack1 {
    ($engine:expr, $opcode:expr, $name:literal) => {{
        popsingle!($engine, $opcode, $name, 1)
    }};
}

macro_rules! popstack2 {
    ($engine:expr, $opcode:expr, $name:literal) => {{
        let v1 = popsingle!($engine, $opcode, $name, 2);
        let v2 = popsingle!($engine, $opcode, $name, 2);
        (v1, v2)
    }};
}

macro_rules! cval_u16 {
    ($code:expr, $n:expr, $name:literal) => {{
        if $code.len() - $n < 1 {
            return Err(BytecodeError::ValueError(ValueError::new($code[$n], $name, 2)));
        }
        $code[$n+1]
    }}
}

macro_rules! cval_u16_2 {
    ($code:expr, $n:expr, $name:literal) => {{
        if $code.len() - $n < 2 {
            return Err(BytecodeError::ValueError(ValueError::new($code[$n], $name, 4)));
        }
        ($code[$n+1], $code[$n+2])
    }}
}

macro_rules! cval_u16_4 {
    ($code:expr, $n:expr, $name:literal) => {{
        if $code.len() - $n < 4 {
            return Err(BytecodeError::ValueError(ValueError::new($code[$n], $name, 8)));
        }
        ($code[$n+1], $code[$n+2], $code[$n+3], $code[$n+4])
    }}
}

macro_rules! cval_u32 {
    ($code:expr, $n:expr, $name:literal) => {{
        let (v1, v2) = cval_u16_2!($code, $n, $name);
        ((v1 as u32) << 16) | (v2 as u32)
    }}
}

macro_rules! cval_u64 {
    ($code:expr, $n:expr, $name:literal) => {{
        let (v1, v2, v3, v4) = cval_u16_4!($code, $n, $name);
        ((v1 as u64) << 48) | ((v2 as u64) << 32) | ((v3 as u64) << 16) | (v4 as u64)
    }}
}

impl Engine {
    /// Create a new Engine instance
    pub fn new() -> Engine {
        Engine {
            stack: Vec::new(),
        }
    }

    /// Run some arbitrary bytecode
    pub fn run(&mut self, code: &[u16]) -> Result<(), BytecodeError> {
        let codelen = code.len();
        let mut n: usize = 0;

        while n < codelen {
            let opcode = code[n];
            let group = ((opcode & bytecode::GROUP_MASK) >> bytecode::GROUP_SHIFT) as u8;
            let value = ((opcode & bytecode::DATA_MASK) >> bytecode::DATA_SHIFT) as u8;
            println!("OpCode: {:#06x} ({:#04x}|{:#04x})", opcode, group, value);
            match group {
                bytecode::groups::SYSTEM => n = self.op_system(n, code, value)?,
                bytecode::groups::STACK  => n = self.op_stack(n, code, value)?,
                bytecode::groups::MATH   => n = self.op_math(n, code, value)?,
                _ => {
                    // Unimplemented
                    return Err(BytecodeError::BadOpCodeError(opcode));
                }
            }
        }

        Ok(())
    }

    fn op_system(&mut self, n: usize, code: &[u16], value: u8) -> Result<usize, BytecodeError> {
        match value {
            bytecode::sys::NOP => return Ok(n+1),
            bytecode::sys::PRINT_STACK => {
                println!("Stack: {:?}", self.stack);
            },
            bytecode::sys::PRINT_U64 => {
                let value = popstack1!(self, code[n], "print.u64");
                println!("PRINT: {}", value);
            },
            bytecode::sys::PRINT_I64 => {
                let value = popstack1!(self, code[n], "print.i64");
                println!("PRINT: {}", value as i64);
            },
            bytecode::sys::PRINT_F32 => {
                let value = popstack1!(self, code[n], "print.f32");
                println!("PRINT: {}", f32::from_bits(value as u32));
            },
            bytecode::sys::PRINT_F64 => {
                let value = popstack1!(self, code[n], "print.f64");
                println!("PRINT: {}", f64::from_bits(value));
            },
            _ => {
                // Unimplemented
                return Err(BytecodeError::BadOpCodeError(code[n]));
            }
        }
        Ok(n+1)
    }

    fn op_stack(&mut self, n: usize, code: &[u16], value: u8) -> Result<usize, BytecodeError> {
        match value {
            bytecode::stack::CONST_0 => self.stack.push(0),
            bytecode::stack::CONST_1 => self.stack.push(1),
            bytecode::stack::CONST_2 => self.stack.push(2),
            bytecode::stack::CONST_3 => self.stack.push(3),
            bytecode::stack::CONST_4 => self.stack.push(4),
            bytecode::stack::CONST_8 => self.stack.push(8),
            bytecode::stack::CONST_16 => self.stack.push(16),
            bytecode::stack::CONST_32 => self.stack.push(32),
            bytecode::stack::CONST_64 => self.stack.push(64),
            bytecode::stack::CONST_128 => self.stack.push(128),
            bytecode::stack::CONST_N1 => self.stack.push((-1 as i64) as u64),
            bytecode::stack::CONST_U16 => {
                self.stack.push(cval_u16!(code, n, "const.u16") as u64);
                return Ok(n+2);
            },
            bytecode::stack::CONST_U32 => {
                self.stack.push(cval_u32!(code, n, "const.u32") as u64);
                return Ok(n+3);
            },
            bytecode::stack::CONST_U64 => {
                self.stack.push(cval_u64!(code, n, "const.u64"));
                return Ok(n + 5);
            },
            bytecode::stack::CONST_I16 => {
                if code.len() - n < 1 {
                    return Err(BytecodeError::ValueError(ValueError::new(
                        code[n], "const.i16", 2,
                    )));
                }
                self.stack.push(((code[n+1] as i16) as i64) as u64);
                return Ok(n+2);
            },
            bytecode::stack::CONST_I32 => {
                if code.len() - n < 2 {
                    return Err(BytecodeError::ValueError(ValueError::new(
                        code[n], "const.i32", 4,
                    )))
                }
                let value = (((((code[n + 1] as u32) << 16) | (code[n + 2] as u32)) as i32) as i64) as u64;
                self.stack.push(value);
                return Ok(n+3);
            },
            _ => {
                return Err(BytecodeError::BadOpCodeError(code[n]));
            }
        }
        Ok(n + 1)
    }

    fn op_math(&mut self, n: usize, code: &[u16], value: u8) -> Result<usize, BytecodeError> {
        match value {
            bytecode::math::ADD => {
                let (v1, v2) = popstack2!(self, code[n], "add");
                self.stack.push((Wrapping(v1) + Wrapping(v2)).0);
            },
            bytecode::math::ADD_C => {
                let c = cval_u16!(code, n, "add.c") as u64;
                let v = popstack1!(self, code[n], "add.c");
                self.stack.push((Wrapping(c) + Wrapping(v)).0);
                return Ok(n+2);
            },
            bytecode::math::SUB => {
                let (v1, v2) = popstack2!(self, code[n], "sub");
                self.stack.push((Wrapping(v1) - Wrapping(v2)).0);
            },
            bytecode::math::SUB_C => {
                let c = cval_u16!(code, n, "sub.c") as u64;
                let v = popstack1!(self, code[n], "sub.c");
                self.stack.push((Wrapping(v) - Wrapping(c)).0);
                return Ok(n+2);
            },
            bytecode::math::MUL => {
                let (v1, v2) = popstack2!(self, code[n], "mul");
                self.stack.push((Wrapping(v1) * Wrapping(v2)).0);
            },
            bytecode::math::MUL_C => {
                let c = cval_u16!(code, n, "mul.c") as u64;
                let v = popstack1!(self, code[n], "mul.c");
                self.stack.push((Wrapping(v) * Wrapping(c)).0);
                return Ok(n+2);
            },
            bytecode::math::DIV => {
                let (v1, v2) = popstack2!(self, code[n], "div");
                self.stack.push((Wrapping(v1) / Wrapping(v2)).0);
            },
            bytecode::math::DIV_C => {
                let c = cval_u16!(code, n, "div.c") as u64;
                let v = popstack1!(self, code[n], "div.c");
                self.stack.push((Wrapping(v) / Wrapping(c)).0);
                return Ok(n+2);
            }
            _ => {
                return Err(BytecodeError::BadOpCodeError(code[n]));
            }
        }
        Ok(n + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! stack {
        ($($x:expr),*) => {{
            let v: Vec<u64> = vec![$($x),*];
            v
        }}
    }

    fn test_stack(bytecode: &[u16], expected: Vec<u64>) {
        let mut engine = Engine::new();
        let result = engine.run(bytecode);
        if let Err(e) = result {
            assert!(true, "Unexpected error: {}", e);
        }
        assert_eq!(engine.stack, expected);
    }

    #[test]
    fn test_const_0() {
        test_stack(&[inst_stack!(CONST_0)], stack![0]);
    }
    
    #[test]
    fn test_const_1() {
        test_stack(&[inst_stack!(CONST_1)], stack![1]);
    }

    #[test]
    fn test_const_2() {
        test_stack(&[inst_stack!(CONST_2)], stack![2]);
    }

    #[test]
    fn test_const_3() {
        test_stack(&[inst_stack!(CONST_3)], stack![3]);
    }

    #[test]
    fn test_const_4() {
        test_stack(&[inst_stack!(CONST_4)], stack![4]);
    }

    #[test]
    fn test_const_8() {
        test_stack(&[inst_stack!(CONST_8)], stack![8]);
    }

    #[test]
    fn test_const_16() {
        test_stack(&[inst_stack!(CONST_16)], stack![16]);
    }

    #[test]
    fn test_const_32() {
        test_stack(&[inst_stack!(CONST_32)], stack![32]);
    }

    #[test]
    fn test_const_64() {
        test_stack(&[inst_stack!(CONST_64)], stack![64]);
    }

    #[test]
    fn test_const_128() {
        test_stack(&[inst_stack!(CONST_128)], stack![128]);
    }

    #[test]
    fn test_const_n1() {
        test_stack(&[inst_stack!(CONST_N1)], stack![0xFFFFFFFFFFFFFFFF]);
    }

    #[test]
    fn test_const_u16() {
        test_stack(&[inst_stack!(CONST_U16), 0x1234], stack![0x1234]);
    }

    #[test]
    fn test_const_u32() {
        test_stack(&[inst_stack!(CONST_U32), 0x1234, 0x5678], stack![0x12345678]);
    }

    #[test]
    fn test_const_u64() {
        test_stack(&[inst_stack!(CONST_U64), 0x1234, 0x5678, 0x9ABC, 0xDEF0], stack![0x123456789ABCDEF0]);
    }

    #[test]
    fn test_const_i16() {
        test_stack(&[inst_stack!(CONST_I16), 0xFFFD], stack![0xFFFFFFFFFFFFFFFD]);
    }

    #[test]
    fn test_const_i32() {
        test_stack(&[inst_stack!(CONST_I32), 0x8000, 0x1234], stack!(0xFFFFFFFF80001234));
    }
}