/// Allows to obtain values in associated types.
pub trait Get<T> {
    fn get() -> T;
}

// Gettable type around usize
pub struct ConstUSize<const T: usize>;
impl<const T: usize> Get<usize> for ConstUSize<T> {
    fn get() -> usize {
        T
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DispatchError {
    RTM {
        rtm_id: &'static str,
        reason: String,
    },
    /// All other errors, with some explanatory string.
    Other(&'static str),
}

impl std::fmt::Display for DispatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::RTM { rtm_id: _, reason } => reason.to_owned(),
            Self::Other(err) => err.to_string(),
        };
        write!(f, "{:?}", msg)
    }
}

impl std::error::Error for DispatchError {}

pub type DispatchResult<T> = Result<T, DispatchError>;

pub trait Dispatchable {
    type Origin;
    type Response;

    fn dispatch(self, origin: Self::Origin) -> DispatchResult<Self::Response>;
}

/// Contains functionality all origins should provide.
// TODO: Implement, as it's just a dummy implementation for now.
pub trait Origin {
    fn is_origin() -> bool {
        true
    }
}

// For now we implement origin on unit.
// TODO: Remove
// impl Origin for () {}

// For now we implement origin on everything.
// TODO: Remove
impl<T> Origin for T {}

pub trait Config {
    type Origin: Origin;
}
