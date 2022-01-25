//! Provides constants defining the bytecode
//!
//! Provides constants for defining bytecode, separated out into separate
//! modules to group constants by use.
//!
//! Each bytecode value is a 16-bit value where the first 8 bits are the group
//! code, and the final 8 bits are the 'data' for that group.

/// The mask for the group byte
pub const GROUP_MASK: u16 = 0xFF00;

/// The shift for group bytes to normalize it into a u8
pub const GROUP_SHIFT: u16 = 8;

/// The mask for the data byte
pub const DATA_MASK: u16 = 0x00FF;

/// The shift for data bytes to normalize it into a u8
///
/// This value is included for completeness, but is otherwise useless as a shift
/// by zero doesn't actually do anything.
pub const DATA_SHIFT: u16 = 0;

/// Generate a stack instruction
///
/// # Examples
/// ```
/// use tstack;
/// let bytes = &[
///     tstack::inst_stack!(CONST_N1),
///     tstack::inst_stack!(CONST_64),
///     tstack::inst_stack!(POP),
/// ];
/// ```
#[macro_export]
macro_rules! inst_stack {
    ($instr:ident) => {
        ((($crate::bytecode::groups::STACK as u16) << 8) | ($crate::bytecode::stack::$instr as u16))
    }
}

/// Generate a system instruction
///
/// # Examples
/// ```
/// use tstack;
/// let bytes = &[
///     tstack::inst_sys!(NOP),
///     tstack::inst_sys!(PRINT_STACK),
/// ];
/// ```
#[macro_export]
macro_rules! inst_sys {
    ($instr:ident) => {
        ((($crate::bytecode::groups::SYSTEM as u16) << 8) | ($crate::bytecode::sys::$instr as u16))
    }
}

/// Generate a math instruction
///
/// # Examples
/// ```
/// use tstack;
/// let bytes = &[
///     tstack::inst_math!(ADD),
///     tstack::inst_math!(ADD_C), 0x0001,
/// ];
/// ```
#[macro_export]
macro_rules! inst_math {
    ($instr:ident) => {
        ((($crate::bytecode::groups::MATH as u16) << 8) | ($crate::bytecode::math::$instr as u16))
    }
}

/// Generate a jump instruction
///
/// # Examples
/// ```
/// use tstack;
/// ```
#[macro_export]
macro_rules! inst_jump {
    ($src:ident, $mode:ident) => {
        (
            (($crate::bytecode::groups::JUMP as u16) << 8)
            | ($crate::bytecode::jump::$src as u16)
            | ($crate::bytecode::jump::$mode as u16)
            | ($crate::bytecode::jump::CONDITIONAL_FALSE as u16)
        )
    };
    ($src:ident, $mode:ident, $condition:ident) => {
        (
            (($crate::bytecode::groups::JUMP as u16) << 8)
            | ($crate::bytecode::jump::$src as u16)
            | ($crate::bytecode::jump::$mode as u16)
            | ($crate::bytecode::jump::CONDITIONAL_TRUE as u16)
            | ($crate::bytecode::jump::$condition as u16)
        )
    }
}

/// Groups broadly correspond to a type of instruction.
///
/// The group value is the first byte of a bytecode value, and is used to group
/// instructions by similar purpose.
pub mod groups {
    /// Group for misc. system instructions and debug operations
    pub const SYSTEM: u8 = 0x00;

    /// Group for stack maniuplation instructions
    pub const STACK: u8 = 0x01;

    /// Group for jumps within the same context
    pub const JUMP: u8 = 0x02;

    /// Group for arithmetic operations on integers
    pub const MATH: u8 = 0x03;

    /// Group for arithmetic operations on floating point values
    pub const FPMATH: u8 = 0x04;

    /// Group for stack and frame based jumps, possibly between contexts
    pub const FUNCTION: u8 = 0x05;
}

