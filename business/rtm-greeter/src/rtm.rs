use support::traits::{DispatchResult, Dispatchable, Get};
use crate::{config::Config, traits::Greeter, GreetCall, GreetResponse};

fn capitalize(input: String) -> String {
    let first_char = input.chars().next().unwrap().to_uppercase().to_string();

    // Concatenate the uppercase first character with the rest of the string
    let mut capitalized_string = first_char;
    capitalized_string.push_str(&input[1..]);
    capitalized_string
}

// Just a helper function to contruct the response.
fn format_greeting(name: String, greeting: String, number: usize) -> String {
    // Create a repeated greeting using the specified number
    let repeated_greeting = format!("{}, ", greeting).repeat(number);

    // Combine the repeated greeting with the capitalized name
    format!("{}{}!", capitalize(repeated_greeting), capitalize(name))
}

/// Abstraction that holds the business logic for this runtime module.
pub struct RTM<T: Config>(std::marker::PhantomData<T>);
impl<T: Config> RTM<T> {
    // Implement business logic here as private functions.

    /// Returns a greeting for the provided name.
    /// 
    /// RTM functions are reached via dispatched calls, therefore returning a DispatchResult
    /// makes it easy to prolongate results up the call chain.
    fn greet(_origin: T::Origin, name: String) -> DispatchResult<GreetResponse<T>> {
        let answer = format_greeting(name, "hello".into(), T::Times::get());
        Ok(GreetResponse::new(answer))
    }
}

// The Greeter trait is an adaptor to make functionality available from the outside.
// This is needed to allow interaction between runtime modules without exposing all functionality from RTM,
// but only those that have to be exposed. For this reason all functions of RTM are private.
impl<T:Config> Greeter<T> for RTM<T> {
    fn greet(origin: T::Origin, name: String) -> DispatchResult<GreetResponse<T>> {
        RTM::<T>::greet(origin, name)
    }
}

/****************************************************************************/
/* Call dispatching                                                         */
/****************************************************************************/
// Implementing the Dispatchable trait for all calls is done here instead   // 
// of calls.rs, because we need to access the private functions from RTM.   // 
// That wouldn´t be possible from traits.rs.                                //
/****************************************************************************/

// Implementing Dispatchable makes GreetCall dispatchable. Dispatching is a 
// convenient way to move calls between the layers of the service.
//
// We will simply do: `GreetCall::new(String::from("Arjan")).dispatch(origin);`
// to make the request.
impl<T: Config> Dispatchable for GreetCall<T> {
    /// Type for call origins.
    type Origin = T::Origin;

    /// Type for call responses.
    type Response = GreetResponse<T>;

    fn dispatch(self, origin: Self::Origin) -> DispatchResult<Self::Response> {
        // Execute the business logic.
        RTM::<T>::greet(origin, self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_greeting_works() {
        let result = format_greeting("Luna".into(), "hello".into(), 3);
        assert_eq!(
            result,
            "Hello, hello, hello, Luna!".to_string()
        );
    }
}