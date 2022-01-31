use super::Cmd;
use crate::ops::do_pipe;
use clap::Arg;
use pipebased_common::{grpc::daemon::daemon_client::DaemonClient, Result};
use tonic::transport::Channel;

pub fn create_pipe() -> Cmd {
    Cmd::new("create").about("create pipe").arg(
        Arg::new("file")
            .help("Specify path to create pipe request")
            .required(true)
            .index(1),
    )
}

pub async fn exec_create_pipe(
    mut client: DaemonClient<Channel>,
    args: &clap::ArgMatches,
) -> Result<()> {
    let file = args.value_of("file").unwrap_or("pipe-compose.yml");
    let _ = do_pipe::create_pipe(&mut client, file).await?;
    Ok(())
}
