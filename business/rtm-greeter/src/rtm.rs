use support::traits::{DispatchError, DispatchResult, Dispatchable};

pub trait Config {
    /// The identifier of this element.
    const RTM_ID: &'static str;

    type Origin: Clone;
}

pub trait Greeter<T: Config> {
    fn greet(_origin: T::Origin, _name: String) -> DispatchResult<CallResponse<T>>;
}

#[derive(Debug, PartialEq)]
pub enum Call<T: Config> {
    Greet(String),
    #[allow(non_camel_case_types)]
    __marker(std::marker::PhantomData<T>),
}

#[derive(Debug, PartialEq)]
pub enum CallResponse<T: Config> {
    Greet(String),
    #[allow(non_camel_case_types)]
    __marker(std::marker::PhantomData<T>),
}

#[allow(non_camel_case_types)]
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
    fn greet(_origin: T::Origin, name: String) -> DispatchResult<CallResponse<T>> {
        let answer = format!("Hello, {}!", name);
        Ok(CallResponse::Greet(answer))
    }
}

impl<T: Config> Dispatchable for Call<T> {
    type Origin = T::Origin;
    type Response = CallResponse<T>;

    fn dispatch(self, origin: T::Origin) -> DispatchResult<Self::Response> {
        match self {
            Call::Greet(input) => RTM::<T>::greet(origin, input),
            Call::__marker(_) => unreachable!("__marker should never be printed"),
        }
    }
}

impl<T: Config> Greeter<T> for RTM<T> {
    fn greet(origin: T::Origin, name: String) -> DispatchResult<CallResponse<T>> {
        RTM::<T>::greet(origin, name)
    }
}
