//! Definitions for modules and symbols

use std::collections::HashMap;

#[derive(Clone)]
pub struct LocalSymbol {
    pub name_id: u32,
    pub code_offset: u32,
}

#[derive(Clone)]
pub struct ExternalSymbol {
    pub module_name_id: u32,
    pub module_id: u32,
    pub symbol_name_id: u32,
    pub symbol_id: u32,
}

#[derive(Clone)]
pub struct Module {
    pub name: String,
    pub strings: Vec<String>,
    // TODO: pub data: Vec<u16>?
    pub local_symbols: Vec<LocalSymbol>,
    pub external_symbols: Vec<ExternalSymbol>,
    pub bytecode: Vec<u16>,

    pub symbol_lookup: HashMap<String, u32>,
}
