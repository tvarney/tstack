
#[macro_use]
mod macros;

pub mod bytecode;
pub mod errors;

use errors::BytecodeError;
use std::num::Wrapping;

/// The virtual machine engine
pub struct Engine {
    /// The operand stack, used to hold dynamic arguments to instructions
    pub stack: Vec<u64>,

    /// The maximum depth of the stack
    pub maxstack: usize,
}

impl Engine {
    /// Create a new Engine instance
    pub fn new() -> Engine {
        Engine {
            stack: Vec::new(),
            maxstack: 0x8FFF,
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
            println!("Opcode: {:#06x} ({:#04x}|{:#04x})", opcode, group, value);
            match group {
                bytecode::groups::SYSTEM => n = self.op_system(n, code, value)?,
                bytecode::groups::STACK  => n = self.op_stack(n, code, value)?,
                bytecode::groups::MATH   => n = self.op_math(n, code, value)?,
                _ => {
                    // Unimplemented
                    return Err(BytecodeError::BadOpcode(opcode));
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
                return Err(BytecodeError::BadOpcode(code[n]));
            }
        }
        Ok(n+1)
    }

    fn op_stack(&mut self, n: usize, code: &[u16], value: u8) -> Result<usize, BytecodeError> {
        match value {
            //bytecode::stack::CONST_0 => self.stack.push(0),
            bytecode::stack::CONST_0 => pushstack!(self, code[n], "const.0", 0),
            bytecode::stack::CONST_1 => pushstack!(self, code[n], "const.1", 1),
            bytecode::stack::CONST_2 => pushstack!(self, code[n], "const.2", 2),
            bytecode::stack::CONST_3 => pushstack!(self, code[n], "const.3", 3),
            bytecode::stack::CONST_4 => pushstack!(self, code[n], "const.4", 4),
            bytecode::stack::CONST_8 => pushstack!(self, code[n], "const.8", 8),
            bytecode::stack::CONST_16 => pushstack!(self, code[n], "const.16", 16),
            bytecode::stack::CONST_32 => pushstack!(self, code[n], "const.32", 32),
            bytecode::stack::CONST_64 => pushstack!(self, code[n], "const.64", 64),
            bytecode::stack::CONST_128 => pushstack!(self, code[n], "const.128", 128),
            bytecode::stack::CONST_N1 => pushstack!(self, code[n], "const.n1", (-1 as i64)),
            bytecode::stack::CONST_U16 => {
                pushstack!(self, code[n], "const.u16", cval_u16!(code, n, "const.u16"));
                return Ok(n+2);
            },
            bytecode::stack::CONST_U32 => {
                pushstack!(self, code[n], "const.u32", cval_u32!(code, n, "const.u32"));
                return Ok(n+3);
            },
            bytecode::stack::CONST_U64 => {
                pushstack!(self, code[n], "const.u64", cval_u64!(code, n, "const.u64"));
                return Ok(n + 5);
            },
            bytecode::stack::CONST_I16 => {
                pushstack!(self, code[n], "const.i16", ((cval_u16!(code, n, "const.i16") as i16) as i64));
                return Ok(n+2);
            },
            bytecode::stack::CONST_I32 => {
                pushstack!(self, code[n], "const.i32", ((cval_u32!(code, n, "const.i32") as i32) as i64));
                return Ok(n+3);
            },
            bytecode::stack::DUPE => {
                let num = popstack1!(self, code[n], "dupe");
                checkstack!(self, code[n], "dupe", num);
                if (self.stack.len() as u64) < num {
                    return Err(BytecodeError::stack_underflow(code[n], "dupe", num));
                }
                // We can be 100% certain num fits within usize due to the above
                // checks
                let base = self.stack.len() - (num as usize);
                for i in 0..(num as usize) {
                    self.stack.push(self.stack[base+i]);
                }
            },
            bytecode::stack::DUPE_1 => {
                checkstack!(self, code[n], "dupe.1", 1);
                if self.stack.len() < 1 {
                    return Err(BytecodeError::stack_underflow(code[n], "dupe.1", 1));
                }
                self.stack.push(self.stack[self.stack.len()-1]);
            },
            bytecode::stack::DUPE_C => {
                let num = cval_u16!(code, n, "dupe.c") as usize;
                checkstack!(self, code[n], "dupe.c", num as u64);
                if self.stack.len() < num {
                    return Err(BytecodeError::stack_underflow(code[n], "dupe.c", num as u64))
                }
                let base = self.stack.len() - num;
                for i in 0..num {
                    self.stack.push(self.stack[base+i]);
                }
                return Ok(n+2);
            },
            _ => {
                return Err(BytecodeError::BadOpcode(code[n]));
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
                return Err(BytecodeError::BadOpcode(code[n]));
            }
        }
        Ok(n + 1)
    }
}