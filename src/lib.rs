extern crate self as workflow_core;

pub mod enums;
pub mod utils;

#[cfg(not(target_arch = "bpf"))]
pub mod task;

#[cfg(not(target_arch = "bpf"))]
pub mod channel;

#[cfg(not(target_arch = "bpf"))]
pub mod bitrigger;

pub use workflow_core_macros::describe_enum;
