use tstack;

use std::collections::HashMap;
use std::rc::Rc;

use tstack::module::{LocalSymbol, Module};

fn main() {
    let mut engine = tstack::Engine::new();
    let r = engine.add_module(Rc::new(Module {
        name: String::from("main"),
        strings: vec![String::from("main")],
        data: vec![],
        local_symbols: vec![LocalSymbol { name_id: 0, code_offset: 0 }],
        external_symbols: vec![],
        bytecode: vec![
            tstack::inst_stack!(CONST_N1),
            tstack::inst_stack!(CONST_U16),
            0x0015,
            tstack::inst_sys!(PRINT_STACK),
            tstack::inst_math!(ADD),
            tstack::inst_sys!(PRINT_I64),
        ],
        symbol_lookup: HashMap::new(),
    }));
    if let Err(e) = r {
        println!("Error adding module: {}", e);
    }
    if let Err(e) = engine.run(0, 0) {
        println!("Error: {}", e);
    }
}
