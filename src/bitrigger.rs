
use triggered::{Trigger,Listener};

/// Wrapper containing a single Trigger instance
#[derive(Debug, Clone)]
pub struct TriggerInstance {
    pub trigger : Trigger,
    pub listener : Listener,
}

impl TriggerInstance {
    fn new() -> TriggerInstance {
        let (trigger,listener) = triggered::trigger();
        TriggerInstance { 
            trigger,
            listener,
        }
    }
}

/// Bi-directional trigger meant to function in
/// request/response fashion
#[derive(Debug, Clone)]
pub struct BiTrigger {
    pub request : TriggerInstance,
    pub response : TriggerInstance,
}

impl BiTrigger {
    pub fn new() -> BiTrigger {
        BiTrigger {
            request : TriggerInstance::new(),
            response : TriggerInstance::new(),
        }
    }
}
