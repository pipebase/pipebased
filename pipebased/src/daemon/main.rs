mod bootstrap;
mod config;
mod daemon;

use bootstrap::bootstrap;
use config::Config;
use pipebased_common::{
    grpc::daemon::daemon_server::DaemonServer, init_tracing_subscriber, read_yml, Result,
};
use std::net::SocketAddr;
use tonic::transport::Server;
use tracing::{info, instrument};

const ENV_PIPEBASED_CONFIG_FILE: &str = "PIPEBASED_CONFIG_FILE";

#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
    init_tracing_subscriber();
    info!("read configuration ...");
    let config: Config = read_yml(std::env::var(ENV_PIPEBASED_CONFIG_FILE)?)?;
    let daemon_svc = bootstrap(config.daemon);
    let addr: SocketAddr = config.address.parse()?;
    info!("run daemon server ...");
    Server::builder()
        .add_service(DaemonServer::new(daemon_svc))
        .serve(addr)
        .await?;
    info!("daemon server exit ...");
    Ok(())
}