/// Misc system instructions and debug operations
///
///| Constant    | ID   | Args | Stack     | Description
///|-------------|------|------|-----------|------------
///| NOP         |`0x00`|      |           | Do nothing
///| HALT        |`0x01`|      |           | Stop execution normally
///| PRINT_STACK |`0x01`|      |           | Debug print the contents of the stack as u64 values
///| PRINT_U64   |`0x02`|      |`[a] -> []`| Debug print the topmost stack value as u64
///| PRINT_I64   |`0x03`|      |`[a] -> []`| Debug print the topmost stack value as i64
///| PRINT_F32   |`0x04`|      |`[a] -> []`| Debug print the topmost stack value as f32 (truncating)
///| PRINT_F64   |`0x05`|      |`[a] -> []`| Debug print the topmost stack value as f64
///| FAULT       |`0xFF`|      |           | Force a fault
pub mod sys {
    pub const NOP:         u8 = 0x00;
    pub const HALT:        u8 = 0x01;
    pub const PRINT_STACK: u8 = 0x02;
    pub const PRINT_U64:   u8 = 0x03;
    pub const PRINT_I64:   u8 = 0x04;
    pub const PRINT_F32:   u8 = 0x05;
    pub const PRINT_F64:   u8 = 0x06;
    pub const BREAKPOINT:  u8 = 0x07;
}

