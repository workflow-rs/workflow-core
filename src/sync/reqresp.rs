use crate::sync::oneshot::Oneshot;

#[derive(Clone)]
pub struct ReqResp<Req = (),Resp = ()> 
where
    Req: Unpin,
    Resp: Unpin,
{
    pub request : Oneshot<Req>,
    pub response : Oneshot<Resp>,
}

impl<Req, Resp> ReqResp<Req, Resp>
where
    Req: Unpin,
    Resp: Unpin,
{
    pub fn new() -> Self {

        let request = Oneshot::new();
        let response = Oneshot::new();
        
        Self {
            request,
            response,
        }
    }   
}
