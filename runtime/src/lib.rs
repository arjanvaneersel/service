pub use rtm_upper::Call as UpperCall;
use support::traits::DispatchResult;
pub use support::traits::Dispatchable;

pub struct Runtime;

impl rtm_upper::Config for Runtime {
    const RTM_ID: &'static str = "RTM_UPPER";

    type Origin = String;
    type Input = String;
}

pub enum RuntimeCall {
    BusinessLogic(rtm_upper::Call<Runtime>),
}

impl Dispatchable for RuntimeCall {
    type Origin = <Runtime as rtm_upper::Config>::Origin;

    fn dispatch(self, origin: Self::Origin) -> DispatchResult {
        match self {
            RuntimeCall::BusinessLogic(element) => element.dispatch(origin),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = RuntimeCall::BusinessLogic(rtm_upper::Call::Capitalize("hello".into()))
            .dispatch(String::new());
        assert!(result.is_ok());
    }
}
