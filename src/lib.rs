extern crate self as workflow_core;

pub mod enums;
pub mod utils;

#[cfg(not(target_arch = "bpf"))]
pub mod sync;

pub use workflow_core_macros::describe_enum;
