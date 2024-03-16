use support::traits::DispatchError;

use crate::Config;

#[allow(non_camel_case_types)]
/// Holds all errors that this runtime module could generate.
pub enum Error<T: Config> {
    /// An unknown error occured.
    Unknown,
    #[allow(non_camel_case_types)]
    /// The marker is used, because we're not using any types from T here.
    /// However later on we will need T, therefore we bound it this way.
    __marker(std::marker::PhantomData<T>),
}

// We implement the debug trait to provide a human readable description of the error.
impl<T: Config> std::fmt::Debug for Error<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unknown => write!(f, "Unknown"),
            Error::__marker(_) => unreachable!("__marker should never be printed"),
        }
    }
}

// We implement the Display trait so that errors can be printed too.
// The implementation simply prints the debug value.
impl<T: Config> std::fmt::Display for Error<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Any error should implement the Error trait, which is what we do here.
impl<T: Config> std::error::Error for Error<T> {}

// The from trait is implemented to easily convert our error to a DispatchError,
// this makes prolongating of errors possible.
impl<T: Config> From<Error<T>> for DispatchError {
    fn from(value: Error<T>) -> Self {
        DispatchError::RTM {
            rtm_id: T::RTM_ID,
            reason: format!("{:?}", value),
        }
    }
}