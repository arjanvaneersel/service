use jsonrpsee::{
    core::{async_trait, RpcResult},
    proc_macros::rpc,
    server::Server,
    types::ErrorObject,
};
use runtime::{Dispatchable, GreeterGreetCall};
use std::sync::Arc;
use tokio::{
    sync::{oneshot, Mutex},
    task::JoinHandle,
};

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
        GreeterGreetCall::new(input)
            .dispatch(())
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

pub struct RpcServer {
    shutdown_tx: oneshot::Sender<()>,
    shutdown_rx: Arc<Mutex<Option<oneshot::Receiver<()>>>>,
}

impl Default for RpcServer {
    fn default() -> Self {
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        Self {
            shutdown_tx,
            shutdown_rx: Arc::new(Mutex::new(Some(shutdown_rx))),
        }
    }
}

#[async_trait]
impl RuntimeAPIServer for RpcServer {}
impl RpcServer {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn serve(&self) -> Result<JoinHandle<()>, std::io::Error> {
        let server = Server::builder().build("127.0.0.1:3000").await?;

        // let addr = server.local_addr()?;
        let handle = server.start(RpcServer::default().into_rpc());

        let shutdown_rx = self.shutdown_rx.lock().await.take().unwrap();

        let task = tokio::spawn(async move {
            tokio::select! {
                _ = handle.clone().stopped() => {
                    println!("Server stopped.");
                }
                _ = shutdown_rx => {
                    println!("Received shutdown signal. Stopping server...");
                    handle.stop().unwrap();
                }
            }
        });

        Ok(task)
    }

    pub async fn shutdown(self) {
        self.shutdown_tx.send(()).unwrap();
    }
}
