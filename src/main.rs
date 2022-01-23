
use tstack;

fn main() {
    let code: &[u16] = &[
        tstack::inst_stack!(CONST_N1),
        tstack::inst_stack!(CONST_U16), 0x0015,
        tstack::inst_sys!(PRINT_STACK),
        tstack::inst_math!(ADD),
        tstack::inst_sys!(PRINT_I64),
    ];
    let mut engine = tstack::Engine::new();
    let result = engine.run(&code);
    if let Err(e) = result {
        println!("Error: {}", e);
    }
}