/// Stack and frame manipulation instruction data byte values.
///
///| Constant   | ID   | Args  | Stack                               | Description
///|------------|------|-------|-------------------------------------|------------
///| CONST_0    |`0x00`|       |`[] -> [0]`                          | Push constant `0`
///| CONST_1    |`0x01`|       |`[] -> [1]`                          | Push constant `1`
///| CONST_2    |`0x02`|       |`[] -> [2]`                          | Push constant `2`
///| CONST_3    |`0x03`|       |`[] -> [3]`                          | Push constant `3`
///| CONST_4    |`0x04`|       |`[] -> [4]`                          | Push constant `4`
///| CONST_8    |`0x05`|       |`[] -> [8]`                          | Push constant `8`
///| CONST_16   |`0x06`|       |`[] -> [16]`                         | Push constant `16`
///| CONST_32   |`0x07`|       |`[] -> [32]`                         | Push constant `32`
///| CONST_64   |`0x08`|       |`[] -> [64]`                         | Push constant `64`
///| CONST_128  |`0x09`|       |`[] -> [128]`                        | Push constant `128`
///| CONST_N1   |`0x0A`|       |`[] -> [-1]`                         | Push constant `-1`
///| CONST_U16  |`0x0B`|`c:u16`|`[] -> [c]`                          | Push zero-extended 16-bit constant `$c`
///| CONST_U32  |`0x0C`|`c:u32`|`[] -> [c]`                          | Push zero-extended 32-bit constant `$c`; used for non-widened f32 as well
///| CONST_U64  |`0x0D`|`c:u64`|`[] -> [c]`                          | Push 64-bit constant `$c`; used for f64 and i64 as well
///| CONST_I16  |`0x0E`|`c:i16`|`[] -> [c]`                          | Push sign-extended 16-bit constant `$c`
///| CONST_I32  |`0x0F`|`c:i32`|`[] -> [c]`                          | Push sign-extended 32-bit constant `$c`
///| DUPE       |`0x10`|       |`[a1...a$n,n] -> [a1...a$n,a1...a$n]`| Duplicate the topmost `$n` stack elements
///| DUPE_1     |`0x11`|       |`[a]          -> [a,a]`              | Duplicate the topmost stack element
///| DUPE_C     |`0x12`|`c:u16`|`[a1...a$c]   -> [a1...a$c,a1...a$c]`| Duplicate the topmost `$c` stack elements
///| SWAP       |`0x13`|       |`[a1...a$n,n] -> [a$n...a$1]`        | Reverse the topmost `$n` stack elements
///| SWAP_1     |`0x14`|       |`[a,b]        -> [b,a]`              | Swap the two topmost stack elements
///| SWAP_C     |`0x15`|`c:u16`|`[a1...a$c]   -> [a$c...a1]`         | Reverse the topmost `$c` stack elements
///| ROTATE     |`0x16`|       |`[a1...a$n,n,p] -> [a$p...a$n,a1...a$p-1]`| Rotate the topmost `$p` stack elements back `$n` places
///| ROTATE_1   |`0x17`|       |`[a1...a$n,n] -> [a$n,a1...a$n-1]`   | Rotate the topmost element back `$n` places
///| ROTATE_C   |`0x18`|`c:u16`|`[a1...a$c,p] -> [a$p...a$c,a1...a$p-1]`| Rotate the topmost `$p` stack elements back `$c` places
///| ROTATE_1_C |`0x19`|`c:u16`|`[a1...a$c]   -> [a$c,a1...a$c-1]`   | Rotate the topmost element back `$c` places
///| POP        |`0x1A`|       |`[a1...a$n,n] -> []`                 | Remove the topmost `$n` elements from the stack
///| POP_1      |`0x1B`|       |`[a]          -> []`                 | Remove the topmost element from the stack
///| POP_C      |`0x1C`|`c:u16`|`[a1...a$c]   -> []`                 | Remove the topmost `$c` elements from the stack
///| GET_U8     |`0x1D`|       |`[n] -> [local[$n/8]]`               | Zero extend and push 8-bit local at index `$n/8`[^n1]
///| GET_U8_C   |`0x1E`|`c:u16`|`[]  -> [local[$c/8]]`               | Zero extend and push 8-bit local at index `$c/8`[^n1]
///| GET_U16    |`0x1F`|       |`[n] -> [local[$n/4]]`               | Zero extend and push 16-bit local at index `$n/4`[^n1]
///| GET_U16_C  |`0x20`|`c:u16`|`[]  -> [local[$c/4]]`               | Zero extend and push 16-bit local at index `$c/4`[^n1]
///| GET_U32    |`0x21`|       |`[n] -> [local[$n/2]]`               | Zero extend and push 32-bit local at index `$n/2`[^n1]
///| GET_U32_C  |`0x22`|`c:u16`|`[]  -> [local[$c/2]]`               | Zero extend and push 32-bit local at index `$c/2`[^n1]
///| GET_U64    |`0x23`|       |`[n] -> [local[$n]]`                 | Push 64-bit local at index `$n`
///| GET_U64_C  |`0x24`|`c:u16`|`[]  -> [local[$c]]`                 | Push 64-bit local at index `$c`
///| GET_I8     |`0x25`|       |`[n] -> [local[$n/8]]`               | Sign extend and push 8-bit local at index `$n/8`[^n1]
///| GET_I8_C   |`0x26`|`c:u16`|`[]  -> [local[$c/8]]`               | Sign extend and push 8-bit local at index `$c/8`[^n1]
///| GET_I16    |`0x27`|       |`[n] -> [local[$n/4]]`               | Sign extend and push 16-bit local at index `$n/4`[^n1]
///| GET_I16_C  |`0x28`|`c:u16`|`[]  -> [local[$c/4]]`               | Sign extend and push 16-bit local at index `$c/4`[^n1]
///| GET_I32    |`0x29`|       |`[n] -> [local[$n/2]]`               | Sign extend and push 32-bit local at index `$n/2`[^n1]
///| GET_I32_C  |`0x2A`|`c:u16`|`[]  -> [local[$c/2]]`               | Sign extend and push 32-bit local at index `$c/2`[^n1]
///| SET_U8     |`0x2D`|       |`[v,n] -> []; local[$n/8]=$v`[^n2]   | Truncate `$v` to 8-bits and save to local at index `$n/8`[^n3]
///| SET_U8_C   |`0x2E`|`c:u16`|`[v]   -> []; local[$c/8]=$v`[^n2]   | Truncate `$v` to 8-bits and save to local at index `$c/8`[^n3]
///| SET_U16    |`0x2F`|       |`[v,n] -> []; local[$n/4]=$v`[^n2]   | Truncate `$v` to 16-bits and save to local at index `$n/4`[^n3]
///| SET_U16_C  |`0x30`|`c:u16`|`[v]   -> []; local[$c/4]=$v`[^n2]   | Truncate `$v` to 16-bits and save to local at index `$c/4`[^n3]
///| SET_U32    |`0x31`|       |`[v,n] -> []; local[$n/2]=$v`[^n2]   | Truncate `$v` to 32-bits and save to local at index `$n/2`[^n3]
///| SET_U32_C  |`0x32`|`c:u16`|`[v]   -> []; local[$c/2]=$v`[^n2]   | Truncate `$v` to 32-bits and save to local at index `$c/2`[^n3]
///| SET_U64    |`0x33`|       |`[v,n] -> []; local[$n]=$v`[^n2]     | Save `$v` to local at index `$n`[^n3]
///| SET_U64_C  |`0x34`|`c:u16`|`[v]   -> []; local[$c]=$v`[^n2]     | Save `$v` to local at index `$c`[^n3]
///| STACK_SIZE |`0x37`|       |`[a1...a$n] -> [a1...a$n,n]`         | Push the size of the stack to the stack
///| PUSH_STACK |`0x38`|       |`[a1...a$n] -> [a1...a$n,n\|]`       | Push the size of the stack and set the new stack base 1 past it
///| POP_STACK  |`0x39`|       |`[a1...a$n,n\|b1...] -> [a1...a$n,b1...]`| Fetch previous stack size, subtract from current stack base, and shift other elements
///| RESERVE    |`0x3A`|       |`[n] -> []`                          | Extend or reduce the number of locals reserved by `$n`[^n4]
///| RESERVE_C  |`0x3B`|`c:i16`|                                     | Extend or reduce the number of locals reserved by `$c`[^n4]
///
///
/// [^n1]: The index for types smaller than 64-bits are for values packed
///     into locals such that all of the space is used. E.g. for an 8-bit value,
///     the actual index will be calculated as `$index / 8`, with the value
///     shifted by `8 * ($index % 8)` and then masked by `0xFF`.
///
/// [^n2]: Set instructions operate on the local storage within the current
///     frame. The adjusted index being written to must have already been
///     reserved by an instance of `RESERVE` or `RESERVE_C`.
///
/// [^n3]: Set instructions pack values into an adjusted index. E.g. for an
///     8-bit value, the actual index will be calculated as `$index / 8`, with
///     the value shifted by `8 * ($index % 8)`.
///
/// [^n4]: The reserve instructions take their argument as a signed value. For
///     positive values the instruction will extend the local storage by that
///     many values. For negative values the instruction will reduce the local
///     storage by that many values. If the result of the instruction would
///     result in a local storage size `n` where `$n > 255` or `$n < 0`, the
///     machine will fault. E.g. `reserve.c 0x7FFF` will always fault, as it
///     attempts to reserve ~32,000 locals.
pub mod stack {
    pub const CONST_0:    u8 = 0x00;
    pub const CONST_1:    u8 = 0x01;
    pub const CONST_2:    u8 = 0x02;
    pub const CONST_3:    u8 = 0x03;
    pub const CONST_4:    u8 = 0x04;
    pub const CONST_8:    u8 = 0x05;
    pub const CONST_16:   u8 = 0x06;
    pub const CONST_32:   u8 = 0x07;
    pub const CONST_64:   u8 = 0x08;
    pub const CONST_128:  u8 = 0x09;
    pub const CONST_N1:   u8 = 0x0A;
    pub const CONST_U16:  u8 = 0x0B;
    pub const CONST_U32:  u8 = 0x0C;
    pub const CONST_U64:  u8 = 0x0D;
    pub const CONST_I16:  u8 = 0x0E;
    pub const CONST_I32:  u8 = 0x0F;
    pub const DUPE:       u8 = 0x10;
    pub const DUPE_1:     u8 = 0x11;
    pub const DUPE_C:     u8 = 0x12;
    pub const SWAP:       u8 = 0x13;
    pub const SWAP_1:     u8 = 0x14;
    pub const SWAP_C:     u8 = 0x15;
    pub const ROTATE:     u8 = 0x16;
    pub const ROTATE_1:   u8 = 0x17;
    pub const ROTATE_C:   u8 = 0x18;
    pub const ROTATE_1_C: u8 = 0x19;
    pub const POP:        u8 = 0x1A;
    pub const POP_1:      u8 = 0x1B;
    pub const POP_C:      u8 = 0x1C;
    pub const GET_U8:     u8 = 0x1D;
    pub const GET_U8_C:   u8 = 0x1E;
    pub const GET_U16:    u8 = 0x1F;
    pub const GET_U16_C:  u8 = 0x20;
    pub const GET_U32:    u8 = 0x21;
    pub const GET_U32_C:  u8 = 0x22;
    pub const GET_U64:    u8 = 0x23;
    pub const GET_U64_C:  u8 = 0x24;
    pub const GET_I8:     u8 = 0x25;
    pub const GET_I8_C:   u8 = 0x26;
    pub const GET_I16:    u8 = 0x27;
    pub const GET_I16_C:  u8 = 0x28;
    pub const GET_I32:    u8 = 0x29;
    pub const GET_I32_C:  u8 = 0x2A;
    pub const GET_F32:    u8 = 0x2B;
    pub const GET_F32_C:  u8 = 0x2C;
    pub const SET_8:      u8 = 0x2D;
    pub const SET_8_C:    u8 = 0x2E;
    pub const SET_16:     u8 = 0x2F;
    pub const SET_16_C:   u8 = 0x30;
    pub const SET_32:     u8 = 0x31;
    pub const SET_32_C:   u8 = 0x32;
    pub const SET_64:     u8 = 0x33;
    pub const SET_64_C:   u8 = 0x34;
    pub const SET_F32:    u8 = 0x35;
    pub const SET_F32_C:  u8 = 0x36;
    pub const STACK_SIZE: u8 = 0x37;
    pub const PUSH_STACK: u8 = 0x38;
    pub const POP_STACK:  u8 = 0x39;
    pub const RESERVE_C:  u8 = 0x3A;
    pub const RESERVE_N:  u8 = 0x3B;
}

