use super::Cmd;
use crate::ops::{do_app, do_catalogs};
use clap::Arg;
use pipebased_common::{grpc::daemon::daemon_client::DaemonClient, Result};
use tonic::transport::Channel;

pub fn pull_catalogs() -> Cmd {
    Cmd::new("pullc").about("pull catalogs").args(vec![
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

pub fn pull_app() -> Cmd {
    Cmd::new("pulla").about("pull app").args(vec![
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

pub async fn exec_pull_catalogs(
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
    let _ = do_catalogs::pull_catalogs(&mut client, namespace.to_owned(), id.to_owned(), version)
        .await?;
    Ok(())
}

pub async fn exec_pull_app(
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
    let _ = do_app::pull_app(&mut client, namespace.to_owned(), id.to_owned(), version).await?;
    Ok(())
}
