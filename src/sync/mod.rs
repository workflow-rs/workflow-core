pub mod oneshot;
pub mod reqresp;
pub mod task;

use std::marker::Unpin;

pub fn oneshot<T : Unpin>() -> oneshot::Oneshot<T> {
    oneshot::Oneshot::<T>::new()
}

pub fn reqresp<Req : Unpin, Resp : Unpin>() -> reqresp::ReqResp<Req, Resp> {
    reqresp::ReqResp::new()
}

