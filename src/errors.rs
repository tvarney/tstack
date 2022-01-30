//! Definitions of error types the engine can return

/// Information about a number of required values
#[derive(Debug, Clone)]
pub struct RequiredValues {
    instruction: u16,
    required: u64,
}

/// Instruction fault error type
///
/// This is the error type for instruction faults when the engine is running.
/// Instruction faults indicate that an instruction level error occurred during
/// the execution of the bytecode; these faults may indicate that the bytecode
/// is malformed in some way, or that a logic error was made when constructing
/// the bytecode. Additionally, memory issues (out of memory, stack overflows,
/// etc) are handled by faults as well.
///
/// Currently there is no way for running code to handle faults, though it is
/// planned to add a signals like interface for registering fault handlers.
#[derive(Debug, Clone)]
pub enum BytecodeError {
    BadOpcode(u16),
    CodeData(RequiredValues),
    InvalidAddress(usize),
    InvalidModule(u32),
    InvalidSymbol(u32),
    StackOverflow(u16),
    StackUnderflow(RequiredValues),
}

impl std::fmt::Display for BytecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BytecodeError::BadOpcode(v) => {
                write!(f, "invalid opcode {:#06x}", v)
            },
            BytecodeError::CodeData(r) => {
                write!(f, "insufficient data bytes for {:#06x}; {} bytes required", r.instruction, r.required)
            },
            BytecodeError::InvalidAddress(addr) => {
                write!(f, "invalid address {}", addr)
            },
            BytecodeError::InvalidModule(id) => {
                write!(f, "invalid module ID {}", id)
            },
            BytecodeError::InvalidSymbol(id) => {
                write!(f, "invalid symbol ID {}", id)
            },
            BytecodeError::StackOverflow(i) => {
                write!(f, "stack size exceeded maximum allowed on opcode {}", i)
            },
            BytecodeError::StackUnderflow(r) => {
                write!(f, "too few operands for {:#06x}; {} values required", r.instruction, r.required)
            }
        }
    }
}

impl BytecodeError {
    /// Create a new BytecodeError::StackOverflow error
    pub fn stack_overflow(opcode: u16) -> BytecodeError {
        BytecodeError::StackOverflow(opcode)
    }

    /// Create a new BytecodeError::StackUnderflow error
    pub fn stack_underflow(opcode: u16, req: u64) -> BytecodeError {
        BytecodeError::StackUnderflow(RequiredValues{
            instruction: opcode,
            required: req,
        })
    }

    /// Create a new BytecodeError::CodeData error
    pub fn code_data(opcode: u16, req: u64) -> BytecodeError {
        BytecodeError::CodeData(RequiredValues{
            instruction: opcode,
            required: req,
        })
    }

    /// Check if the BytecodeError is a BytecodeError::StackOverflow instance
    pub fn is_stack_overflow(&self) -> bool {
        if let BytecodeError::StackOverflow(_) = self {
            return true;
        }
        false
    }

    /// Check if the BytecodeError is a BytecodeError::StackUnderflow instance
    pub fn is_stack_underflow(&self) -> bool {
        if let BytecodeError::StackUnderflow(_) = self {
            return true;
        }
        false
    }

    /// Check if the BytecodeError is a BytecodeError::CodeData instance
    pub fn is_code_data(&self) -> bool {
        if let BytecodeError::CodeData(_) = self {
            return true;
        }
        false
    }

    /// Check if the BytecodeError is a BytecodeError::BadOpcode instance
    pub fn is_bad_opcode(&self) -> bool {
        if let BytecodeError::BadOpcode(_) = self {
            return true;
        }
        false
    }
}

/// A compound error type for errors when defining modules
#[derive(Clone, Debug)]
pub enum ModuleError {
    InvalidName(String),
    NameCollision(String),
}

impl std::fmt::Display for ModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ModuleError::InvalidName(name) => {
                write!(f, "invalid module name {}", name)
            },
            ModuleError::NameCollision(name) => {
                write!(f, "module {} already defined", name)
            }
        }
    }
}
