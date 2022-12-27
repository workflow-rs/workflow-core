//!
//! [`task`](self) module provides helper functions for use with async closures that operate uniformally
//! in native ([`tokio`](https://crates.io/crates/tokio)-backed) and WASM ([`async_std`]-backed) environments.
//! 
//! Following functions are are available:
//! - `spawn(Future)` - non-blocking spawn of the supplied async closure
//! - `sleep(Duration)` - suspends the task for a given Duration 
//! 
//! Please note that blocking spawn is not available as browser-WASM can
//! not block task execution due to a single-threaded async environment.
//! 

#[allow(unused_imports)]
use cfg_if::cfg_if;
use futures::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crate::channel::{oneshot,Sender,Receiver,RecvError,TryRecvError,SendError};
pub use async_std::task::yield_now;

use thiserror::Error;

#[derive(Debug,Error)]
pub enum TaskError {
    #[error("{0:?}")]
    SendError(String),
    #[error("{0:?}")]
    RecvError(#[from] RecvError),
    #[error("{0:?}")]
    TryRecvError(#[from] TryRecvError),
}

impl<T> From<SendError<T>> for TaskError {
    fn from(err: SendError<T>) -> Self {
        TaskError::SendError(err.to_string())
    }
}

pub type TaskResult<T> = std::result::Result<T, TaskError>;


cfg_if! {
    if #[cfg(not(any(target_arch = "wasm32", target_os = "solana")))] {

        pub mod native {
            //! native implementation
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
    //! WASM implementation
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

            /// Suspends current task for the given [`Duration`]
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


#[derive(Clone)]
pub struct Task<F,T,P> 
where
    F: Future<Output = T> + Send + 'static,
    T: 'static,
    P: Fn(Arc<Box<Receiver<()>>>)->F + Send + Sync + 'static,
{
    termination : (Sender<()>, Receiver<()>),
    completion : (Sender<()>, Receiver<()>),
    running : Arc<AtomicBool>,
    task_fn: P
}

impl<F,T,P> Task<F,T,P>
where
    F: Future<Output = T> + Send + 'static,
    T: 'static,
    P: Fn(Arc<Box<Receiver<()>>>)->F + Send + Sync + 'static
{

    pub fn new(task_fn: P) -> Task<F,T,P> {
        let terminate = oneshot();
        let completion = oneshot();

        Task {
            termination: terminate,
            completion,
            running : Arc::new(AtomicBool::new(false)),
            task_fn : task_fn,
        }
    }

    pub async fn run(self : &Arc<Self>) -> TaskResult<()> {
        while self.completion.1.len() > 0 {
            self.completion.1.try_recv()?;
        }

        while self.termination.0.len() > 0 {
            self.termination.1.try_recv()?;
        }

        let this = self.clone();
        workflow_core::task::spawn(async move {
        // async_std::task::spawn(async move {
            this.running.store(true, Ordering::SeqCst);
            (this.task_fn)(Arc::new(Box::new(this.termination.1.clone()))).await;
            this.running.store(false, Ordering::SeqCst);
            this.completion.0.send(()).await.expect("Error signaling task completion");
        });

        Ok(())
    }

    pub async fn terminate(&self) -> TaskResult<()> {
        if self.running.load(Ordering::SeqCst) {
            self.termination.0.send(()).await?;
        }
        Ok(())
    }

    pub async fn wait(&self) -> TaskResult<()> {
        if self.running.load(Ordering::SeqCst) {
            self.completion.1.recv().await?;
        }
        Ok(())
    }

    pub async fn terminate_and_wait(&self) -> TaskResult<()> {
        self.terminate().await?;
        self.wait().await?;
        Ok(())
    }
}
