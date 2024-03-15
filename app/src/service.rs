pub use tokio::signal;

#[tokio::main]
async fn main() {
    println!("Running RPC Server");
    let rpc_server = transport::rpc::RpcServer::new();
    let task = rpc_server.serve().await.unwrap();

    // Wait for CTRL+C signal to gracefully shut down the task
    let mut sigint = signal::unix::signal(signal::unix::SignalKind::interrupt()).unwrap();
    sigint.recv().await;
    rpc_server.shutdown().await;

    task.await.unwrap();
}
