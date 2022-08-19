use std::sync::Arc;
use std::sync::Mutex;
use std::marker::Unpin;
use manual_future::{
    ManualFuture, 
    ManualFutureCompleter
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    FutureIsTaken,
    CompleterIsTaken,
}

type Result<T> = std::result::Result<T, Error>;

pub type Future<T> = ManualFuture<T>;
pub type Completer<T> = ManualFutureCompleter<T>;

#[derive(Clone)]
pub struct Oneshot<T = ()> where T: Unpin {
    future : Arc<Mutex<Option<Future<T>>>>,
    completer : Arc<Mutex<Option<Completer<T>>>>,
}

impl<T> Oneshot<T> where T: Unpin {
    pub fn new() -> Self {
        let (future, completer) = Future::<T>::new();

        Self {
            future: Arc::new(Mutex::new(Some(future))),
            completer: Arc::new(Mutex::new(Some(completer))),
        }
    }

    pub fn try_take_future(&self) -> Result<Future<T>> {
        self.future.lock().unwrap().take().ok_or(Error::FutureIsTaken)
    }
    
    pub fn take_future(&self) -> Future<T> {
        self.future.lock().unwrap().take().unwrap()
    }

    pub fn try_take_completer(&self) -> Result<Completer<T>> {
        self.completer.lock().unwrap().take().ok_or(Error::CompleterIsTaken)
    }
    
    pub fn take_completer(&self) -> Completer<T> {
        self.completer.lock().unwrap().take().unwrap()
    }

    pub async fn try_send(&self, v : T) -> Result<()> {
        self.try_take_completer()?.complete(v).await;
        Ok(())
    }

    pub async fn send(&self, v : T) {
        self.take_completer().complete(v).await
    }

    pub async fn try_recv(&self) -> Result<T> {
        Ok(self.try_take_future()?.await)
    }

    pub async fn recv(&self) -> T {
        self.take_future().await
    }
}
