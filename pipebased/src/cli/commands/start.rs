use super::Cmd;
use crate::ops::do_pipe;
use clap::Arg;
use pipebased_common::{grpc::daemon::daemon_client::DaemonClient, Result};
use tonic::transport::Channel;

pub fn start_pipe() -> Cmd {
    Cmd::new("start").about("start pipe").arg(
        Arg::new("id")
            .help("Specify pipe id")
            .required(true)
            .index(1),
    )
}

pub async fn exec_start_pipe(
    mut client: DaemonClient<Channel>,
    args: &clap::ArgMatches,
) -> Result<()> {
    let id = args.value_of("id").unwrap();
    let _ = do_pipe::start_pipe(&mut client, id.to_owned()).await?;
    Ok(())
}