/// Values for decoding the jump instructions
///
/// Jump instructions are not codified directly by constants in this module,
/// instead a series of masks and the values they represent are defined. As
/// such, a jump instruction can be viewed as a bitfield laid out like so:
///
///| Category | T | T | T | T | C | M | S | S |
///|----------|---|---|---|---|---|---|---|---|
///| Bit      | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
///
/// With categories:
/// * `S` -- the source type
/// * `M` -- the mode flag
/// * `C` -- the conditional flag
/// * `T` -- the type of jump conditional
///
/// # Examples
///
/// Absolute unconditional jump from 16-bit const source:
/// ```
/// use tstack::bytecode::{groups, jump};
/// let jmp = (
///     ((groups::JUMP as u16) << 8)
///     | (jump::SRC_C16 as u16)
///     | (jump::MODE_ABSOLUTE as u16)
///     | (jump::CONDITIONAL_FALSE as u16)
/// );
/// ```
///
/// Relative jump using source on the stack if `stack[-1] > 0`:
/// ```
/// use tstack::bytecode::{groups, jump};
/// let jmp = (
///     ((groups::JUMP as u16) << 8)
///     | (jump::SRC_DYN as u16)
///     | (jump::MODE_RELATIVE as u16)
///     | (jump::CONDITIONAL_TRUE as u16)
///     | (jump::TYPE_POS as u16)
/// );
/// ```
pub mod jump {
    /// Mask for the source of a jump
    ///
    /// This masks the source of a jump, which is where the address or offset
    /// comes from. Sources may either be an inline constant (e.g. hardcoded in
    /// the bytecode) of various sizes, or a value on the stack.
    ///
    /// If the mode of the jump is absolute, the source is assumed to be
    /// unsigned. Otherwise if the jump is relative the source is assumed to be
    /// signed.
    pub const SRC_MASK: u8 = 0x03;
    /// The value for a 16-bit constant jump source
    pub const SRC_C16:  u8 = 0x00;
    /// The value for a 32-bit constant jump source
    pub const SRC_C32:  u8 = 0x01;
    /// The value for a 64-bit constant jump source
    pub const SRC_C64:  u8 = 0x02;
    /// The value for a stack value jump source
    pub const SRC_DYN:  u8 = 0x03;

