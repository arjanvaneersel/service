use crate::Config;

#[derive(Debug, PartialEq)]
/// The response to a GreetCall.
/// 
/// No types from T are used, but T is provided to keep a consistent API.
pub struct GreetResponse<T>(String, std::marker::PhantomData<T>);
impl<T: Config> GreetResponse<T> {

    /// Constructor for easy instantiation.
    pub fn new(input: String) -> GreetResponse<T> {
        GreetResponse(input, std::marker::PhantomData)
    }
}

#[derive(Debug, PartialEq)]
/// Holds all parameters for greet calls.
/// 
/// In this case that's just a String.
/// No types from T are used, but T is provided to keep a consistent API.
pub struct GreetCall<T:Config>(String, std::marker::PhantomData<T>);
impl<T:Config> GreetCall<T> {

    /// Constructor for easy instantiation.
    pub fn new(input: String) -> GreetCall<T> {
        GreetCall(input, std::marker::PhantomData)
    }

    /// Getter for the value.
    /// 
    /// The idea is to keep parameters private and expose values through getters
    /// when parts outside of this module need to read them. Since this type contains just one
    /// value, the getter is simply called value.
    pub fn value(self) -> String {
        self.0
    }
}