//! Definitions of error types the engine can return

/// StackError is an error indicating that a stack pop operation failed
///
/// This error type is indicative of a compilation failure, as it is the result
/// of the bytecode not pushing enough values onto the stack.
#[derive(Debug, Clone)]
pub struct StackError {
    opcode: u16,
    opname: &'static str,
    reqvalues: u32,
}

impl std::fmt::Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "too few values in stack for {} ({:#06x}); {} values required",
            self.opname, self.opcode, self.reqvalues
        )
    }
}

impl StackError {
    /// Create a new StackError for the given opcode and number of required
    /// values
    pub fn new(opcode: u16, name: &'static str, reqvals: u32) -> StackError {
        StackError {
            opcode: opcode,
            opname: name,
            reqvalues: reqvals,
        }
    }
}

/// ValueError is an error indicating that a load operation failed due to
/// insufficient remaining bytes in the bytecode
pub struct ValueError {
    opcode: u16,
    opname: &'static str,
    reqbytes: u32,
}

impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "unexpected end of code while parsing value for {} ({:#06x}); {} words required",
            self.opname, self.opcode, self.reqbytes
        )
    }
}

impl ValueError {
    /// Creates a new ValueError
    pub fn new(opcode: u16, name: &'static str, reqbytes: u32) -> ValueError {
        ValueError {
            opcode: opcode,
            opname: name,
            reqbytes: reqbytes,
        }
    }
}

/// BytecodeError is the generic error type returned by the run() function of
/// the engine
pub enum BytecodeError {
    StackError(StackError),
    ValueError(ValueError),
    BadOpCodeError(u16),
}

impl std::fmt::Display for BytecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BytecodeError::StackError(e) => write!(f, "{}", e),
            BytecodeError::ValueError(e) => write!(f, "{}", e),
            BytecodeError::BadOpCodeError(v) => write!(f, "invalid opcode {:#06x}", v),
        }
    }
}
