pub use rtm_greeter::{Call as GreeterCall, CallResponse as GreeterResponse};
use support::traits::DispatchResult;
pub use support::traits::Dispatchable;

#[derive(Debug, PartialEq)]
pub struct Runtime;

impl rtm_greeter::Config for Runtime {
    const RTM_ID: &'static str = "RTM_UPPER";

    type Origin = String;
}

#[derive(Debug, PartialEq)]
pub enum RuntimeCall {
    Greeter(GreeterCall<Runtime>),
}

#[derive(Debug, PartialEq)]
pub enum RuntimeResponse {
    Greeter(GreeterResponse<Runtime>),
}

impl Dispatchable for RuntimeCall {
    type Origin = <Runtime as rtm_greeter::Config>::Origin;
    type Response = RuntimeResponse;

    fn dispatch(self, origin: Self::Origin) -> DispatchResult<Self::Response> {
        match self {
            RuntimeCall::Greeter(call) => Ok(RuntimeResponse::Greeter(call.dispatch(origin)?)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = RuntimeCall::Greeter(rtm_greeter::Call::Greet("Luna".into()))
            .dispatch(String::new())
            .unwrap();
        assert_eq!(
            result,
            RuntimeResponse::Greeter(GreeterResponse::<Runtime>::Greet(String::from(
                "Hello, Luna!"
            )))
        );
    }
}
