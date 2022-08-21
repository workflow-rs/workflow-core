use cfg_if::cfg_if;
extern crate self as workflow_core;

pub mod enums;
pub mod utils;

pub use workflow_core_macros::describe_enum;

cfg_if! {
    if #[cfg(not(target_arch = "bpf"))] {
        /// task re-exports and shims
        pub mod task;
        /// channel re-exports and shims
        pub mod channel {
            pub use async_std::channel::*;
            pub fn oneshot<T>() -> (Sender<T>,Receiver<T>) {
                bounded(1)
            }
        }
        /// bi-directional (req/resp) trigger
        pub mod bitrigger;
        /// trigger re-expor
        pub mod trigger {
            pub use triggered::*;
        }
    }
}