    /// Mask for the mode of a jump
    ///
    /// This masks the mode bit of a jump, which tells the virtual machine how
    /// to interpret the source. If this bit is not set, then the jump will set
    /// the instruction pointer to that value. If the bit is set, then the jump
    /// will add the value to the instruction pointer.
    pub const MODE_MASK:     u8 = 0x04;
    /// The value for a realtive jump
    pub const MODE_RELATIVE: u8 = 0x04;
    /// The value for an absolute jump
    pub const MODE_ABSOLUTE: u8 = 0x00;

    /// Mask for the conditional flag of a jump
    ///
    /// This masks the conditional bit of a jump, which tells the virtual
    /// machine if it should evaluate a condition and only jump if true. If this
    /// bit is set, then the top 4 bits of the jump are the type of the
    /// condition to evaluate. If it is unset, the jump always happens.
    pub const CONDITIONAL_MASK:  u8 = 0x08;
    /// The value of the conditional flag if set
    pub const CONDITIONAL_TRUE:  u8 = 0x08;
    /// The value of the conditional flag if not set
    pub const CONDITIONAL_FALSE: u8 = 0x00;

    /// Mask for the conditional type of the jump
    ///
    /// This masks the conditional type of a jump, which tells the virtual
    /// machine what condition to evaluate when performing a conditional jump.
    /// These bits are ignored if the conditional bit is not set.
    ///
    /// Conditions may evaluate either one or two stack items; this will result
    /// in those items being popped off the stack.
    ///
    /// Conditions assume that they are operating with integer values. To use
    /// floating point values (both f32 and f64), comparison operations from
    /// the floating point math group should be used.
    pub const TYPE_MASK: u8 = 0xF0;
    /// Jump if `stack[-1] == 0`
    pub const TYPE_Z: u8 = 0x00;
    /// Jump if `stack[-1] != 0`
    pub const TYPE_NZ: u8 = 0x10;
    /// Jump if `i64(stack[-1]) > 0`
    pub const TYPE_POS: u8 = 0x20;
    /// Jump if `i64(stack[-1]) < 0`
    pub const TYPE_NEG: u8 = 0x30;
    /// Jump if `i64(stack[-1]) >= 0`
    pub const TYPE_GZ: u8 = 0x40;
    /// Jump if `i64(stack[-1]) <= 0`
    pub const TYPE_LZ: u8 = 0x50;
    /// Jump if `stack[-1] == stack[-2]`
    pub const TYPE_EQ: u8 = 0x60;
    /// Jump if `stack[-1] != stack[-2]`
    pub const TYPE_NEQ: u8 = 0x70;
    /// Jump if `stack[-1] > stack[-2]`
    pub const TYPE_GT: u8 = 0x80;
    /// Jump if `i64(stack[-1]) > i64(stack[-2])`
    pub const TYPE_GTS: u8 = 0x90;
    /// Jump if `stack[-1] < stack[-2]`
    pub const TYPE_LT: u8 = 0xA0;
    /// Jump if `i64(stack[-1]) < i64(stack[-2])`
    pub const TYPE_LTS: u8 = 0xB0;
    /// Jump if `stack[-1] >= stack[-2]`
    pub const TYPE_GE: u8 = 0xC0;
    /// Jump if `i64(stack[-1]) >= i64(stack[-2])`
    pub const TYPE_GES: u8 = 0xD0;
    /// Jump if `stack[-1] <= stack[-2]`
    pub const TYPE_LE: u8 = 0xE0;
    /// Jump if `i64(stack[-1]) <= i64(stack[-2])`
    pub const TYPE_LES: u8 = 0xF0;
}

