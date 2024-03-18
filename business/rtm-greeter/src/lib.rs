//! Greeter runtime module
//! 
//! A runtime module (RTM) contains business logic for a certain abstraction of tasks.
//! Runtime modules are plugged into the Runtime to enable functionality for the runtime of this service.

pub mod config;
pub mod calls;
pub mod error;
pub mod rtm;
pub mod traits;

pub use calls::{GreetCall, GreetResponse};
pub use config::Config;