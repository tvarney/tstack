//! Definitions of error types the engine can return

/// Information about the faulting instruction
#[derive(Debug, Clone)]
pub struct Instruction<'a> {
    code: u16,
    name: &'a str,
}

impl<'a> std::fmt::Display for Instruction<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ({:#06x})", self.name, self.code)
    }
}

/// Information about a number of required values
#[derive(Debug, Clone)]
pub struct RequiredValues<'a> {
    instruction: Instruction<'a>,
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
pub enum BytecodeError<'a> {
    StackOverflow(Instruction<'a>),
    StackUnderflow(RequiredValues<'a>),
    CodeData(RequiredValues<'a>),
    BadOpcode(u16),
}

impl<'a> std::fmt::Display for BytecodeError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BytecodeError::StackOverflow(i) => {
                write!(f, "stack size exceeded maximum allowed on opcode {}", i)
            },
            BytecodeError::StackUnderflow(r) => {
                write!(f, "too few operands for {}; {} values required", r.instruction, r.required)
            },
            BytecodeError::CodeData(r) => {
                write!(f, "insufficient data bytes for {}; {} bytes required", r.instruction, r.required)
            },
            BytecodeError::BadOpcode(v) => {
                write!(f, "invalid opcode {:#06x}", v)
            }
        }
    }
}

impl<'a> BytecodeError<'a> {
    pub fn stack_overflow(opcode: u16, name: &'a str) -> BytecodeError<'a> {
        BytecodeError::StackOverflow(Instruction{
            code: opcode,
            name: name,
        })
    }

    pub fn stack_underflow(opcode: u16, name: &'a str, req: u64) -> BytecodeError<'a> {
        BytecodeError::StackUnderflow(RequiredValues{
            instruction: Instruction {
                code: opcode,
                name: name,
            },
            required: req,
        })
    }

    pub fn code_data(opcode: u16, name: &'a str, req: u64) -> BytecodeError<'a> {
        BytecodeError::CodeData(RequiredValues{
            instruction: Instruction {
                code: opcode,
                name: name,
            },
            required: req,
        })
    }

    pub fn is_stack_overflow(&self) -> bool {
        if let BytecodeError::StackOverflow(_) = self {
            return true;
        }
        false
    }

    pub fn is_stack_underflow(&self) -> bool {
        if let BytecodeError::StackUnderflow(_) = self {
            return true;
        }
        false
    }

    pub fn is_code_data(&self) -> bool {
        if let BytecodeError::CodeData(_) = self {
            return true;
        }
        false
    }

    pub fn is_bad_opcode(&self) -> bool {
        if let BytecodeError::BadOpcode(_) = self {
            return true;
        }
        false
    }
}
