
macro_rules! checkstack {
    ($engine:expr, $opcode:expr, $name:literal, $count:expr) => {
        if $engine.maxstack - $engine.stack.len() < $count {
            return Err(BytecodeError::stack_overflow($opcode, $name));
        }
    }
}

macro_rules! pushstack {
    ($engine:expr, $opcode:expr, $name:literal, $value:expr) => {
        pushstack!(@n $engine, $opcode, $name, 1, $value)
    };
    ($engine:expr, $opcode:expr, $name:literal, $v1:expr, $v2:expr) => {
        pushstack!(@n $engine, $opcode, $name, 2, $v1, $v2)
    };
    (@n $engine:expr, $opcode:expr, $name:literal, $count:literal, $($value:expr),*) => {{
        checkstack!($engine, $opcode, $name, $count);
        $(
            $engine.stack.push($value as u64);
        )*
    }}
}

macro_rules! popsingle {
    ($engine:expr, $opcode:expr, $name:literal, $count:literal) => {{
        let value: u64;
        let res = $engine.stack.pop();
        match res {
            None => {
                return Err(BytecodeError::stack_underflow($opcode, $name, $count));
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
            return Err(BytecodeError::code_data($code[$n], $name, 2));
        }
        $code[$n+1]
    }}
}

macro_rules! cval_u16_2 {
    ($code:expr, $n:expr, $name:literal) => {{
        if $code.len() - $n < 2 {
            return Err(BytecodeError::code_data($code[$n], $name, 4));
        }
        ($code[$n+1], $code[$n+2])
    }}
}

macro_rules! cval_u16_4 {
    ($code:expr, $n:expr, $name:literal) => {{
        if $code.len() - $n < 4 {
            return Err(BytecodeError::code_data($code[$n], $name, 8));
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
