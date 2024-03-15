use support::traits::{DispatchError, DispatchResult, Dispatchable};

use crate::to_upper::ToUpper;

pub trait Config {
    /// The identifier of this element.
    const RTM_ID: &'static str;

    type Origin: Clone;
    type Input: Clone + ToUpper;
}

pub trait BusinessLogic {
    type Origin;
    type Input;

    fn capitalize(origin: Self::Origin, input: Self::Input) -> DispatchResult;
}

pub enum Call<T: Config> {
    Capitalize(T::Input),
}

pub enum Error<T: Config> {
    Unknown,
    #[allow(non_camel_case_types)]
    __marker(std::marker::PhantomData<T>),
}

impl<T: Config> std::fmt::Debug for Error<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unknown => write!(f, "Unknown"),
            Error::__marker(_) => unreachable!("__marker should never be printed"),
        }
    }
}

impl<T: Config> std::fmt::Display for Error<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T: Config> std::error::Error for Error<T> {}

impl<T: Config> From<Error<T>> for DispatchError {
    fn from(value: Error<T>) -> Self {
        DispatchError::RTM {
            rtm_id: T::RTM_ID,
            reason: format!("{:?}", value),
        }
    }
}

pub struct RTM<T: Config>(std::marker::PhantomData<T>);
impl<T: Config> RTM<T> {
    fn capitalize(_origin: T::Origin, input: T::Input) -> DispatchResult {
        println!("{}", input.as_uppercase());
        Ok(())
    }
}

impl<T: Config> Dispatchable for Call<T> {
    type Origin = T::Origin;

    fn dispatch(self, origin: T::Origin) -> DispatchResult {
        match self {
            Call::Capitalize(input) => RTM::<T>::capitalize(origin, input),
        }
    }
}

impl<T: Config> BusinessLogic for RTM<T> {
    type Origin = T::Origin;
    type Input = T::Input;

    fn capitalize(origin: Self::Origin, input: Self::Input) -> DispatchResult {
        RTM::<T>::capitalize(origin, input)
    }
}
