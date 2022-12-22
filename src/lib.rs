use cfg_if::cfg_if;
extern crate self as workflow_core;

pub mod enums;
pub mod utils;
// pub mod time;

pub use workflow_core_macros::describe_enum;

cfg_if! {
    if #[cfg(not(target_os = "solana"))] {
        /// Generic 8-byte identifier
        pub mod id;
        /// task re-exports and shims
        pub mod task;
        /// [`async_std::channel`] re-exports and shims
        pub mod channel {
            pub use async_std::channel::*;
            /// Creates a oneshot channel (bounded channel with a limit of 1 message)
            pub fn oneshot<T>() -> (Sender<T>,Receiver<T>) {
                bounded(1)
            }
        }
        /// re-exports triggered crate as well as
        /// two wrappers SingleTrigger and ReqRespTrigger
        pub mod trigger;

        pub mod time {
            pub use instant::*;
        }

        #[cfg(target_arch = "wasm32")]
        pub use workflow_async_trait::async_trait_without_send as workflow_async_trait;

        #[cfg(not(target_arch = "wasm32"))]
        pub use workflow_async_trait::async_trait_with_send as workflow_async_trait;

        pub use workflow_async_trait::{
            async_trait,
            async_trait_with_send,
            async_trait_without_send
        };
    }
}
