use std::net::SocketAddr;

use jsonrpsee::{
    core::{async_trait, RpcResult},
    proc_macros::rpc,
    server::Server,
    types::ErrorObject,
};
use runtime::{Dispatchable, RuntimeCall, UpperCall};
pub enum Error {
    DecodeError,
    RuntimeError,
}

impl From<Error> for i32 {
    fn from(e: Error) -> i32 {
        match e {
            Error::RuntimeError => 1,
            Error::DecodeError => 2,
        }
    }
}

// Define a trait for the RPC methods
#[rpc(server, client)]
trait RuntimeAPI {
    #[method(name = "upper_capitalize")]
    async fn upper_capitalize(&self, input: String) -> RpcResult<()> {
        RuntimeCall::BusinessLogic(UpperCall::Capitalize(input.into()))
            .dispatch(String::new())
            .map_err(|err| {
                ErrorObject::owned(
                    Error::RuntimeError.into(),
                    "Unable to query nonce.",
                    Some(err.to_string()),
                )
            })?;
        Ok(())
    }
}

#[derive(Default)]
pub struct RpcServer;

#[async_trait]
impl RuntimeAPIServer for RpcServer {}
impl RpcServer {
    pub async fn serve() -> Result<(tokio::task::JoinHandle<()>, SocketAddr), std::io::Error> {
        let server = Server::builder().build("127.0.0.1:0").await?;

        let addr = server.local_addr()?;
        let handle = server.start(RpcServer::default().into_rpc());

        let task = tokio::spawn(handle.stopped());
        Ok((task, addr))
    }
}
