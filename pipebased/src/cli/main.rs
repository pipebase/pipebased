mod commands;
mod config;
mod ops;

use config::Config;
use ops::print::Printer;
use pipebased_common::{
    grpc::{
        client::{DaemonClientBuilder, RpcClientConfig},
        daemon::daemon_client::DaemonClient,
    },
    Result,
};
use std::process;
use tonic::transport::Channel;
use tracing::instrument;

#[tokio::main]
#[instrument]
async fn main() {
    let result = run().await;
    process::exit(match result {
        Ok(_) => 0,
        Err(_) => 1,
    })
}

async fn run() -> Result<()> {
    let mut printer = Printer::new();
    let matches = clap::App::new("pipe")
        .args(vec![clap::Arg::new("config")
            .short('c')
            .takes_value(true)
            .help("path to config file, default ~/.pd/config")])
        .subcommands(commands::cmds())
        .get_matches();
    let config_path = matches.value_of("config");
    let config = Config::parse_or_default(config_path);
    let client_config = config.rpc;
    let client = build_client(client_config).await?;
    let (cmd, matches) = matches.subcommand().unwrap();
    match commands::exec(cmd, client, matches).await {
        Ok(_) => Ok(()),
        Err(err) => {
            let _ = printer.error(&err);
            Err(err)
        }
    }
}

async fn build_client(config: RpcClientConfig) -> Result<DaemonClient<Channel>> {
    let protocol = config.protocol;
    let address = config.address;
    DaemonClientBuilder::default()
        .protocol(protocol)
        .address(address.as_str())
        .build()
        .await
}
