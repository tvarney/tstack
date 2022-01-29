
#[macro_use]
mod macros;

pub mod bytecode;
pub mod context;
pub mod module;
pub mod errors;

use std::collections::HashMap;
use std::num::Wrapping;
use std::rc::Rc;

use context::Context;
use self::errors::{BytecodeError, ModuleError};
use module::Module;

/// The virtual machine engine
pub struct Engine {
    /// The operand stack, used to hold dynamic arguments to instructions
    pub stack: Vec<u64>,

    /// The maximum depth of the stack
    pub maxstack: usize,

    /// The list of loaded modules
    pub modules: Vec<Rc<Module>>,

    /// A lookup between module names and module id
    pub module_lookup: HashMap<String, u32>,

    pub context: Context,
}

impl Engine {
    /// Create a new Engine instance
    pub fn new() -> Engine {
        Engine {
            stack: Vec::new(),
            maxstack: 0x8FFF,
            modules: Vec::new(),
            module_lookup: HashMap::new(),
            context: Context::new(Rc::new(Module{
                    name: String::from(""),
                    strings: vec![],
                    local_symbols: vec![],
                    external_symbols: vec![],
                    bytecode: vec![inst_sys!(NOP)],
                    symbol_lookup: HashMap::new(),
                }), 0,
            ).unwrap(),
        }
    }

    pub fn add_module(&mut self, module: Rc<Module>) -> Result<(), ModuleError> {
        if let Some(_) = self.module_lookup.get(&module.name) {
            return Err(ModuleError::NameCollision(module.name.clone()));
        }

        self.modules.push(Rc::clone(&module));

        Ok(())
    }

    fn get_context(&self, module_id: u32, symbol_id: u32) -> Result<Context, BytecodeError> {
        if module_id as u64 >= self.modules.len() as u64 {
            return Err(BytecodeError::InvalidModule(module_id));
        }
        let module = &self.modules[module_id as usize];
        if symbol_id as u64 >= module.local_symbols.len() as u64 {
            return Err(BytecodeError::InvalidSymbol(symbol_id));
        }
        let symbol = &module.local_symbols[symbol_id as usize];
        Ok(Context::new(Rc::clone(module), symbol.code_offset as usize)?)
    }

    pub fn run(&mut self, module_id: u32, symbol_id: u32) -> Result<(), BytecodeError> {
        self.context = self.get_context(module_id, symbol_id)?;

        while self.context.has_next() {
            let opcode = self.context.next().unwrap();
            let group = ((opcode & bytecode::GROUP_MASK) >> bytecode::GROUP_SHIFT) as u8;
            let value = ((opcode & bytecode::DATA_MASK) >> bytecode::DATA_SHIFT) as u8;
            println!("Opcode: {:#06x} ({:#04x}|{:#04x})", opcode, group, value);
            match group {
                bytecode::groups::SYSTEM => self.op_system(opcode, value)?,
                bytecode::groups::STACK  => self.op_stack(opcode, value)?,
                bytecode::groups::MATH   => self.op_math(opcode, value)?,
                _ => {
                    // Unimplemented
                    return Err(BytecodeError::BadOpcode(opcode));
                }
            };
        }

        Ok(())
    }

    fn op_system(&mut self, opcode: u16, value: u8) -> Result<(), BytecodeError> {
        match value {
            bytecode::sys::NOP => (),
            bytecode::sys::PRINT_STACK => {
                println!("Stack: {:?}", self.stack);
            },
            bytecode::sys::PRINT_U64 => {
                let value = popstack1!(self, opcode);
                println!("PRINT: {}", value);
            },
            bytecode::sys::PRINT_I64 => {
                let value = popstack1!(self, opcode);
                println!("PRINT: {}", value as i64);
            },
            bytecode::sys::PRINT_F32 => {
                let value = popstack1!(self, opcode);
                println!("PRINT: {}", f32::from_bits(value as u32));
            },
            bytecode::sys::PRINT_F64 => {
                let value = popstack1!(self, opcode);
                println!("PRINT: {}", f64::from_bits(value));
            },
            _ => {
                // Unimplemented
                return Err(BytecodeError::BadOpcode(opcode));
            }
        }
        Ok(())
    }

