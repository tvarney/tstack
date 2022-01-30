//! Execution context definitions

use std::rc::Rc;

use crate::module::Module;
use crate::errors::BytecodeError;

/// An execution context of the virtual machine
///
/// The Context struct contains an 'execution context' within the virtual
/// machine, holding the currently executing module (which contains the
/// bytecode) and an instruction pointer into it.
///
/// When combined with a stack, this allows for function calls both within
/// and between modules.
pub struct Context {
    module: Rc<Module>,
    offset: usize,
    current: u16,
}

impl Context {
    /// Create a new context with the given module and instruction pointer
    pub fn new(module: Rc<Module>, offset: usize) -> Result<Context, BytecodeError> {
        if offset >= (*module).bytecode.len() {
            return Err(BytecodeError::InvalidAddress(offset));
        }
        let current = (*module).bytecode[offset];
        Ok(Context{
            module: module,
            offset: offset,
            current: current,
        })
    }

    /// Get the currently executing opcode
    #[inline]
    pub fn opcode(&self) -> u16 {
        self.current
    }

    /// Fetch the next opcode and increment the current instruction pointer
    #[inline]
    pub fn next(&mut self) -> Option<u16> {
        if !self.has_next() {
            return None
        }
        let v = (*(self.module)).bytecode[self.offset];
        self.offset += 1;

        Some(v)
    }

    /// Check if there exists an opcode at the current offset
    #[inline]
    pub fn has_next(&self) -> bool {
        self.offset < (*(self.module)).bytecode.len()
    }

    /// Use the next value in the bytecode as a constant u16 value
    #[inline]
    pub fn cval_u16(&mut self) -> Result<u16, BytecodeError> {
        if (*(self.module)).bytecode.len() - self.offset < 1 {
            return Err(BytecodeError::code_data(self.current, 1));
        }
        let v = (*(self.module)).bytecode[self.offset];
        self.offset += 1;
        Ok(v)
    }

    /// Use the next two values in the bytecode as two u16 values
    #[inline]
    pub fn cval_u16_2(&mut self) -> Result<(u16, u16), BytecodeError> {
        if (*(self.module)).bytecode.len() - self.offset < 2 {
            return Err(BytecodeError::code_data(self.current, 2));
        }
        let v1 = (*(self.module)).bytecode[self.offset];
        let v2 = (*(self.module)).bytecode[self.offset+1];
        self.offset += 2;
        Ok((v1, v2))
    }

    /// Use the next four values in the bytecode as four u16 values
    #[inline]
    pub fn cval_u16_4(&mut self) -> Result<(u16, u16, u16, u16), BytecodeError> {
        if (*(self.module)).bytecode.len() - self.offset < 4 {
            return Err(BytecodeError::code_data(self.current, 4));
        }
        let v1 = (*(self.module)).bytecode[self.offset];
        let v2 = (*(self.module)).bytecode[self.offset + 1];
        let v3 = (*(self.module)).bytecode[self.offset + 2];
        let v4 = (*(self.module)).bytecode[self.offset + 3];
        self.offset += 4;
        Ok((v1, v2, v3, v4))
    }

    /// Use the next two values in the bytecode as a single u32 value
    #[inline]
    pub fn cval_u32(&mut self) -> Result<u32, BytecodeError> {
        let (v1, v2) = self.cval_u16_2()?;
        Ok(((v1 as u32) << 16) | (v2 as u32))
    }

    /// Use the next four values in the bytecode as a single u64 value
    #[inline]
    pub fn cval_u64(&mut self) -> Result<u64, BytecodeError> {
        let (v1, v2, v3, v4) = self.cval_u16_4()?;
        Ok(((v1 as u64) << 48) | ((v2 as u64) << 32) | ((v3 as u64) << 16) | (v4 as u64))
    }
}