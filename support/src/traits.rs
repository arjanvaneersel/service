pub trait Get<T> {
    fn get() -> T;
}

pub trait Responder {
    fn is_responder() -> bool {
        true
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DispatchError {
    RTM {
        rtm_id: &'static str,
        reason: String,
    },
    /// All other errors, with some explanatory string.
    Other(&'static str),
}

pub type DispatchResult = Result<(), DispatchError>;

pub trait Dispatchable {
    type Origin;

    fn dispatch(self, origin: Self::Origin) -> DispatchResult;
}
