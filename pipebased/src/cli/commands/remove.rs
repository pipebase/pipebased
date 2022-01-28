use super::Cmd;
use crate::ops::{do_app, do_catalogs, do_pipe};
use clap::Arg;
use pipebased_common::{grpc::daemon::daemon_client::DaemonClient, Result};
use tonic::transport::Channel;

pub fn remove_pipe() -> Cmd {
    Cmd::new("rm").about("remove pipe").arg(
        Arg::new("id")
            .help("Specify pipe id")
            .required(true)
            .index(1),
    )
}

pub fn remove_catalogs() -> Cmd {
    Cmd::new("rmc").about("remove catalogs").args(vec![
        Arg::new("namespace")
            .short('n')
            .help("Specify namespace")
            .required(true)
            .takes_value(true),
        Arg::new("id")
            .short('i')
            .help("Specify project id")
            .required(true)
            .takes_value(true),
        Arg::new("version")
            .short('v')
            .help("Specify catalogs version")
            .required(true)
            .takes_value(true),
    ])
}

pub fn remove_app() -> Cmd {
    Cmd::new("rma").about("remove app").args(vec![
        Arg::new("namespace")
            .short('n')
            .help("Specify namespace")
            .required(true)
            .takes_value(true),
        Arg::new("id")
            .short('i')
            .help("Specify project id")
            .required(true)
            .takes_value(true),
        Arg::new("version")
            .short('v')
            .help("Specify app version")
            .required(true)
            .takes_value(true),
    ])
}

pub async fn exec_remove_pipe(
    mut client: DaemonClient<Channel>,
    args: &clap::ArgMatches,
) -> Result<()> {
    let id = args.value_of("id").unwrap();
    let _ = do_pipe::remove_pipe(&mut client, id.to_owned()).await?;
    Ok(())
}

pub async fn exec_remove_catalogs(
    mut client: DaemonClient<Channel>,
    args: &clap::ArgMatches,
) -> Result<()> {
    let namespace = args.value_of("namespace").unwrap();
    let id = args.value_of("id").unwrap();
    let version: u64 = args
        .value_of("version")
        .unwrap()
        .parse()
        .expect("invalid catalogs version");
    let _ = do_catalogs::remove_catalogs(&mut client, namespace.to_owned(), id.to_owned(), version)
        .await?;
    Ok(())
}

pub async fn exec_remove_app(
    mut client: DaemonClient<Channel>,
    args: &clap::ArgMatches,
) -> Result<()> {
    let namespace = args.value_of("namespace").unwrap();
    let id = args.value_of("id").unwrap();
    let version: u64 = args
        .value_of("version")
        .unwrap()
        .parse()
        .expect("invalid app version");
    let _ = do_app::remove_app(&mut client, namespace.to_owned(), id.to_owned(), version).await?;
    Ok(())
}
