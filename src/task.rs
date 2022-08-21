#[allow(unused_imports)]
use std::sync::{Arc, Mutex};
use futures::Future;

// pub fn spawn<T>(future: T) -> JoinHandle<T::Output>
// where
//     T: Future + Send + 'static,
//     T::Output: Send + 'static,

#[cfg(not(any(target_arch = "wasm32", target_arch = "bpf")))]
pub mod native {
    // use tokio::task::JoinHandle;

    // pub async fn yield_now() {
    //     tokio::task::yield_now().await
    // }

    pub use super::*;
    pub fn spawn<F, T>(future: F)
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,

    // pub fn spawn<T>(future: T)// -> JoinHandle<T::Output>
    // where
    //     T: Future + Send + 'static,
    //     T::Output: Send + 'static,


    {
        // let _result = async_std::task::spawn(async {
        // let _result = 
        tokio::task::spawn(async {
            future.await
        });
    }
}
#[cfg(not(any(target_arch = "wasm32", target_arch = "bpf")))]
pub use native::*;

// explicitly retain this in native!
pub mod wasm {
    pub use super::*;
    pub fn spawn<F, T>(future: F)
    where
        F: Future<Output = T> + 'static,
        T: 'static,
    {
        // wasm32 spawn shim
        // spawn and spawn_local are currently not available on wasm32 architectures
        // ironically, block_on is but it spawns a task instead of blocking it
        async_std::task::block_on(async move { future.await });
    }
}
#[cfg(target_arch = "wasm32")]
pub use wasm::*;