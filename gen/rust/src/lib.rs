//! Generated Rust types for the Kyanite common schema.
//!
//! The contents of `gen/` are produced by `buf generate` via the
//! `buf.build/anthropics/buffa` remote plugin (`file_per_package=true`). This
//! module tree is the only hand-written file in the crate; it wires each
//! generated `<package>.rs` into the matching module path.
//!
//! `no_std`: the only consumer is the AST2700 firmware. buffa provides the
//! `alloc`-backed types (`::buffa::alloc::…`); the firmware binary supplies the
//! global allocator.
#![no_std]

pub mod schema {
    pub mod v1 {
        include!("gen/schema.v1.rs");
    }
}
