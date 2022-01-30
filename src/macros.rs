macro_rules! checkstack {
    ($engine:expr, $opcode:expr, $count:expr) => {
        if (($engine.maxstack - $engine.stack.len()) as u64) < $count {
            return Err(BytecodeError::stack_overflow($opcode));
        }
    };
}

macro_rules! pushstack {
    ($engine:expr, $opcode:expr, $value:expr) => {
        pushstack!(@n $engine, $opcode, 1, $value)
    };
    ($engine:expr, $opcode:expr, $v1:expr, $v2:expr) => {
        pushstack!(@n $engine, $opcode, 2, $v1, $v2)
    };
    (@n $engine:expr, $opcode:expr, $count:literal, $($value:expr),*) => {{
        checkstack!($engine, $opcode, $count);
        $(
            $engine.stack.push($value as u64);
        )*
    }}
}

macro_rules! popsingle {
    ($engine:expr, $opcode:expr, $count:literal) => {{
        let value: u64;
        let res = $engine.stack.pop();
        match res {
            None => {
                return Err(BytecodeError::stack_underflow($opcode, $count));
            }
            Some(v) => value = v,
        }
        value
    }};
}

macro_rules! popstack1 {
    ($engine:expr, $opcode:expr) => {{
        popsingle!($engine, $opcode, 1)
    }};
}

macro_rules! popstack2 {
    ($engine:expr, $opcode:expr) => {{
        let v1 = popsingle!($engine, $opcode, 2);
        let v2 = popsingle!($engine, $opcode, 2);
        (v1, v2)
    }};
}