    fn op_stack(&mut self, opcode: u16, value: u8) -> Result<(), BytecodeError> {
        match value {
            //bytecode::stack::CONST_0 => self.stack.push(0),
            bytecode::stack::CONST_0 => pushstack!(self, opcode, 0),
            bytecode::stack::CONST_1 => pushstack!(self, opcode, 1),
            bytecode::stack::CONST_2 => pushstack!(self, opcode, 2),
            bytecode::stack::CONST_3 => pushstack!(self, opcode, 3),
            bytecode::stack::CONST_4 => pushstack!(self, opcode, 4),
            bytecode::stack::CONST_8 => pushstack!(self, opcode, 8),
            bytecode::stack::CONST_16 => pushstack!(self, opcode, 16),
            bytecode::stack::CONST_32 => pushstack!(self, opcode, 32),
            bytecode::stack::CONST_64 => pushstack!(self, opcode, 64),
            bytecode::stack::CONST_128 => pushstack!(self, opcode, 128),
            bytecode::stack::CONST_N1 => pushstack!(self, opcode, (-1 as i64)),
            bytecode::stack::CONST_U16 => pushstack!(self, opcode, self.context.cval_u16()?),
            bytecode::stack::CONST_U32 => pushstack!(self, opcode, self.context.cval_u32()?),
            bytecode::stack::CONST_U64 => pushstack!(self, opcode, self.context.cval_u64()?),
            bytecode::stack::CONST_I16 => pushstack!(self, opcode, ((self.context.cval_u16()? as i16) as i64)),
            bytecode::stack::CONST_I32 => pushstack!(self, opcode, ((self.context.cval_u32()? as i32) as i64)),
            bytecode::stack::DUPE => {
                let num = popstack1!(self, opcode);
                checkstack!(self, opcode, num);
                if (self.stack.len() as u64) < num {
                    return Err(BytecodeError::stack_underflow(opcode, num));
                }
                // We can be 100% certain num fits within usize due to the above
                // checks
                let base = self.stack.len() - (num as usize);
                for i in 0..(num as usize) {
                    self.stack.push(self.stack[base+i]);
                }
            },
            bytecode::stack::DUPE_1 => {
                checkstack!(self, opcode, 1);
                if self.stack.len() < 1 {
                    return Err(BytecodeError::stack_underflow(opcode, 1));
                }
                self.stack.push(self.stack[self.stack.len()-1]);
            },
            bytecode::stack::DUPE_C => {
                let num = self.context.cval_u16()? as usize;
                checkstack!(self, opcode, num as u64);
                if self.stack.len() < num {
                    return Err(BytecodeError::stack_underflow(opcode, num as u64))
                }
                let base = self.stack.len() - num;
                for i in 0..num {
                    self.stack.push(self.stack[base+i]);
                }
            },
            _ => {
                return Err(BytecodeError::BadOpcode(opcode));
            }
        }
        Ok(())
    }

    fn op_math(&mut self, opcode: u16, value: u8) -> Result<(), BytecodeError> {
        match value {
            bytecode::math::ADD => {
                let (v1, v2) = popstack2!(self, opcode);
                self.stack.push((Wrapping(v1) + Wrapping(v2)).0);
            },
            bytecode::math::ADD_C => {
                let c = self.context.cval_u16()? as u64;
                let v = popstack1!(self, opcode);
                self.stack.push((Wrapping(c) + Wrapping(v)).0);
            },
            bytecode::math::SUB => {
                let (v1, v2) = popstack2!(self, opcode);
                self.stack.push((Wrapping(v1) - Wrapping(v2)).0);
            },
            bytecode::math::SUB_C => {
                let c = self.context.cval_u16()? as u64;
                let v = popstack1!(self, opcode);
                self.stack.push((Wrapping(v) - Wrapping(c)).0);
            },
            bytecode::math::MUL => {
                let (v1, v2) = popstack2!(self, opcode);
                self.stack.push((Wrapping(v1) * Wrapping(v2)).0);
            },
            bytecode::math::MUL_C => {
                let c = self.context.cval_u16()? as u64;
                let v = popstack1!(self, opcode);
                self.stack.push((Wrapping(v) * Wrapping(c)).0);
            },
            bytecode::math::DIV => {
                let (v1, v2) = popstack2!(self, opcode);
                self.stack.push((Wrapping(v1) / Wrapping(v2)).0);
            },
            bytecode::math::DIV_C => {
                let c = self.context.cval_u16()? as u64;
                let v = popstack1!(self, opcode);
                self.stack.push((Wrapping(v) / Wrapping(c)).0);
            }
            _ => {
                return Err(BytecodeError::BadOpcode(opcode));
            }
        }
        Ok(())
    }
}