//! data structures

// To enable `#[allow(clippy::all)]`
#![allow(non_camel_case_types,  non_snake_case,  non_upper_case_globals)]
#![feature(tool_lints)]

/// Generated Message module from Flatbuffers compiler
include!(concat!(env!("OUT_DIR"),  "/protocol_generated.rs"));
