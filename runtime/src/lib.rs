pub use rtm_greeter::{GreetCall, GreetResponse};
use support::traits::ConstUSize;
pub use support::traits::Dispatchable;

#[derive(Debug, PartialEq)]
/// Composite element for runtime modules.
pub struct Runtime;

// Every Config of runtime modules use support::Config as its base implementation,
// therefore Runtime should implement it, as it will be used as the value for generics
// of calls.
impl support::Config for Runtime {
    type Origin = ();
}

// Implement the Config trait of each runtime module that will be enabled for the runtime.

// Enable rtm_greeter by implementing the Config trait.
impl rtm_greeter::Config for Runtime {
    const RTM_ID: &'static str = "RTM_UPPER";

    type Times = ConstUSize<3>;
}

/// Alias for easier composing of GreetCalls.
pub type GreeterGreetCall = GreetCall<Runtime>;

/// Alias for easier compising of GreetResponse.
pub type GreeterGreetResponse = GreetResponse<Runtime>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_works() {
        let result = GreeterGreetCall::new("Luna".into()).dispatch(()).unwrap();
        assert_eq!(
            result,
            GreeterGreetResponse::new("Hello, hello, hello, Luna!".into())
        );
    }
}
