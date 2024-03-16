pub use rtm_greeter::{GreetCall, GreetResponse};
pub use support::traits::Dispatchable;

#[derive(Debug, PartialEq)]
pub struct Runtime;

impl rtm_greeter::Config for Runtime {
    const RTM_ID: &'static str = "RTM_UPPER";

    type Origin = String;
}

pub type GreeterGreetCall = GreetCall<Runtime>;
pub type GreeterGreetResponse = GreetResponse<Runtime>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = GreeterGreetCall::new("Luna".into()).dispatch(String::new()).unwrap();
        assert_eq!(
            result,
            GreeterGreetResponse::new("Hello Luna!".into())
        );
    }
}
