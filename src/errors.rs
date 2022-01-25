//! Definitions of error types the engine can return

/// Information about the faulting instruction
#[derive(Debug, Clone)]
pub struct Instruction {
    code: u16,
    name: &'static str,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ({:#06x})", self.name, self.code)
    }
}

/// Information about a number of required values
#[derive(Debug, Clone)]
pub struct RequiredValues {
    instruction: Instruction,
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
    StackOverflow(Instruction),
    StackUnderflow(RequiredValues),
    CodeData(RequiredValues),
    BadOpCode(u16),
}

impl std::fmt::Display for BytecodeError {
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
            BytecodeError::BadOpCode(v) => {
                write!(f, "invalid opcode {:#06x}", v)
            }
        }
    }
}

impl BytecodeError {
    pub fn stack_overflow(opcode: u16, name: &'static str) -> BytecodeError {
        BytecodeError::StackOverflow(Instruction{
            code: opcode,
            name: name,
        })
    }

    pub fn stack_underflow(opcode: u16, name: &'static str, req: u64) -> BytecodeError {
        BytecodeError::StackUnderflow(RequiredValues{
            instruction: Instruction {
                code: opcode,
                name: name,
            },
            required: req,
        })
    }

    pub fn code_data(opcode: u16, name: &'static str, req: u64) -> BytecodeError {
        BytecodeError::CodeData(RequiredValues{
            instruction: Instruction {
                code: opcode,
                name: name,
            },
            required: req,
        })
    }
}
