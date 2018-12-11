// To enable `#[allow(clippy::all)]`
//#![feature(tool_lints)]

#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate serde_derive;

/// Module containing functions to generate Witnet's protocol messages
pub mod builders;

/// Module containing Witnet's chain data types
pub mod chain;

/// Module generated by flatbuffers compiler, containing flatbuffers protocol messages types
pub mod flatbuffers;

/// Module containing functions to cast Witnet's protocol messages to flatbuffers and vice versa
pub mod serializers;

/// Module containing Witnet's protocol messages types
pub mod types;

/// Module containing error definitions
pub mod error;
