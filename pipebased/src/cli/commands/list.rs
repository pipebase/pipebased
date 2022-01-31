use super::Cmd;
use crate::ops::{do_app, do_catalogs, do_pipe, print::PrintRecords};
use pipebased_common::{grpc::daemon::daemon_client::DaemonClient, Result};
use tonic::transport::Channel;

pub fn list_pipe() -> Cmd {
    Cmd::new("ps").about("List pipe instance")
}

pub fn list_app() -> Cmd {
    Cmd::new("apps").about("List app binary")
}

pub fn list_catalogs() -> Cmd {
    Cmd::new("catalogs").about("List catalogs manifest")
}

pub async fn exec_list_pipe(mut client: DaemonClient<Channel>) -> Result<()> {
    let response = do_pipe::list_pipe(&mut client).await?;
    response.print_records();
    Ok(())
}

pub async fn exec_list_app(mut client: DaemonClient<Channel>) -> Result<()> {
    let response = do_app::list_app(&mut client).await?;
    response.print_records();
    Ok(())
}

pub async fn exec_list_catalogs(mut client: DaemonClient<Channel>) -> Result<()> {
    let response = do_catalogs::list_catalogs(&mut client).await?;
    response.print_records();
    Ok(())
}