/// Instructions which perform math on stack operands
///
///| Constant  | ID   | Args    | Stack                | Description
///|-----------|------|---------|----------------------|-------------
///| ADD       |`0x00`|         |`[a,b]  -> [b+a]`     | Add two values on the stack
///| ADD_C     |`0x01`|`c:u16`  |`[a]    -> [a+c]`     | Add a constant to a value on the stack
///| SUB       |`0x02`|         |`[a,b]  -> [b-a]`     | Subtract values on the stack
///| SUB_C     |`0x03`|`c:u16`  |`[a]    -> [a-c]`     | Subtract a constant from a value on the stack
///| MUL       |`0x04`|         |`[a,b]  -> [b*a]`     | Multiply two values on the stack
///| MUL_C     |`0x05`|`c:u16`  |`[a]    -> [a*c]`     | Multiply a value on the stack by a constant
///| DIV       |`0x06`|         |`[a,b]  -> [b/a]`     | Divide two values on the stack
///| DIV_C     |`0x07`|`c:u16`  |`[a]    -> [a/c]`     | Divide a value on the stack by a constant
///| IDIV      |`0x08`|         |`[a,b]  -> [b/a]`     | Divide two signed values on the stack
///| IDIV_C    |`0x09`|`c:i16`  |`[a]    -> [a/c]`     | Divide a signed value on the stack by a signed constant
///| MOD       |`0x0A`|         |`[a,b]  -> [b%a]`     | Get remainder of division of two values on the stack
///| MOD_C     |`0x0B`|`c:u16`  |`[a]    -> [a%c]`     | Get remainder of division of value on the stack by constant
///| IMOD      |`0x0C`|         |`[a,b]  -> [b%a]`     | Get remainder of division of two signed values on the stack
///| IMOD_C    |`0x0D`|`c:i16`  |`[a]    -> [a%c]`     | Get remainder of division of signed value on the stack by a constant
///| DIVMOD    |`0x0E`|         |`[a,b]  -> [b/a,b%a]` | Get quotient and remainder of two values on the stack
///| DIVMOD_C  |`0x0F`|`c:u16`  |`[a]    -> [a/c,a%c]` | Get quotient and remainder of a value on the stack and a constant
///| IDIVMOD   |`0x10`|         |`[a,b]  -> [b/a,b%a]` | Get quotient and remainder of two signed values on the stack
///| IDIVMOD_C |`0x11`|`c:i16`  |`[a]    -> [a/c,a%c]` | Get quotient and remainder of a signed value on the stack and a signed constant
///| FMA       |`0x12`|         |`[a,b,c] -> [(c*b)+a]`| Fused multiply add of three values on the stack
///| FMA_C     |`0x13`|`c:u16`  |`[a,b]  -> [(a*b)+c]` | Fused multiply of two values on the stack and add of a constant
///| POW       |`0x14`|         |`[a,b] -> [b**a]`     | Get `b` raised to the power `a`
///| POW_C     |`0x15`|`c:u16`  |`[a]   -> [a**c]`     | Get `a` raised to the power `c`
///| POW_C_R   |`0x16`|`c:u16`  |`[a]   -> [c**a]`     | Get `c` raised to the power `a`
///| IPOW      |`0x17`|         |`[a,b] -> [b**a]`     | Get `b` raised to the power `a`
///| IPOW_C    |`0x18`|`c:i16`  |`[a]   -> [a**c]`     | Get `a` raised to the power `c`
///| IPOW_C_R  |`0x19`|`c:i16`  |`[a]   -> [c**a]`     | Get `c` raised to the power `c`
///| MAX       |`0x1A`|         |`[a,b]  -> [max(a,b)]`| Get maximum of top two values on the stack
///| MAX_C     |`0x1B`|`c:u16`  |`[a]    -> [max(a,c)]`| Get maximum of top value on stack and constant
///| IMAX      |`0x1C`|         |`[a,b]  -> [max(a,b)]`| Get maximum of top two signed values on the stack
///| IMAX_C    |`0x1D`|`c:i16`  |`[a]    -> [max(a,c)]`| Get maximum of top signed value on the stack and a signed constant
///| MIN       |`0x1E`|         |`[a,b]  -> [min(a,b)]`| Get minimum of top two values on the stack
///| MIN_C     |`0x1F`|`c:u16`  |`[a]    -> [min(a,c)]`| Get minimum of top value on stack and constant
///| IMIN      |`0x20`|         |`[a,b]  -> [min(a,b)]`| Get minimum of top two signed values on the stack
///| IMIN_C    |`0x21`|`c:i16`  |`[a]    -> [min(a,c)]`| Get minimum of top signed value on the stack and a signed constant
///| CLAMP     |`0x22`|         |`[v,u,l] -> [clamp(u,v,l)]`| Clamp value `v` between `u` and `l` inclusive
///| CLAMP_C   |`0x23`|`u,l:u16`|`[v]     -> [clamp(u,v,l)]`| Clamp value `v` between `u` and `l` inclusive
///| ICLAMP    |`0x24`|         |`[v,u,l] -> [clamp(u,v,l)]`| Clamp signed value `v` between `u` and `l` inclusive
///| ICLAMP_C  |`0x25`|`u,l:i16`|`[v]     -> [clamp(u,v,l)]`| Clamp signed value `v` between `u` and `l` inclusive
///| NIMIN_C   |`0xF4`|`c:u16`  |`[a1...a$c]   -> [max(a$c...a1)]`| Maximum of top `$c` signed values on the stack
///| NIMIN     |`0xF5`|         |`[a1...a$n,n] -> [max(a$n...a1)]`| Maximum of top `$n` signed values on the stack
///| NMIN_C    |`0xF6`|`c:u16`  |`[a1...a$c]   -> [max(a$c...a1)]`| Maximum of top `$c` values on the stack
///| NMIN      |`0xF7`|         |`[a1...a$n,n] -> [max(a$n...a1)]`| Maximum of top `$n` values on the stack
///| NIMAX_C   |`0xF8`|`c:u16`  |`[a1...a$c]   -> [max(a$c...a1)]`| Maximum of top `$c` signed values on the stack
///| NIMAX     |`0xF9`|         |`[a1...a$n,n] -> [max(a$n...a1)]`| Maximum of top `$n` signed values on the stack
///| NMAX_C    |`0xFA`|`c:u16`  |`[a1...a$c]   -> [max(a$c...a1)]`| Maximum of top `$c` values on the stack
///| NMAX      |`0xFB`|         |`[a1...a$n,n] -> [max(a$n...a1)]`| Maximum of top `$n` values on the stack
///| DIFF_C    |`0xFC`|`c:u16`  |`[a1...a$c]   -> [a$c-...-a1]`   | Repeated subtraction of top `$c` values on the stack
///| DIFF      |`0xFD`|         |`[a1...a$n,n] -> [a$n-...-a1]`   | Repeated subtraction of top `$n` values on the stack
///| SUM_C     |`0xFE`|`c:u16`  |`[a1...a$c]   -> [a$c+...+a1]`   | Sum top `$c` values on the stack
///| SUM       |`0xFF`|         |`[a1...a$n,n] -> [a$n+...+a1]`   | Sum top `$n` values on the stack
pub mod math {
    pub const ADD:       u8 = 0x00;
    pub const ADD_C:     u8 = 0x01;
    pub const SUB:       u8 = 0x02;
    pub const SUB_C:     u8 = 0x03;
    pub const MUL:       u8 = 0x04;
    pub const MUL_C:     u8 = 0x05;
    pub const DIV:       u8 = 0x06;
    pub const DIV_C:     u8 = 0x07;
    pub const IDIV:      u8 = 0x08;
    pub const IDIV_C:    u8 = 0x09;
    pub const MOD:       u8 = 0x0A;
    pub const MOD_C:     u8 = 0x0B;
    pub const IMOD:      u8 = 0x0C;
    pub const IMOD_C:    u8 = 0x0D;
    pub const DIVMOD:    u8 = 0x0E;
    pub const DIVMOD_C:  u8 = 0x0F;
    pub const IDIVMOD:   u8 = 0x10;
    pub const IDIVMOD_C: u8 = 0x11;
    pub const FMA:       u8 = 0x12;
    pub const FMA_C:     u8 = 0x13;
    pub const POW:       u8 = 0x14;
    pub const POW_C:     u8 = 0x15;
    pub const POW_C_R:   u8 = 0x16;
    pub const IPOW:      u8 = 0x17;
    pub const IPOW_C:    u8 = 0x18;
    pub const IPOW_C_R:  u8 = 0x19;
    pub const MAX:       u8 = 0x1A;
    pub const MAX_C:     u8 = 0x1B;
    pub const IMAX:      u8 = 0x1C;
    pub const IMAX_C:    u8 = 0x1D;
    pub const MIN:       u8 = 0x1E;
    pub const MIN_C:     u8 = 0x1F;
    pub const IMIN:      u8 = 0x20;
    pub const IMIN_C:    u8 = 0x21;
    pub const CLAMP:     u8 = 0x22;
    pub const CLAMP_C:   u8 = 0x23;
    pub const ICLAMP:    u8 = 0x24;
    pub const ICLAMP_C:  u8 = 0x25;
    pub const NMIN_C:    u8 = 0xF4;
    pub const NMIN:      u8 = 0xF5;
    pub const NIMIN_C:   u8 = 0xF6;
    pub const NIMIN:     u8 = 0xF7;
    pub const NMAX_C:    u8 = 0xF8;
    pub const NMAX:      u8 = 0xF9;
    pub const NIMAX_C:   u8 = 0xFA;
    pub const NIMAX:     u8 = 0xFB;
    pub const DIFF_C:    u8 = 0xFC;
    pub const DIFF:      u8 = 0xFD;
    pub const SUM_C:     u8 = 0xFE;
    pub const SUM:       u8 = 0xFF;
}
