//! Oxidate - FSM Framework with GUI Visualization
//! A Mermaid-like DSL to Rust code generator for Finite State Machines

pub mod fsm;
pub mod parser;
pub mod codegen;

pub use fsm::*;
pub use parser::parse_fsm;
pub use codegen::generate_rust_code;
