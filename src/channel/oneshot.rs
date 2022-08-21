
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

pub fn channel<T : Unpin>() -> (Sender<T>,Receiver<T>) {
    let (future, completer) = ManualFuture::<T>::new();

    let receiver = Receiver::new(future);
    let sender = Sender::new(completer);

    (sender,receiver)
}

#[derive(Clone)]
pub struct Sender<T> where T: Unpin {
    completer : Arc<Mutex<Option<Completer<T>>>>,
}

impl<T> Sender<T> where T: Unpin {
    pub fn new(completer: Completer<T>) -> Self {
        Sender {
            completer : Arc::new(Mutex::new(Some(completer)))
        }
    }
    pub fn try_take(&self) -> Result<Completer<T>> {
        self.completer.lock().unwrap().take().ok_or(Error::CompleterIsTaken)
    }

    pub fn take(&self) -> Completer<T> {
        self.completer.lock().unwrap().take().unwrap()
    }

    pub fn is_not_taken(&self) -> bool {
        self.completer.lock().unwrap().is_some()
    }

    pub fn is_taken(&self) -> bool {
        self.completer.lock().unwrap().is_none()
    }

    pub async fn try_send(&self, v : T) -> Result<()> {
        self.try_take()?.complete(v).await;
        Ok(())
    }

    pub async fn send(&self, v : T) {
        self.take().complete(v).await
    }
}

#[derive(Clone)]
pub struct Receiver<T> where T: Unpin {
    future : Arc<Mutex<Option<Future<T>>>>,
}

impl<T> Receiver<T> where T: Unpin {
    pub fn new(future: Future<T>) -> Self {
        Receiver {
            future : Arc::new(Mutex::new(Some(future)))
        }
    }
    pub fn try_take(&self) -> Result<Future<T>> {
        self.future.lock().unwrap().take().ok_or(Error::FutureIsTaken)
    }

    pub fn take(&self) -> Future<T> {
        self.future.lock().unwrap().take().unwrap()
    }

    pub async fn try_recv(&self) -> Result<T> {
        Ok(self.try_take()?.await)
    }

    pub async fn recv(&self) -> T {
        self.take().await
    }
}

