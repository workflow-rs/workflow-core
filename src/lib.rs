//! 
//! [`workflow_core`] is a part of the [`workflow-rs`](https://crates.io/workflow-rs) 
//! framework, subset of which is designed to function uniformally across multiple 
//! environments including native Rust, WASM-browser and Solana OS targets.
//! 
//! [`workflow_core`] provides following modules:
//! - [`id`] - a random 64-bit id struct that offers [`std::string::ToString`] trait 
//! which encodes the id as a base58 string (useful for DOM element ids)
//! - [`enums`] - enum macros for integer to enum conversion as well as an automatic
//! enum to string conversion.
//! - [`trigger`] - re-export of [`triggered`](https://crates.io/crates/triggered) with
//! bi-directional wrappers, helpful for async task even triggering.
//! - [`task`] - custom [`task::spawn`] and [`task::sleep`] function that provide non-blocking
//! spawn of `async` closures. These functions operate uniformally in WASM and in native
//! applications running under [`tokio`](https://crates.io/crates/tokio)
//! - [`async_trait`] - a custom `async_trait` attribute macros for declaring async traits
//! that disable `Send` marker in WASM (allowing use of [`wasm-bindgen::JsValue`](https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html) 
//! values inside of async closures).
//! 



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
