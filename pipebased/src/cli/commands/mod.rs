pub mod create;
pub mod list;
pub mod pull;
pub mod remove;
pub mod start;
pub mod stop;

use pipebased_common::{grpc::daemon::daemon_client::DaemonClient, Result};
use tonic::transport::Channel;

pub type Cmd = clap::App<'static>;

pub fn cmds() -> Vec<Cmd> {
    vec![
        create::create_pipe(),
        list::list_pipe(),
        list::list_app(),
        list::list_catalogs(),
        pull::pull_app(),
        pull::pull_catalogs(),
        remove::remove_pipe(),
        remove::remove_app(),
        remove::remove_catalogs(),
        start::start_pipe(),
        stop::stop_pipe(),
    ]
}

pub async fn exec(cmd: &str, client: DaemonClient<Channel>, args: &clap::ArgMatches) -> Result<()> {
    match cmd {
        "create" => create::exec_create_pipe(client, args).await,
        "ps" => list::exec_list_pipe(client).await,
        "apps" => list::exec_list_app(client).await,
        "catalogs" => list::exec_list_catalogs(client).await,
        "pulla" => pull::exec_pull_app(client, args).await,
        "pullc" => pull::exec_pull_catalogs(client, args).await,
        "rm" => remove::exec_remove_pipe(client, args).await,
        "rma" => remove::exec_remove_app(client, args).await,
        "rmc" => remove::exec_remove_catalogs(client, args).await,
        "start" => start::exec_start_pipe(client, args).await,
        "stop" => stop::exec_stop_pipe(client, args).await,
        _ => unreachable!("unknown cmd {}", cmd),
    }
}
