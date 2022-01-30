//! Definitions for modules and symbols
//!
//! The main concepts provided by this module are the Symbol types and the
//! Module type. A symbol in this context is an abstraction of something which
//! can be 'called' via the call bytecode. This includes other bits of bytecode
//! within the same module, bytecode in other modules, or even native code via
//! native code bindings.
//!
//! A Module in this context is therefore a collection of symbols which may
//! be executed in some fashion.

use std::collections::HashMap;

/// A symbol 'local' to the current module
///
/// The `LocalSymbol` type defines a symbol which is local to the Module it is
/// a part of. This means that the module defines this symbol, and the constants
/// and strings any bytecode references are within the same module.
///
/// As the module is understood to be the same one it is defined in, it does not
/// need any definitions for an external module. Additionally, no 'linking' of
/// the symbol needs to be done.
#[derive(Clone)]
pub struct LocalSymbol {
    /// The index within the module string table of the symbol name
    pub name_id: u32,

    /// The offset within the bytecode of this module that this symbol starts
    /// at
    pub code_offset: u32,
}

/// A symbol defined in some other module
///
/// The `ExternalSymbol` type defines a symbol which is present in some other
/// module. This allows bytecode in a module to use the same opcode to call
/// local symbols as well as external (and/or native) symbols. This also keeps
/// all of the linking that the virtual-machine has to do to a singular location
/// instead of needing to scan the bytecode for symbols.
///
/// When defined in a bytecode file, the `module_id` and `symbol_id` are omitted
/// as they can not be known at compile time. Instead, two string constant IDs
/// are provided instead which point to the module and symbol names. These
/// strings are used after all bytecode has been loaded to dynamically look up
/// the appropriate symbol; if a symbol is not defined, the virtual machine will
/// return an error - said error may be ignored, though if the symbol is used by
/// the bytecode upon execution the virtual machine will fault due to an invalid
/// module/symbol ID.
#[derive(Clone)]
pub struct ExternalSymbol {
    /// The index within the module string table of the external module name
    pub module_name_id: u32,
    /// The module ID of the symbol, looked up at runtime
    pub module_id: u32,
    /// The index within the module string table of the symbol name
    pub symbol_name_id: u32,
    /// The symbol ID of the symbol within an external module, looked up at
    /// runtime
    pub symbol_id: u32,
}

/// A collection of symbols and the supporting data for running them
#[derive(Clone)]
pub struct Module {
    /// The name of the module
    pub name: String,
    /// The constant string table used by the module
    pub strings: Vec<String>,
    /// A collection of constant data values
    pub data: Vec<u64>,
    /// The collection of internal symbols that the module defines
    pub local_symbols: Vec<LocalSymbol>,
    /// The collection of external symbols that the module uses
    pub external_symbols: Vec<ExternalSymbol>,
    /// The bytecode as a flat array of u16 opcodes
    pub bytecode: Vec<u16>,

    /// A symbol lookup table, used to link modules together after loading them
    pub symbol_lookup: HashMap<String, u32>,
}
