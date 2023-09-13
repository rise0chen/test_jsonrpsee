use std::net::SocketAddr;

use jsonrpsee::core::client::ClientT;
use jsonrpsee::server::Server;
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::{rpc_params, RpcModule};
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()?;

    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(filter)
        .finish()
        .try_init()?;

    let addr = run_server().await?;
    let url = format!("ws://{}", addr);

    let client = WsClientBuilder::default().build(&url).await?;
    loop {
        let memory_info = psutil::process::Process::current()?.memory_info()?;
        tracing::info!("memory_info: {:?}", memory_info);
        for _ in 0..3000 {
            let _: String = client.request("say_hello", rpc_params![]).await?;
        }
    }
}

async fn run_server() -> anyhow::Result<SocketAddr> {
    let server = Server::builder().build("127.0.0.1:0").await?;
    let mut module = RpcModule::new(());
    module.register_method("say_hello", |_, _| "lo")?;
    let addr = server.local_addr()?;
    let handle = server.start(module);

    // In this example we don't care about doing shutdown so let's it run forever.
    // You may use the `ServerHandle` to shut it down or manage it yourself.
    tokio::spawn(handle.stopped());

    Ok(addr)
}
