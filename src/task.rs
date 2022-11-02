#[allow(unused_imports)]
use cfg_if::cfg_if;
use futures::Future;

pub use async_std::task::yield_now;

cfg_if! {
    if #[cfg(not(any(target_arch = "wasm32", target_os = "solana")))] {

        pub mod native {
            
            pub use super::*;
            pub fn spawn<F, T>(future: F)
            where
            F: Future<Output = T> + Send + 'static,
            T: Send + 'static,
            {
                tokio::task::spawn(async {
                // async_std::task::spawn(async {
                    future.await
                });
            }
        
            pub use async_std::task::sleep;
        }

        pub use native::*;
    }
}


// we explicitly must retain this in native
// to allow access to wasm::spawn from any target
// (this is needed for workflow-ux)
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
    
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            use std::sync::{Arc, Mutex};
            use wasm_bindgen::prelude::*;
            use instant::Duration;

            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen (catch, js_name = setTimeout)]
                pub fn set_timeout(closure: &Closure<dyn FnMut()>, timeout: u32) -> std::result::Result<u32, JsValue>;
                #[wasm_bindgen (catch, js_name = clearTimeout)]
                pub fn clear_timeout(interval: u32) -> std::result::Result<(), JsValue>;
            }

            pub async fn sleep(duration : Duration) {
                let (sender, receiver) = crate::channel::oneshot::<()>();
                let interval = {
                    let mutex_init : Arc<Mutex<Option<Closure<dyn FnMut()>>>> = Arc::new(Mutex::new(None));
                    let mutex_clear = mutex_init.clone();
                    let closure = Closure::new(move ||{
                        sender.try_send(()).unwrap();
                        *mutex_clear.clone().lock().unwrap() = None;
                    });
                    let interval = set_timeout(&closure, duration.as_millis() as u32).unwrap();
                    *mutex_init.lock().unwrap() = Some(closure);
                    interval
                };
                receiver.recv().await.unwrap();
                clear_timeout(interval).unwrap();
            
            } 
        }
    }   
}
#[cfg(target_arch = "wasm32")]
pub use wasm::*;