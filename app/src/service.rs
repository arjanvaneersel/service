pub use tokio::signal;

#[tokio::main]
async fn main() {
    let (task, server_addr) = transport::rpc::RpcServer::serve().await.unwrap();
    let url = format!("ws://{}", server_addr);
    println!("Running on {}", url);

    // Wait for CTRL+C signal to gracefully shut down the task
    let mut sigint = signal::unix::signal(signal::unix::SignalKind::interrupt()).unwrap();
    sigint.recv().await;

    // TODO: Replace with a graceful shutdown mechanism.
    task.abort();

    task.await.unwrap();
}
